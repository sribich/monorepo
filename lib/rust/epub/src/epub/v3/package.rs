use std::collections::HashMap;
use std::io::read_to_string;

use quick_xml::de::from_str;
use zip::ZipArchive;

use crate::Error;
use crate::archive::EpubArchive;
use crate::archive::EpubFile;
use crate::epub::v3::nav::Nav;

#[derive(Clone)]
pub struct Package {
    spec: spec::Package,
}

impl Package {
    pub fn title(&self) -> &str {
        self.spec.metadata.title.first().map_or("", |it| &it.value)
    }

    pub fn parse(zip: &mut ZipArchive<EpubFile>, path: &str) -> Result<Self, Error> {
        let file = zip.by_name(path).unwrap();

        let data = read_to_string(file)?;
        let spec: spec::Package = from_str(&data).unwrap();

        Ok(Self { spec })
    }

    pub fn manifest(
        &self,
        epub: &mut EpubArchive,
    ) -> Result<HashMap<String, (String, String)>, Error> {
        self.spec
            .manifest
            .items
            .iter()
            .filter(|item| {
                item.media_type == "application/xhtml+xml" || item.href.ends_with("html")
            })
            .cloned()
            .map(|item| (item.id, item.href))
            .map(|(id, path)| {
                let content = epub.read(&path)?;

                Ok((id, (path, content)))
            })
            .collect::<Result<HashMap<_, _>, Error>>()
    }

    pub fn navigation(&self, epub: &mut EpubArchive) -> Result<Vec<(String, String)>, Error> {
        let nav = self
            .spec
            .manifest
            .items
            .iter()
            .find(|item| matches!(&item.properties, Some(properties) if properties == "nav"))
            .unwrap();

        Ok(Nav::parse(epub, &nav.href)?.to_entries())
    }

    pub fn spine(&self) -> impl Iterator<Item = &String> {
        self.spec.spine.itemrefs.iter().map(|it| &it.idref)
    }
}

#[expect(dead_code, reason = "adhering to spec")]
mod spec {
    use serde::Deserialize;

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
    #[derive(Clone, Debug, Deserialize)]
    #[derive(Default)]
    pub enum BaseDirection {
        Ltr,
        Rtl,
        #[default]
        Auto,
    }

    

    /// The package element encapsulates all the information expressed in
    /// the package document.
    ///
    /// <https://www.w3.org/TR/epub-33/#sec-package-elem>
    #[derive(Clone, Debug, Deserialize)]
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

    #[derive(Clone, Debug, Deserialize)]
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
    #[derive(Clone, Debug, Deserialize)]
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
    #[derive(Clone, Debug, Deserialize)]
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
    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename = "dc:language")]
    pub struct Language {
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "$value")]
        pub value: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Manifest {
        #[serde(rename = "item")]
        pub items: Vec<Item>,
    }

    #[derive(Clone, Debug, Deserialize)]
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

    #[derive(Clone, Debug, Deserialize)]
    pub struct Spine {
        #[serde(rename = "itemref")]
        pub itemrefs: Vec<ItemRef>,
    }

    #[derive(Clone, Debug, Deserialize)]
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

    #[derive(Clone, Debug, Deserialize)]
    pub struct Bindings;

    #[derive(Clone, Debug, Deserialize)]
    pub struct Collection;
}
