use std::collections::HashMap;
use std::fmt::Display;

use convert_case::Case;
use convert_case::Casing;
use serde::Deserialize;
use serde_json::Value;
use serde_json::value::RawValue;

#[derive(Debug)]
pub enum YomitanFile {
    Index(Index),
    KanjiTerm(Vec<KanjiTerm>),
    KanjiMeta(Vec<KanjiMeta>),
    Tag(Vec<Tag>),
    Term(Vec<Term>),
    TermMeta(Vec<TermMeta>),
}

#[derive(Debug, Deserialize)]
pub struct Index {
    /// The title of the dictionary.
    pub title: String,
    /// Revision of the dictionary. This value is displayed, and used to check
    /// for dictionary updates.
    pub revision: String,
    /// Whether or not this dictionary contains sequencing information for
    /// related terms.
    #[serde(default = "Default::default")]
    pub sequenced: bool,
    /// Format of data found in the JSON data files.
    #[serde(alias = "version")]
    pub format: Option<u32>,
    /// Creator of the dictionary.
    pub author: Option<String>,
    /// Whether this dictionary contains links to its latest version.
    #[serde(rename = "isUpdatable", default = "Default::default")]
    pub is_updatable: bool,
    /// URL for the index file of the latest revision of the dictionary, used to
    /// check for updates.
    #[serde(rename = "indexUrl")]
    pub index_url: Option<String>,
    /// URL for the download of the latest revision of the dictionary.
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    /// URL for the source of the dictionary, displayed in the dictionary
    /// details.
    pub url: Option<String>,
    /// Description of the dictionary data.
    pub description: Option<String>,
    /// Attribution information for the dictionary data.
    pub attribution: Option<String>,
    /// Language of the terms in the dictionary.
    /// Must match regular expression: ^[a-z]{2,3}$
    #[serde(rename = "sourceLanguage")]
    pub source_language: Option<String>,
    /// Main language of the definitions in the dictionary.
    #[serde(rename = "targetLanguage")]
    pub target_language: Option<String>,
    /// ...
    #[serde(rename = "frequencyMode")]
    pub frequency_mode: Option<String>,
    /// Tag information for terms and kanji. This object is obsolete and
    /// individual tag files should be used instead.
    #[serde(rename = "tagMeta")]
    pub tag_meta: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(expecting = "expecting [<kanji>, <onyomi>, <kunyomi>, <tags>, <meanings>, <meta>]")]
pub struct KanjiTerm {
    /// Kanji character.
    kanji: String,
    /// String of space-separated onyomi readings for the kanji character. An
    /// empty string is treated as no readings.
    onyomi: String,
    /// String of space-separated kunyomi readings for the kanji character. An
    /// empty string is treated as no readings.
    kunyomi: String,
    /// String of space-separated tags for the kanji character. An empty string
    /// is treated as no tags.
    tags: String,
    /// Array of meanings for the kanji character.
    meanings: Vec<String>,
    /// Various stats for the kanji character.
    meta: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(expecting = "expecting [<kanji>, <kind>, <data>]")]
