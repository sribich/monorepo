use std::io::Read;
use std::io::Seek;
use std::io::read_to_string;

use quick_xml::de::from_str;
use railgun::error::OptionExt;
use railgun::error::ResultExt;
use serde::Deserialize;
use serde::de::Visitor;
use serde_util::StringOf;
use zip::ZipArchive;

use super::FromParameterizedZip;
use super::FromZip;
use super::GeneralPackage;
use super::v3;
use crate::archive::EpubArchive;
use crate::epub::v2;
use crate::error::NoPackagesContext;
use crate::error::Result;
use crate::error::ValidationContext;
use crate::error::parse_error::IoErrorContext;
use crate::error::parse_error::MissingRequiredFileContext;

pub const EPUB_CONTAINER_NAMESPACE: &str = "urn:oasis:names:tc:opendocument:xmlns:container";
pub const EPUB_PACKAGE_MIME_TYPE: &str = "application/oebps-package+xml";

/// The [container element][container] encapsulates all the information in the
/// container.xml file.
///
/// [container]: https://www.w3.org/TR/epub-34/#sec-container.xml-container-elem
#[derive(Debug, Deserialize)]
pub struct Container {
    #[serde(rename = "@version")]
    pub version: StringOf<"1.0">,
    #[serde(rename = "@xmlns")]
    pub xmlns: StringOf<EPUB_CONTAINER_NAMESPACE>,
    #[serde(rename = "rootfiles")]
    pub rootfiles: RootFiles,
    #[serde(rename = "links")]
    pub links: Option<Links>,
}

/// The [rootfiles element][rootfiles] contains a list of package documents
/// available in the EPUB container.
///
/// [rootfiles]: https://www.w3.org/TR/epub-34/#sec-container.xml-rootfiles-elem
#[derive(Debug, Deserialize)]
pub struct RootFiles {
    pub rootfile: Vec<RootFile>,
}

/// Each [rootfile element][rootfile] identifies the location of one package
/// document in the EPUB container.
///
/// [rootfile]: https://www.w3.org/TR/epub-34/#sec-container.xml-rootfiles-elem
#[derive(Clone, Debug, Deserialize)]
pub struct RootFile {
    /// Identifies the location of a package document.
    ///
    /// The value of the attribute MUST be a path-relative-scheme-less-URL
    /// string. The path is relative to the root directory.
    #[serde(rename = "@full-path")]
    pub full_path: String,
    /// Identifies the media type of the package document.
    ///
    /// The value of the attribute MUST be `application/oebps-package+xml`.
    ///
    /// # Note
    ///
    /// - In EPUB 2.0 this did not have a mandated value, and could instead be
    ///   any form of alternate rendering. It did require the first entry to
    ///   be `application/oebps-package+xml`. We have chosen to ignore these
    ///   alternate renderings.
    #[serde(rename = "@media-type")]
    pub media_type: String,
}

/// The [links element][links] identifies resources necessary for the processing
/// of the OCF ZIP container.
///
/// # Note
///
/// - Links were not a part of the EPUB 2.0 specification.
///
/// [links]: https://www.w3.org/TR/epub-33/#sec-container.xml-links-elem
#[derive(Debug, Deserialize)]
pub struct Links {
    pub links: Vec<Link>,
}

/// The [link element][link] identifies an individual resource necessary
/// for the processing of the OCF ZIP container.
///
/// [link]: https://www.w3.org/TR/epub-33/#sec-container.xml-link-elem
#[derive(Debug, Deserialize)]
pub struct Link {
    /// Identifies the location of a resource.
    ///
    /// The value of the link element href attribute MUST be a path relative
    /// scheme-less URL string. The path is relative to the root directory.
    #[serde(rename = "@href")]
    pub href: String,
    /// Identifies the type and format of the referenced resource.
    ///
    /// The value of the attribute MUST be a media type [rfc2046].
    ///
    /// [rfc2046]: https://www.rfc-editor.org/rfc/rfc2046.html
    #[serde(rename = "@media-type")]
    pub media_type: Option<String>,
    /// Identifies the relationship of the resource.
    ///
    /// The value of the attribute MUST be a space-separated list of tokens.
    #[serde(rename = "@rel")]
    pub rel: String,
}

