use std::io::read_to_string;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use quick_xml::de::from_str;
use railgun::error::ResultExt;
use zip::ZipArchive;

use crate::Error;
use crate::archive::EpubFile;
use crate::error::parse_error::IoErrorContext;
use crate::error::parse_error::MissingRequiredFileContext;

pub struct Container {
    basedir: PathBuf,
    spec: spec::Container,
}

impl Container {
    pub fn parse(zip: &mut ZipArchive<EpubFile>) -> Result<Container, Error> {
        let path = "META-INF/container.xml";
        let file = zip
            .by_name(path)
            .context(MissingRequiredFileContext { path })?;

        let content = read_to_string(file).context(IoErrorContext {})?;

        Container::from_str(&content)
    }

    pub fn basedir(&self) -> &Path {
        &self.basedir
    }

    pub fn package_path(&self) -> &str {
        &self.spec.get_first_package().unwrap().full_path
    }
}

impl FromStr for Container {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spec: spec::Container = from_str(s)?;

        let basedir = PathBuf::from(spec.get_first_package().unwrap().full_path.clone())
            .parent()
            .unwrap()
            .to_owned();

        Ok(Self { basedir, spec })
    }
}

#[expect(dead_code, reason = "adhering to spec")]
mod spec {
    use serde::Deserialize;
    use serde_util::StringOf;

    pub const EPUB_CONTAINER_NAMESPACE: &str = "urn:oasis:names:tc:opendocument:xmlns:container";
    pub const EPUB_PACKAGE_MIME_TYPE: &str = "application/oebps-package+xml";

    /// The [container element][container] encapsulates all the information in the
    /// container.xml file.
    ///
    /// [container]: https://www.w3.org/TR/epub-34/#sec-container.xml-container-elem
    #[derive(Deserialize)]
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

    impl Container {
        /// Returns the first valid EPUB package document.
        pub fn get_first_package(&self) -> Option<&RootFile> {
            self.rootfiles.package_iter().next()
        }
    }

    /// The [rootfiles element][rootfiles] contains a list of package documents
    /// available in the EPUB container.
    ///
    /// [rootfiles]: https://www.w3.org/TR/epub-34/#sec-container.xml-rootfiles-elem
    #[derive(Deserialize)]
    pub struct RootFiles {
        pub rootfile: Vec<RootFile>,
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

    /// Each [rootfile element][rootfile] identifies the location of one package
    /// document in the EPUB container.
    ///
    /// [rootfile]: https://www.w3.org/TR/epub-34/#sec-container.xml-rootfiles-elem
    #[derive(Clone, Deserialize)]
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
    #[derive(Deserialize)]
    pub struct Links {
        pub links: Vec<Link>,
    }

    /// The [link element][link] identifies an individual resource necessary
    /// for the processing of the OCF ZIP container.
    ///
    /// [link]: https://www.w3.org/TR/epub-33/#sec-container.xml-link-elem
    #[derive(Deserialize)]
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
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use indoc::indoc;

    use crate::epub::Container;

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

        let container = Container::from_str(xml).unwrap();

        assert_eq!(container.spec.rootfiles.rootfile.len(), 1);
    }
}