pub struct KanjiMeta {
    /// Kanji character.
    kanji: String,
    /// Type of data. "freq" corresponds to frequency information.
    kind: String,
    /// Data for the character.
    data: KanjiMetaData,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KanjiMetaData {
    Number(u32),
    String(String),
    Value {
        value: u32,
        #[serde(rename = "displayValue")]
        display: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(expecting = "expecting [<name>, <category>, <sort_order>, <notes>, <score>]")]
pub struct Tag {
    /// Tag name.
    name: String,
    /// Category for the tag.
    category: String,
    /// Sorting order for the tag.
    sort_order: i32,
    /// Notes for the tag.
    notes: String,
    /// Score used to determine popularity. Negative values are more rare and
    /// positive values are more frequent. This score is also used to sort
    /// search results.
    score: i32,
}

#[derive(Debug, Deserialize)]
#[serde(
    expecting = "expecting [<term>, <reading>, <definition_tags>, <inflections>, <score>, <defintions>, <sequence>, <tags>]"
)]
pub struct Term {
    /// The text for the term.
    pub term: String,
    /// Reading of the term, or an empty string if the reading is the same as
    /// the term.
    pub reading: String,
    /// String of space-separated tags for the definition. An empty string is
    /// treated as no tags.
    pub definition_tags: String,
    /// String of space-separated rule identifiers for the definition which is
    /// used to validate deinflection. An empty string should be used for words
    /// which aren't inflected.
    pub inflections: String,
    /// Score used to determine popularity. Negative values are more rare and
    /// positive values are more frequent. This score is also used to sort
    /// search results.
    pub score: i64,
    /// Array of definitions for the term.
    pub definitions: Vec<Definition>,
    /// Sequence number for the term. Terms with the same sequence number can be
    /// shown together when the "resultOutputMode" option is set to "merge".
    pub sequence: i64,
    /// String of space-separated tags for the term. An empty string is treated
    /// as no tags.
    pub tags: String,
}

#[derive(Clone, Debug)]
pub enum TermMeta {
    Frequency {
        word: String,
        reading: String,
        frequency: u32,
        display: Option<String>,
    },
    PitchAccent {
        word: String,
        reading: String,
        pitches: Vec<Pitch>,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pitch {
    pub position: u32,
}

impl<'de> serde::de::Deserialize<'de> for TermMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw: Box<RawValue> = Deserialize::deserialize(deserializer).unwrap();
        let value: Value = serde_json::from_str(raw.get()).unwrap();

        if value.is_array() {
            let array = value.as_array().unwrap();

            let word = array[0].as_str().unwrap().to_owned();
            let kind = array[1].as_str().unwrap(); // freq, pitch, ipa
            let data = array[2].clone();

            match kind {
                "freq" => {
                    match 1 {
                        1 if data.is_number() => {
                            return Ok(TermMeta::Frequency {
                                word: word.clone(),
                                reading: word,
                                frequency: data.as_u64().unwrap() as u32,
                                display: None,
                            });
                        }
                        1 if data.is_string() => {
                            return Ok(TermMeta::Frequency {
                                word: word.clone(),
                                reading: word,
                                frequency: data.as_u64().unwrap() as u32,
                                display: None,
                            });
                        }
                        1 if data.is_object() => {
                            let data = data.as_object().unwrap();

                            let reading = data
                                .get("reading")
                                .map(|it| it.as_str().unwrap().to_owned())
                                .unwrap_or_else(|| word.clone());

                            let display =
                                data.get("displayValue")
                                    .map(|it| Some(it.as_str().map(std::borrow::ToOwned::to_owned)))
                                    .unwrap_or_else(|| {
                                        let frequency = data.get("frequency").unwrap();

                                        if frequency.is_number() {
                                            return None;
                                        }

                                        frequency.as_object().unwrap().get("displayValue").map(
                                            |it| it.as_str().map(std::string::ToString::to_string),
                                        )
                                    })
                                    .unwrap_or(None);

                            let frequency = data
                                .get("value")
                                .map(|it| it.as_u64().unwrap() as u32)
                                .unwrap_or_else(|| {
                                    let frequency = data.get("frequency").unwrap();

                                    if frequency.is_number() {
                                        return frequency.as_u64().unwrap() as u32;
                                    }

                                    frequency
                                        .as_object()
                                        .unwrap()
                                        .get("value")
                                        .unwrap()
                                        .as_u64()
                                        .unwrap() as u32
                                });

                            return Ok(TermMeta::Frequency {
                                word,
                                reading,
                                frequency,
                                display,
                            });
                        }
                        _ => {
                            panic!("Unexpected freq kind {data:#?}");
                        }
                    }
                }
                "pitch" => {
                    let inner = array[2].as_object().unwrap();

                    return Ok(TermMeta::PitchAccent {
                        word: array[0].as_str().unwrap().to_owned(),
                        reading: inner.get("reading").unwrap().as_str().unwrap().to_owned(),
                        pitches: inner
                            .get("pitches")
                            .unwrap()
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|it| Pitch {
                                position: it
                                    .as_object()
                                    .unwrap()
                                    .get("position")
                                    .unwrap()
                                    .as_u64()
                                    .unwrap() as u32,
                            })
                            .collect(),
                    });
                }
                _ => panic!("Unknown kind {}", array[1]),
            }
        }