impl Container {
    pub fn package<R>(&self, zip: &mut ZipArchive<R>) -> Result<GeneralPackage>
    where
        R: Read + Seek,
    {
        let package = self
            .rootfiles
            .package_iter()
            .next()
            .context(NoPackagesContext {})?;

        let PackageWithVersion { version } =
            EpubArchive::parse_with::<PackageWithVersion>(zip, &package.full_path)?;

        Ok(match version {
            PackageVersion::V2 => GeneralPackage::V2(EpubArchive::parse_with::<v2::Package>(
                zip,
                &package.full_path,
            )?),
            PackageVersion::V3 => GeneralPackage::V3(EpubArchive::parse_with::<v3::Package>(
                zip,
                &package.full_path,
            )?),
        })
    }

    pub fn validate(&self) -> Result<()> {
        self.rootfiles.validate()?;

        Ok(())
    }
}

impl<'a> FromZip<'a> for Container {
    type Type = Container;

    fn read(zip: &'a mut ZipArchive<impl Read + Seek>) -> Result<String> {
        let path = "META-INF/container.xml";
        let file = zip
            .by_name(path)
            .context(MissingRequiredFileContext { path })?;

        Ok(read_to_string(file).context(IoErrorContext {})?)
    }

    fn parse<S: AsRef<str>>(data: S) -> Result<Self::Type> {
        let container: Self = from_str(data.as_ref())?;

        container.validate()?;

        Ok(container)
    }
}

impl RootFiles {
    /// Returns an iterator over EPUB package documents of type
    /// [`EPUB_PACKAGE_MIME_TYPE`], excluding any alternate renderings.
    ///
    /// Version 2 allows alternate renderings of a publication to
    /// be of any `media_type` while version 3 limits alternate
    /// renderings to [`EPUB_PACKAGE_MIME_TYPE`].
    pub fn package_iter(&self) -> impl Iterator<Item = &'_ RootFile> {
        self.rootfile
            .iter()
            .filter(|rootfile| rootfile.media_type == EPUB_PACKAGE_MIME_TYPE)
    }
}

#[derive(Debug, Deserialize)]
pub struct PackageWithVersion {
    #[serde(rename = "@version")]
    pub version: PackageVersion,
}

impl<'a> FromParameterizedZip<'a> for PackageWithVersion {
    type Params = &'a str;
    type Type = PackageWithVersion;

    fn parse(zip: &'a mut ZipArchive<impl Read + Seek>, path: Self::Params) -> Result<Self::Type> {
        let file = zip.by_name(path).unwrap();

        let data = read_to_string(file).context(IoErrorContext {}).unwrap();
        let container: Self = from_str(&data).unwrap();

        Ok(container)
    }
}

#[derive(Debug)]
pub enum PackageVersion {
    V2,
    V3,
}

impl<'de> Deserialize<'de> for PackageVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PackageVersionVisitor;

        impl<'de> Visitor<'de> for PackageVersionVisitor {
            type Value = PackageVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`2.0`, or `3.0`")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "2.0" => Ok(PackageVersion::V2),
                    "3.0" => Ok(PackageVersion::V3),
                    _ => Err(E::custom(format!(
                        "Deserialized value '{v}' does not match the expected value `2.0`, or `3.0`"
                    ))),
                }
            }
        }

        deserializer.deserialize_identifier(PackageVersionVisitor)
    }
}

impl RootFiles {
    fn validate(&self) -> Result<()> {
        if self.rootfile.len() != 1 {
            return ValidationContext {
                invariant: "epub must only contain 1 rootfile",
            }
            .fail();
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::epub::Container;
    use crate::epub::FromZip;

    #[test]
    fn unknown_container_elements_are_ignored() {
        let xml = indoc! {r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0">
                <foo:bar />
                <rootfiles>
                    <rootfile full-path="item/package.opf" media-type="application/oebps-package+xml"/>
                </rootfiles>
            </container>
        "#};

        let container = Container::parse(xml).unwrap();

        assert_eq!(container.rootfiles.rootfile.len(), 1);
    }
}
