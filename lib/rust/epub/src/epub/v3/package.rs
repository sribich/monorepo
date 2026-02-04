use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::read_to_string;

use quick_xml::de::from_str;
use railgun::error::ResultExt;
use serde::Deserialize;
use zip::ZipArchive;
use zip::read::ZipFile;

use crate::Error;
use crate::epub::FromParameterizedZip;
use crate::epub::FromZip;
use crate::error::IoErrorContext;
use crate::error::Result;
use crate::error::XmlErrorContext;

/// Specifies the base direction [bidi] of the textual content and attribute
/// values of the carrying element and its descendants.
///
/// Allowed values are:
///     ltr — left-to-right base direction;
///     rtl — right-to-left base direction; and
///     auto — base direction is determined using the Unicode Bidi Algorithm
/// [bidi].
///
/// Reading systems will assume the value auto when EPUB creators omit the
/// attribute or use an invalid value.
#[derive(Debug, Deserialize)]
pub enum BaseDirection {
    Ltr,
    Rtl,
    Auto,
}

impl Default for BaseDirection {
    fn default() -> Self {
        Self::Auto
    }
}

/// The package element encapsulates all the information expressed in
/// the package document.
///
/// <https://www.w3.org/TR/epub-33/#sec-package-elem>
#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(rename = "@dir", default = "BaseDirection::default")]
    pub dir: BaseDirection,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "@xml:lang")]
    pub xml_lang: Option<String>,
    #[serde(rename = "@unique-identifier")]
    pub unique_identifier: String,
    #[serde(rename = "@version")]
    pub version: String,

    #[serde(rename = "metadata")]
    pub metadata: Metadata,
    #[serde(rename = "manifest")]
    pub manifest: Manifest,
    #[serde(rename = "spine")]
    pub spine: Spine,
    #[serde(rename = "guide")]
    pub guide: Option<Guide>,
    #[serde(rename = "bindings")]
    pub bindings: Option<Bindings>,
    #[serde(rename = "collection")]
    pub collection: Option<Collection>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    /// While this field is `REQUIRED` in the spec some ebooks seem to omit
    /// this field.
    #[serde(default)]
    pub identifier: Vec<Identifier>,
    pub title: Vec<Title>,
}

/// The dc:identifier element [dcterms] contains an identifier such as
/// a UUID, DOI or ISBN.
///
/// <https://www.w3.org/TR/epub-33/#sec-opf-dcidentifier>
#[derive(Debug, Deserialize)]
#[serde(rename = "dc:identifier")]
pub struct Identifier {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

/// The dc:title element [dcterms] represents an instance of a name for
/// the EPUB publication.
///
/// <https://www.w3.org/TR/epub-33/#sec-opf-dctitle>
#[derive(Debug, Deserialize)]
#[serde(rename = "dc:title")]
pub struct Title {
    #[serde(rename = "@dir", default = "BaseDirection::default")]
    pub dir: BaseDirection,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@xml:lang")]
    pub xml_lang: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

/// The dc:language element [dcterms] specifies the language of the
/// content of the EPUB publication.
///
/// <https://www.w3.org/TR/epub-33/#sec-opf-dclanguage>
#[derive(Debug, Deserialize)]
#[serde(rename = "dc:language")]
pub struct Language {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "@href")]
    pub href: String,

    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@media-type")]
    pub media_type: String,

    #[serde(rename = "@properties")]
    pub properties: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Spine {
    #[serde(rename = "itemref")]
    pub itemrefs: Vec<ItemRef>,
}

#[derive(Debug, Deserialize)]
pub struct ItemRef {
    #[serde(rename = "@idref")]
    pub idref: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Guide {
    #[serde(rename = "reference")]
    pub references: Vec<Reference>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reference {
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    /// This should be set, but in the case of covers it might not be set.
    #[serde(rename = "@title")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Bindings;

#[derive(Debug, Deserialize)]
pub struct Collection;

impl<'a> FromParameterizedZip<'a> for Package {
    type Params = &'a str;
    type Type = Package;

    fn parse(zip: &'a mut ZipArchive<impl Read + Seek>, path: Self::Params) -> Result<Self::Type> {
        let file = zip.by_name(path).unwrap();

        let data = read_to_string(file).context(IoErrorContext {}).unwrap();
        let container: Self = from_str(&data).unwrap();

        Ok(container)
    }
}