        panic!("Expected array");
    }
}

#[derive(Debug)]
pub enum Definition {
    /// Single definition for the term.
    Single(String),
    Text(DefinitionText),
    Structured(DefinitionStructured),
}

impl Definition {
    pub fn to_string(&self, img_prefix: &Option<String>) -> String {
        match self {
            Definition::Single(definition) => definition.clone(),
            Definition::Text(definition_text) => definition_text.text.clone(),
            Definition::Structured(definition_structured) => {
                let mut data = String::new();

                extract_content(
                    &definition_structured.content,
                    &mut data,
                    img_prefix.as_deref().unwrap_or(""),
                );

                data
            }
        }
    }
}

fn extract_content(content: &StructuredContent, full_text: &mut String, img_prefix: &str) {
    match content {
        StructuredContent::Text(text) => {
            full_text.push_str(text);
        }
        StructuredContent::Children(children) => {
            for child in children {
                extract_content(child, full_text, img_prefix);
            }
        }
        StructuredContent::Table(table) => match &*table.tag {
            "td" => {
                if let Some(content) = &table.content {
                    full_text.push_str("<td>");
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str("</td>");
                }
            }
            "th" => {
                if let Some(content) = &table.content {
                    full_text.push_str("<th>");
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str("</th>");
                }
            }
            _ => unreachable!("not covered {}", table.tag),
        },
        StructuredContent::Styled(styled) => match &*styled.tag {
            //             "span" => {
            //                 if let Some(content) = &styled.content {
            //                     full_text.push_str("<span>");
            //                     extract_content(content.as_ref(), &mut full_text, img_prefix);
            //                     full_text.push_str("</span>");
            //                 }
            //             },
            //             "div" => {
            //                 if let Some(content) = &styled.content {
            //                     // if !full_text.ends_with('\n') {
            //                     //     full_text.push('\n');
            //                     // }
            //
            //         full_text.push_str("<div>");
            //         extract_content(content.as_ref(), &mut full_text, img_prefix);
            //         full_text.push_str("</div>");
            //     }
            // },
            // "ol" => {},
            // "ul" => {},
            // "li" => {},
            // "details" => {},
            // "summary" => {},
            tag => {
                if let Some(content) = &styled.content {
                    let style = if let Some(style) = &styled.style {
                        let value = style
                            .iter()
                            .map(|(a, b)| {
                                // TODO: optimization, do this once per item and cache previous
                                // values
                                let a = a.to_case(Case::Kebab);

                                let b = if b.is_string() {
                                    b.as_str().unwrap()
                                } else if b.is_f64() {
                                    &b.as_f64().unwrap().to_string()
                                } else if b.is_u64() {
                                    &b.as_u64().unwrap().to_string()
                                } else {
                                    unreachable!("idk what type b is {b:#?}");
                                };

                                format!("{a}:{}", b.replace('"', "'"))
                            })
                            .collect::<Vec<_>>()
                            .join(";");

                        format!("style=\"{value}\"")
                    } else {
                        String::new()
                    };

                    // if !full_text.ends_with('\n') {
                    //     full_text.push('\n');
                    // }

                    full_text.push_str(&format!("<{tag} {style}>"));
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str(&format!("</{tag}>"));
                }
            }
        },
        StructuredContent::Container(container) => match &*container.tag {
            "ruby" => {
                if let Some(content) = &container.content {
                    full_text.push_str("<ruby>");
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str("</ruby>");
                }
            }
            "rt" => {
                if let Some(content) = &container.content {
                    full_text.push_str("<rt>");
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str("</rt>");
                }
            }
            // "rp" => {},
            "table" => {
                if let Some(content) = &container.content {
                    full_text.push_str("<table>");
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str("</table>");
                }
            }
            // "thead" => {},
            // "tbody" => {},
            // "tfoot" => {},
            "tr" => {
                if let Some(content) = &container.content {
                    full_text.push_str("<tr>");
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str("</tr>");
                }
            }
            _ => unreachable!("not covered {}", container.tag),
        },
        StructuredContent::Image(image) => match &*image.tag {
            "img" => {
                full_text.push_str(&format!(
                    "<img style=\"height:{}{};width:{}{};\" src=\"{}{}\" />",
                    /*
                    height=\"{}\" width=\"{}\"
                    image
                        .height
                        .map(|it| (it.round() as u32).to_string())
                        .unwrap_or("".to_owned()),
                    image
                        .width
                        .map(|it| (it.round() as u32).to_string())
                        .unwrap_or("".to_owned()),
                    */
                    image.height.unwrap_or(1.0),
                    image.size_units.as_ref().unwrap_or(&SizeUnits::Em),
                    image.width.unwrap_or(1.0),
                    image.size_units.as_ref().unwrap_or(&SizeUnits::Em),
                    img_prefix,
                    image.path
                ));

                /*
                if let Some(content) = &container.content {
                    full_text.push_str("<table>");
                    extract_content(content.as_ref(), &mut full_text);
                    full_text.push_str("</table>");
                }
                */
            }
            _ => unreachable!("not covered"),
        },
        StructuredContent::Link(link) => match &*link.tag {
            "a" => {
                if let Some(content) = &link.content {
                    full_text.push_str("<a>");
                    extract_content(content.as_ref(), full_text, img_prefix);
                    full_text.push_str("</a>");
                }
            }
            _ => unreachable!("not covered {}", link.tag),
        },
        StructuredContent::Empty(empty) => match &*empty.tag {
            "br" => full_text.push_str("<br />"),
            _ => unreachable!("not covered {}", empty.tag),
        },
    }
}

