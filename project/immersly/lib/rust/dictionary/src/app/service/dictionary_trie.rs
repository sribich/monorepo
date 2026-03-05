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
    trie: TreeMap<&'static CStr, Option<usize>>,
    #[inject(default)]
    trie_readings: TreeMap<&'static CStr, Option<usize>>,
    // MUST COME LAST -- HOLDS TREEMAP DATA
    #[inject(default)]
    buf: Pin<Box<[u8]>>,
}

impl DictionaryTrieService {
    pub fn get(&self) -> &TreeMap<&'static CStr, Option<usize>> {
        &self.trie
    }

    pub fn get_readings(&self) -> &TreeMap<&'static CStr, Option<usize>> {
        &self.trie_readings
    }
}

#[async_trait]
impl OnStartup for DictionaryTrieService {
    async fn run(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        let home = std::env::var("HOME").unwrap();
        let path = PathBuf::from(format!("{home}/opt/dictionary_entries.csv"));

        let lines = BufReader::new(File::open(&path).unwrap()).lines();
        let len = std::fs::metadata(path).unwrap().len();

        let mut buf = Pin::new(
            vec![0; usize::try_from(len.mul(2)).expect("usize should always fit within u64")]
                .into_boxed_slice(),
        );

        let mut view = &mut buf[..];

        let mut adaptive = TreeMap::<&CStr, Option<usize>>::new();
        let mut adaptive_readings = TreeMap::<&CStr, Option<usize>>::new();

        #[expect(clippy::indexing_slicing, unsafe_code, reason = "See safety comment")]
        for line in lines.map_while(Result::ok) {
            if line == r#""""# {
                continue;
            }

            if let Some((left, right)) = line.split_once(',')
                && let Some((middle, right)) = right.split_once(',')
            {
                let freq = right.parse::<usize>().ok();

                if left != r#""""# {
                    assert!(
                        memchr::memchr(0, left.as_bytes()).is_none(),
                        "string with null byte"
                    );

                    let left_len = left.len();

                    view[..left_len].copy_from_slice(left.as_bytes());
                    view[left_len] = 0;

                    // SAFETY: The type is coerced into a static str so that
                    let left: &'static CStr = unsafe {
                        std::mem::transmute(CStr::from_bytes_with_nul_unchecked(&view[..=left_len]))
                    };
                    adaptive.try_insert(left, freq).unwrap();

                    view = &mut view[(left_len + 1)..];
                }

                if middle != r#""""# {
                    assert!(
                        memchr::memchr(0, middle.as_bytes()).is_none(),
                        "string with null byte"
                    );

                    let middle_len = middle.len();

                    view[..middle_len].copy_from_slice(middle.as_bytes());
                    view[middle_len] = 0;

                    let middle = unsafe {
                        std::mem::transmute(CStr::from_bytes_with_nul_unchecked(
                            &view[..=middle_len],
                        ))
                    };
                    adaptive_readings.try_insert(middle, freq).unwrap();

                    view = &mut view[(middle_len + 1)..];
                }
            }
        }

        self.buf = buf;
        self.trie = adaptive;
        self.trie_readings = adaptive_readings;

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
