pub mod error;
pub mod lang;

use core::fmt::Debug;
use std::path::Path;
use std::sync::Arc;

use error::Result;
use error::UnknownTypeContext;
use lang::jp::yomitan::YomitanDictionary;

#[derive(Debug)]
pub struct DictionaryInfo {
    /// The title of the dictionary.
    pub title: String,
    ///
    pub description: String,

    pub source_language: Option<String>,

    pub target_language: Option<String>,
}

#[derive(Debug)]
pub struct DictionaryWord {
    pub word: String,
    pub reading: String,
    pub definition: String,
}

#[derive(Clone, Debug)]
pub struct Frequency {
    pub word: String,
    pub reading: String,
    pub frequency: u32,
    pub display: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Accent {
    pub word: String,
    pub reading: String,
    pub position: u32,
}

pub trait Dictionary: Send + Sync {
    fn info(&self) -> DictionaryInfo;

    fn words(&self, img_prefix: Option<String>) -> Vec<DictionaryWord>;

    fn frequencies(&self) -> Vec<Frequency>;

    fn accents(&self) -> Vec<Accent>;
}

pub fn load_dictionary<P: AsRef<Path>>(path: &P, data_path: &Path) -> Result<Arc<dyn Dictionary>> {
    if let Some(dictionary) = YomitanDictionary::try_from_path(path, data_path)? {
        return Ok(Arc::new(dictionary));
    }

    UnknownTypeContext {
        path: path.as_ref().to_string_lossy().to_string(),
    }
    .fail()
}

/*
pub async fn import_dictionary(
    State(AppState { db, .. }): State<AppState>,
    Json(data): Json<ImportRequest>,
) -> Json<ImportResponse> {
    let dictionary = Dictionary::from_path(data.dictionary_path).unwrap();

    match dictionary {
        Dictionary::Yomitan(dictionary) => {
            test(dictionary, &db).await;
        },
    }

    Json(ImportResponse {})
}

async fn test(dictionary: YomitanDictionary, db: &PrismaClient) {
    let title = dictionary.index.title;

    let existing = db
        .dictionary()
        .find_first(vec![dictionary::title::equals(title.clone())])
        .exec()
        .await
        .unwrap();

    if let Some(existing) = existing {
        panic!("dictionary exists");
    }

    let data = db
        .dictionary()
        .create(
            "".to_owned(),
            "".to_owned(),
            "yomitan".to_owned(),
            title.clone(),
            vec![],
        )
        .exec()
        .await
        .unwrap();

    // TODO: create_many pls!!!!
    db.word()
        .create_many(
            dictionary
                .terms
                .par_iter()
                .map(|term| {
                    let mut definition = String::new();

                    for def in &term.definitions {
                        match def {
                            Definition::Single(single) => {
                                definition.push_str(&format!("{}\n\n", single))
                            },
                            Definition::Text(text) => {
                                definition.push_str(&format!("{}\n\n", text.text))
                            },
                            Definition::Structured(structured) => {
                                extract_content(&structured.content, &mut definition)
                            },
                        }
                    }

                    word::create_unchecked(term.term.clone(), definition, data.id, vec![])
                })
                .collect(),
        )
        .exec()
        .await
        .unwrap();
}

fn extract_content(content: &StructuredContent, mut full_text: &mut String) {
    match content {
        StructuredContent::Text(text) => {
            full_text.push_str(&text);
        },
        StructuredContent::Children(children) => {
            for child in children {
                extract_content(child, &mut full_text);
            }
        },
        StructuredContent::Table(table) => match &table.tag[..] {
            // "td" => {},
            // "th" => {},
            _ => unreachable!("not covered {}", table.tag),
        },
        StructuredContent::Styled(styled) => match &styled.tag[..] {
            "span" => {
                if let Some(content) = &styled.content {
                    extract_content(content.as_ref(), &mut full_text)
                }
            },
            "div" => {
                if let Some(content) = &styled.content {
                    if !full_text.ends_with('\n') {
                        full_text.push('\n');
                    }

                    extract_content(content.as_ref(), &mut full_text)
                }
            },
            // "ol" => {},
            // "ul" => {},
            // "li" => {},
            // "details" => {},
            // "summary" => {},
            _ => unreachable!("not covered {}", styled.tag),
        },
        StructuredContent::Container(container) => match &container.tag[..] {
            "ruby" => {
                if let Some(content) = &container.content {
                    extract_content(content.as_ref(), &mut full_text)
                }
            },
            "rt" => { /* noop */ },
            // "rp" => {},
            "table" => { /* noop */ },
            // "thead" => {},
            // "tbody" => {},
            // "tfoot" => {},
            // "tr" => {},
            _ => unreachable!("not covered {}", container.tag),
        },
        StructuredContent::Image(image) => match &image.tag[..] {
            "img" => { /* noop */ },
            _ => unreachable!("not covered"),
        },
        StructuredContent::Link(link) => match &link.tag[..] {
            "a" => {
                if let Some(content) = &link.content {
                    extract_content(content.as_ref(), &mut full_text)
                }
            },
            _ => unreachable!("not covered {}", link.tag),
        },
        StructuredContent::Empty(empty) => match &empty.tag[..] {
            "br" => full_text.push('\n'),
            _ => unreachable!("not covered {}", empty.tag),
        },
    }
}
*/