#[derive(Debug, Deserialize)]
pub struct DefinitionText {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct DefinitionStructured {
    #[serde(rename = "type")]
    pub kind: String,
    pub content: StructuredContent,
}

impl<'de> serde::de::Deserialize<'de> for Definition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let raw: Box<RawValue> = Deserialize::deserialize(deserializer).unwrap();
        let value: Value = serde_json::from_str(raw.get()).unwrap();

        if value.is_string() {
            return Ok(Definition::Single(value.as_str().unwrap().to_owned()));
        }

        if value.is_object()
            && let Some(kind) = value.as_object().unwrap().get("type")
            && kind.is_string()
        {
            match kind.as_str().unwrap() {
                "text" => {
                    return Ok(Definition::Text(serde_json::from_str(raw.get()).unwrap()));
                }
                "structured-content" => {
                    return Ok(Definition::Structured(
                        serde_json::from_str(raw.get()).unwrap(),
                    ));
                }
                _ => {
                    panic!("Unknown key");
                }
            }
        }

        println!("{value:#?}");
        panic!("Shouldnt get here");
    }
}

#[derive(Debug)]
pub enum StructuredContent {
    /// Represents a text node.
    Text(String),
    /// An array of child content.
    Children(Vec<StructuredContent>),
    /// Table tags.
    Table(TableContent),
    /// Container tags supporting configurable styles.
    Styled(StyledContent),
    /// Generic container tags.
    Container(ContainerContent),
    /// Image tag.
    Image(ImageContent),
    /// Link tag.
    Link(LinkContent),
    /// Empty tags.
    Empty(EmptyContent),
}

