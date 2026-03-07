use std::cell::Cell;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Deref;
use std::ops::Mul;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use blart::TreeMap;
use railgun::di::Component;
use shared::OnStartup;
use shared::infra::database::Sqlite;
use trie_rs::map::Trie;

/*
            ._query_raw(raw!(
                r#"
SELECT
    Word.word, Word.reading, MIN(Frequency.frequency) as frequency
FROM
    Word
LEFT JOIN
    Frequency
ON
    Frequency.word = Word.word
GROUP BY
    Word.word
                "#
            )).exec().await?;
*/

#[derive(Component)]
#[component(implements(Vec<dyn OnStartup>))]
pub struct DictionaryTrieService {
    #[inject(default)]
    words: Option<Trie<u8, Option<usize>>>,
    #[inject(default)]
    readings: Option<Trie<u8, Option<usize>>>,
    #[inject(default)]
    buf: (Pin<Box<[u8]>>, Pin<Box<[u8]>>),
}

impl DictionaryTrieService {
    pub fn get(&self) -> &Trie<u8, Option<usize>> {
        self.words.as_ref().unwrap()
    }

    pub fn get_readings(&self) -> &Trie<u8, Option<usize>> {
        self.readings.as_ref().unwrap()
    }
}

#[async_trait]
impl OnStartup for DictionaryTrieService {
    async fn run(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        let (buf, words, readings) = load_dictionaries();

        self.buf = buf;
        self.words = Some(words);
        self.readings = Some(readings);

        Ok(())
    }
}

/*
SELECT
    Word.word, Word.reading, MIN(Frequency.frequency) as frequency
FROM
    Word
LEFT JOIN
        Frequency
ON
    Frequency.word = Word.word
GROUP BY Word.word

*/

fn load_words() -> (Pin<Box<[u8]>>, Trie<u8, Option<usize>>) {
    let time = std::time::Instant::now();

    let home = std::env::var("HOME").unwrap();
    let path = PathBuf::from(format!("{home}/opt/dictionary_words.csv"));

    let lines = BufReader::new(File::open(&path).unwrap()).lines();
    let len = std::fs::metadata(path).unwrap().len();

    let mut buf = Pin::new(
        vec![0; usize::try_from(len).expect("usize should always fit within u64")]
            .into_boxed_slice(),
    );

    let mut words = trie_rs::map::TrieBuilder::<u8, Option<usize>>::new();

    let mut view = &mut buf[..];

    for line in lines.map_while(Result::ok) {
        if line == r#""""# {
            continue;
        }

        if let Some((word, freq)) = line.split_once(',') {
            let freq = freq.parse::<usize>().ok();

            if word != r#""""# {
                assert!(
                    memchr::memchr(0, word.as_bytes()).is_none(),
                    "string with null byte"
                );

                let len = word.len();

                view[..len].copy_from_slice(word.as_bytes());

                // SAFETY: The type is coerced into a static str so that
                let word: &'static str = unsafe { std::mem::transmute(&view[..len]) };

                words.push(word, freq);

                view = &mut view[len..];
            }
        }
    }

    let words = words.build();

    (buf, words)
}

fn load_readings() -> (Pin<Box<[u8]>>, Trie<u8, Option<usize>>) {
    let home = std::env::var("HOME").unwrap();
    let path = PathBuf::from(format!("{home}/opt/dictionary_readings.csv"));

    let lines = BufReader::new(File::open(&path).unwrap()).lines();
    let len = std::fs::metadata(path).unwrap().len();

    let mut buf = Pin::new(
        vec![0; usize::try_from(len).expect("usize should always fit within u64")]
            .into_boxed_slice(),
    );

    let mut readings = trie_rs::map::TrieBuilder::<u8, Option<usize>>::new();

    let mut view = &mut buf[..];

    for line in lines.map_while(Result::ok) {
        if line == r#""""# {
            continue;
        }

        if let Some((word, freq)) = line.split_once(',') {
            let freq = freq.parse::<usize>().ok();

            if word != r#""""# {
                assert!(
                    memchr::memchr(0, word.as_bytes()).is_none(),
                    "string with null byte"
                );

                let len = word.len();

                view[..len].copy_from_slice(word.as_bytes());

                // SAFETY: The type is coerced into a static str so that
                let word: &'static str = unsafe { std::mem::transmute(&view[..len]) };

                readings.push(word, freq);

                view = &mut view[len..];
            }
        }
    }

    let readings = readings.build();

    (buf, readings)
}

fn load_dictionaries() -> (
    (Pin<Box<[u8]>>, Pin<Box<[u8]>>),
    Trie<u8, Option<usize>>,
    Trie<u8, Option<usize>>,
) {
    let (words_buf, words) = load_words();
    let (readings_buf, readings) = load_readings();

    ((words_buf, readings_buf), words, readings)
}
