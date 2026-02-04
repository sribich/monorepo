pub mod ast;

use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use ast::YomitanFile;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use zip::ZipArchive;

use crate::Accent;
use crate::Dictionary;
use crate::DictionaryInfo;
use crate::DictionaryWord;
use crate::Frequency;
use crate::error::Result;

#[derive(Debug)]
pub struct YomitanDictionary {
    pub index: ast::Index,
    pub terms: Vec<ast::Term>,
    pub accents: Vec<Accent>,
    pub frequencies: Vec<Frequency>,
}

impl Dictionary for YomitanDictionary {
    fn info(&self) -> DictionaryInfo {
        DictionaryInfo {
            title: self.index.title.clone(),
            description: self.index.description.clone().unwrap_or_default(),
            source_language: self.index.source_language.clone(),
            target_language: self.index.target_language.clone(),
        }
    }

    fn words(&self, img_prefix: Option<String>) -> Vec<DictionaryWord> {
        self.terms
            .iter()
            .map(|term| {
                if term.definitions.len() > 1 {
                    println!("{} definitions", term.definitions.len());
                }

                DictionaryWord {
                    word: term.term.clone(),
                    reading: term.reading.clone(),
                    definition: term.definitions[0].to_string(&img_prefix),
                }
            })
            .collect::<Vec<_>>()
    }

    fn frequencies(&self) -> Vec<crate::Frequency> {
        self.frequencies.clone()
    }

    fn accents(&self) -> Vec<crate::Accent> {
        self.accents.clone()
    }
}

impl YomitanDictionary {
    pub fn try_from_path<P: AsRef<Path>>(
        path: P,
        data_path: &Path,
    ) -> Result<Option<YomitanDictionary>> {
        let path = path.as_ref();

        if !path.is_file() {
            return Ok(None);
        }

        if let Some(extension) = path.extension()
            && extension == "zip"
        {
            return Self::try_from_zip(path, data_path);
        }

        if path.ends_with("index.json") {
            return Self::try_from_index(path);
        }

        Ok(None)
    }

    fn try_from_zip(path: &Path, data_path: &Path) -> Result<Option<Self>> {
        let file = std::fs::File::open(path).unwrap(); // .context(FsContext {})?;
        let zip = Arc::new(Mutex::new(ZipArchive::new(file).unwrap()));

        {
            zip.lock().expect("Poisoned").extract(data_path).unwrap();
        };

        let file_names = zip
            .lock()
            .expect("Mutex is poisoned")
            .file_names()
            .map(str::to_owned)
            .collect::<Vec<_>>();

        let data = file_names
            .par_iter()
            .filter(|it| it.ends_with(".json"))
            .map(|file| {
                let mut content = String::new();

                {
                    Arc::clone(&zip)
                        .lock()
                        .expect("Mutex is poisoned")
                        .by_name(file)
                        .unwrap()
                        .read_to_string(&mut content)
                        .unwrap()
                };

                let file_name_len = file.chars().count();
                let file_name_idx = file
                    .char_indices()
                    .nth(std::cmp::min(file_name_len - 1, 7))
                    .unwrap()
                    .0;

                match &file[..file_name_idx] {
                    // index.json
                    "index.j" => Some(YomitanFile::Index(serde_json::from_str(&content).unwrap())),
                    // kanji_bank_{number}.json
                    "kanji_b" => Some(YomitanFile::KanjiTerm(
                        serde_json::from_str(&content).unwrap(),
                    )),
                    // kanji_meta_bank_{number}.json
                    "kanji_m" => Some(YomitanFile::KanjiMeta(
                        serde_json::from_str(&content).unwrap(),
                    )),
                    // tag_bank_{number}.json
                    "tag_ban" => Some(YomitanFile::Tag(serde_json::from_str(&content).unwrap())),
                    // term_bank_{number}.json
                    "term_ba" => Some(YomitanFile::Term(serde_json::from_str(&content).unwrap())),
                    // term_meta_bank_{number}.json
                    "term_me" => Some(YomitanFile::TermMeta(
                        serde_json::from_str(&content).unwrap(),
                    )),
                    _ => {
                        None
                        // panic!("Unknown file {file}");
                    }
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        struct EnumCollection {
            index: Option<ast::Index>,
            terms: Vec<ast::Term>,
            accents: Vec<Accent>,
            frequencies: Vec<Frequency>,
        }

        let mut collection = EnumCollection {
            index: None,
            terms: vec![],
            accents: vec![],
            frequencies: vec![],
        };

        for item in data {
            match item {
                YomitanFile::Index(item) => {
                    assert!(collection.index.is_none(), "Duplicate index");

                    collection.index = Some(item);
                }
                YomitanFile::KanjiTerm(item) => {}
                YomitanFile::KanjiMeta(item) => {}
                YomitanFile::Tag(item) => {}
                YomitanFile::Term(mut item) => {
                    collection.terms.append(&mut item);
                }
                YomitanFile::TermMeta(items) => {
                    for item in items {
                        match item {
                            ast::TermMeta::Frequency {
                                word,
                                reading,
                                frequency,
                                display,
                            } => collection.frequencies.push(Frequency {
                                word,
                                reading,
                                frequency,
                                display,
                            }),
                            ast::TermMeta::PitchAccent {
                                word,
                                reading,
                                pitches,
                            } => {
                                for pitch in pitches {
                                    collection.accents.push(Accent {
                                        word: word.clone(),
                                        reading: reading.clone(),
                                        position: pitch.position,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Some(Self {
            index: collection.index.unwrap(),
            terms: collection.terms,
            accents: collection.accents,
            frequencies: collection.frequencies,
        }))
    }

    fn try_from_index(path: &Path) -> Result<Option<Self>> {
        Ok(None)
    }
}