impl ToString for StructuredContent {
    fn to_string(&self) -> String {
        match self {
            StructuredContent::Text(text) => text.clone(),
            StructuredContent::Children(vec) => vec
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
            StructuredContent::Table(table_content) => {
                println!("{table_content:#?}");
                String::new()
            }
            StructuredContent::Styled(styled_content) => styled_content.to_string(),
            StructuredContent::Container(container_content) => container_content.to_string(),
            StructuredContent::Image(image_content) => {
                // println!("{:#?}", image_content);
                String::new()
            }
            StructuredContent::Link(link_content) => link_content.to_string(),
            StructuredContent::Empty(empty_content) => empty_content.to_string(),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for StructuredContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let raw: Box<RawValue> = Deserialize::deserialize(deserializer).unwrap();
        let value: Value = serde_json::from_str(raw.get()).unwrap();

        if value.is_string() {
            return Ok(StructuredContent::Text(value.as_str().unwrap().to_owned()));
        }

        if value.is_array() {
            return Ok(StructuredContent::Children(
                serde_json::from_str(raw.get()).unwrap(),
            ));
        }

        if value.is_object()
            && let Some(tag) = value.as_object().unwrap().get("tag")
            && tag.is_string()
        {
            match tag.as_str().unwrap() {
                "td" | "th" => {
                    return Ok(StructuredContent::Table(
                        serde_json::from_str(raw.get()).unwrap(),
                    ));
                }
                "span" | "div" | "ol" | "ul" | "li" | "details" | "summary" => {
                    return Ok(StructuredContent::Styled(
                        serde_json::from_str(raw.get()).unwrap(),
                    ));
                }
                "ruby" | "rt" | "rp" | "table" | "thead" | "tbody" | "tfoot" | "tr" => {
                    return Ok(StructuredContent::Container(
                        serde_json::from_str(raw.get()).unwrap(),
                    ));
                }
                "img" => {
                    return Ok(StructuredContent::Image(
                        serde_json::from_str(raw.get()).unwrap(),
                    ));
                }
                "a" => {
                    return Ok(StructuredContent::Link(
                        serde_json::from_str(raw.get()).unwrap(),
                    ));
                }
                "br" => {
                    return Ok(StructuredContent::Empty(
                        serde_json::from_str(raw.get()).unwrap(),
                    ));
                }
                _ => {
                    panic!("Unknown key");
                }
            }
        }

        println!("{value:#?}");
        panic!("Shouldnt get here");
    }
}

#[derive(Debug, Deserialize)]
pub struct TableContent {
    /// "td", "th"
    pub tag: String,
    /// ...
    pub content: Option<Box<StructuredContent>>,
    /// Generic data attributes that should be added to the element.
    pub data: Option<HashMap<String, String>>,
    /// ...
    #[serde(rename = "colSpan")]
    pub col_span: Option<i32>,
    /// ...
    #[serde(rename = "rowSpan")]
    pub row_span: Option<i32>,
    /// CSS styles to apply to the element.
    pub style: Option<HashMap<String, Value>>,
    /// Defines the language of an element in the format defined by RFC
    /// 5646.
    pub lang: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StyledContent {
    /// "span", "div", "ol", "ul", "li", "details", "summary"
    pub tag: String,
    /// ...
    pub content: Option<Box<StructuredContent>>,
    /// Generic data attributes that should be added to the element.
    pub data: Option<HashMap<String, String>>,
    /// CSS styles to apply to the element.
    pub style: Option<HashMap<String, Value>>,
    /// Hover text for the element.
    pub title: Option<String>,
    /// Defines the language of an element in the format defined by RFC
    /// 5646.
    pub lang: Option<String>,
}

impl ToString for StyledContent {
    fn to_string(&self) -> String {
        self.content
            .as_ref()
            .map(|it| it.to_string())
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
pub struct ContainerContent {
    /// "ruby", "rt", "rp", "table", "thead", "tbody", "tfoot","tr"
    pub tag: String,
    /// ...
    pub content: Option<Box<StructuredContent>>,
    /// Generic data attributes that should be added to the element.
    pub data: Option<HashMap<String, String>>,
    /// Defines the language of an element in the format defined by RFC
    /// 5646.
    pub lang: Option<String>,
}

impl ToString for ContainerContent {
    fn to_string(&self) -> String {
        if self.tag == "rt" || self.tag == "rp" {
            return String::new();
        }

        self.content
            .as_ref()
            .map(|it| it.to_string())
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageContent {
    // Specific value: "img"
    pub tag: String,
    /// Generic data attributes that should be added to the element.
    pub data: Option<HashMap<String, String>>,
    /// Path to the image file in the archive.
    pub path: String,
    /// Preferred width of the image.
    pub width: Option<f32>,
    /// Preferred height of the image.
    pub height: Option<f32>,
    /// Hover text for the image.
    pub title: Option<String>,
    /// Alt text for the image.
    pub alt: Option<String>,
    /// Description of the image.
    pub description: Option<String>,
    /// Whether or not the image should appear pixelated at sizes larger than
    /// the image's native resolution.
    #[serde(default = "Default::default")]
    pub pixelated: bool,
    /// Controls how the image is rendered. The value of this field supersedes
    /// the pixelated field.
    #[serde(default = "Default::default")]
    #[serde(rename = "imageRendering")]
    pub image_rendering: ImageRendering,
    /// Controls the appearance of the image. The "monochrome" value will mask
    /// the opaque parts of the image using the current text color.
    #[serde(default = "Default::default")]
    pub appearance: Appearance,
    /// Whether or not a background color is displayed behind the image.
    #[serde(default = "default_true")]
    pub background: bool,
    /// Whether or not the image is collapsed by default.
    pub collapsed: bool,
    /// Whether or not the image can be collapsed.
    #[serde(default = "Default::default")]
    pub collapsable: bool,
    /// The vertical alignment of the image.
    #[serde(rename = "verticalAlign")]
    pub vertical_align: Option<VerticalAlign>,
    /// Shorthand for border width, style, and color.
    pub border: Option<String>,
    /// Roundness of the corners of the image's outer border edge.
    #[serde(rename = "borderRadius")]
    pub border_radius: Option<String>,
    /// The units for the width and height.
    #[serde(rename = "sizeUnits")]
    pub size_units: Option<SizeUnits>,
}

#[derive(Debug, Deserialize)]
pub enum ImageRendering {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "pixelated")]
    Pixelated,
    #[serde(rename = "crisp-edges")]
    CrispEdges,
}

impl Default for ImageRendering {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Deserialize, Default)]
pub enum Appearance {
    #[serde(rename = "auto")]
    #[default]
    Auto,
    #[serde(rename = "monochrome")]
    Monochrome,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerticalAlign {
    Baseline,
    Sub,
    Super,
    TextTop,
    TextBottom,
    Middle,
    Top,
    Bottom,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SizeUnits {
    Px,
    Em,
}

impl Display for SizeUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SizeUnits::Px => f.write_str("px"),
            SizeUnits::Em => f.write_str("em"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LinkContent {
    // Specific value: "a"
    pub tag: String,
    /// ...
    pub content: Option<Box<StructuredContent>>,
    /// The URL for the link. URLs starting with a ? are treated as internal
    /// links to other dictionary content. Must match regular expression:
    /// ^(?:https?:|\?)[\w\W]*
    pub href: String,
    /// Defines the language of an element in the format defined by RFC 5646.
    pub lang: Option<String>,
}

impl ToString for LinkContent {
    fn to_string(&self) -> String {
        self.content
            .as_ref()
            .map(|it| it.to_string())
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
pub struct EmptyContent {
    ///  Specific value: "br"
    pub tag: String,
    /// Generic data attributes that should be added to the element.
    pub data: Option<HashMap<String, String>>,
}

impl ToString for EmptyContent {
    fn to_string(&self) -> String {
        match &*self.tag {
            "br" => "\n".to_owned(),
            _ => todo!(),
        }
    }
}

const fn default_true() -> bool {
    true
}
