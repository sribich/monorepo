mod char_def;
mod connection_matrix;
mod double_array_trie;
mod feature;
mod overlay;
mod sys_dic;
mod unknown;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub use char_def::CharCategory;
pub use char_def::CharDef;
pub use char_def::CharInfo;
pub use connection_matrix::ConnectionMatrix;
pub use double_array_trie::DartsResult;
pub use double_array_trie::DoubleArrayTrie;
pub use feature::FeatureTable;
use memmap2::Mmap;
pub use overlay::OverlayDictionary;
pub use overlay::OverlayEntry;
use railgun_error::ResultExt;
use railgun_error::ensure;
pub use sys_dic::SysDic;
pub use sys_dic::Token;
pub use unknown::UnknownDictionary;

use crate::error::NotFoundContext;
use crate::error::ParseError;
use crate::lattice::LatticeNode;

/// System dictionary file name
const SYS_DIC_FILE: &str = "sys.dic";
/// Unknown word dictionary file name
const UNK_DIC_FILE: &str = "unk.dic";
/// Connection matrix file name
const MATRIX_FILE: &str = "matrix.bin";
/// Character property file name
const CHAR_BIN_FILE: &str = "char.bin";

pub const MECAB_SYS_DIC: u32 = 0;
pub const MECAB_USR_DIC: u32 = 1;
pub const MECAB_UNK_DIC: u32 = 2;

/// The dictionary containing all the sub-dictionaries that are required
/// for locating and scoring words.
#[derive(Debug)]
pub struct Dictionary {
    /// System dictionary (contains trie, tokens, features)
    pub sys_dic: SysDic<'static>,
    /// Unknown word dictionary
    pub unknown: UnknownDictionary<'static>,
    /// Connection matrix for transition costs
    pub matrix: ConnectionMatrix<'static>,
    /// Character category definitions
    pub char_def: CharDef<'static>,
    /// Overlay dictionary for runtime word additions
    pub overlay: OverlayDictionary,
    /// Memory maps (kept alive to maintain the mapped regions)
    _mmaps: Vec<Arc<Mmap>>,
}

impl Dictionary {
    /// Loads a dictionary from the specified directory
    pub fn load(path: &Path) -> Result<Self, ParseError> {
        ensure!(
            path.exists(),
            NotFoundContext {
                path: path.to_path_buf()
            }
        );

        let mut mmaps = Vec::new();

        let sys_path = path.join(SYS_DIC_FILE);
        let sys_mmap = Arc::new(Self::open_mmap(&sys_path)?);
        let sys_dic = SysDic::from_mmap(&sys_mmap)?;
        mmaps.push(Arc::clone(&sys_mmap));

        let matrix_path = path.join(MATRIX_FILE);
        let matrix_mmap = Arc::new(Self::open_mmap(&matrix_path)?);
        let matrix = ConnectionMatrix::from_mmap(&matrix_mmap)?;
        mmaps.push(Arc::clone(&matrix_mmap));

        let char_path = path.join(CHAR_BIN_FILE);
        let char_mmap = Arc::new(Self::open_mmap(&char_path)?);
        let char_def = CharDef::from_mmap(&char_mmap)?;
        mmaps.push(Arc::clone(&char_mmap));

        let unk_path = path.join(UNK_DIC_FILE);
        let unk_mmap = Arc::new(Self::open_mmap(&unk_path)?);
        let unknown = UnknownDictionary::from_mmap(&unk_mmap)?;
        mmaps.push(Arc::clone(&unk_mmap));

        Ok(Self {
            sys_dic: unsafe { std::mem::transmute::<SysDic<'_>, SysDic<'static>>(sys_dic) },
            unknown: unsafe {
                std::mem::transmute::<UnknownDictionary<'_>, UnknownDictionary<'static>>(unknown)
            },
            matrix: unsafe { std::mem::transmute::<_, _>(matrix) },
            char_def: unsafe { std::mem::transmute::<CharDef<'_>, CharDef<'static>>(char_def) },
            overlay: OverlayDictionary::new(),
            _mmaps: mmaps,
        })
    }

    /// Attempts to open a memory-mapped file.
    fn open_mmap(path: &Path) -> Result<Mmap, ParseError> {
        ensure!(
            path.exists(),
            NotFoundContext {
                path: path.to_path_buf()
            }
        );

        let file = File::open(path).boxed_local()?;
        let mmap = unsafe { Mmap::map(&file).boxed_local()? };

        Ok(mmap)
    }

    /// Look up a word in the dictionary using common prefix search
    ///
    /// This checks the overlay dictionary first, then the system dictionary.
    /// Returns all matching entries from both layers.
    pub fn lookup(&self, key: &str) -> Vec<DictionaryEntry> {
        // Check overlay first (hot path for new words)
        let mut results = self.overlay.lookup(key);

        // Then check system dictionary
        results.extend(self.sys_dic.common_prefix_search(key));

        results
    }

    /// Add a word to the overlay dictionary at runtime
    ///
    /// This allows adding new words without restarting or reloading
    /// the main dictionary.
    ///
    /// # Arguments
    ///
    /// * `surface` - The surface form (the actual text)
    /// * `entry` - The dictionary entry data
    ///
    /// # Example
    ///
    /// ```ignore
    /// use mecrab::dict::{Dictionary, OverlayEntry};
    ///
    /// let dict = Dictionary::load(path)?;
    /// dict.add_word("ChatGPT", OverlayEntry::new(
    ///     "名詞,固有名詞,一般,*,*,*,ChatGPT,チャットジーピーティー,チャットジーピーティー",
    ///     5000,
    /// ));
    /// ```
    pub fn add_word(&self, surface: &str, entry: OverlayEntry) {
        self.overlay.add_word(surface, entry);
    }

    /// Add a word with simple parameters
    ///
    /// Convenience method that creates a common noun entry.
    ///
    /// # Arguments
    ///
    /// * `surface` - The surface form
    /// * `reading` - The katakana reading
    /// * `pronunciation` - The pronunciation
    /// * `wcost` - Word cost (lower = more preferred)
    pub fn add_simple_word(&self, surface: &str, reading: &str, pronunciation: &str, wcost: i16) {
        self.overlay
            .add_simple(surface, reading, pronunciation, wcost);
    }

    /// Remove a word from the overlay dictionary
    ///
    /// Note: This only removes words from the overlay layer.
    /// System dictionary entries cannot be removed.
    ///
    /// Returns true if the word was found and removed.
    pub fn remove_word(&self, surface: &str) -> bool {
        self.overlay.remove_word(surface)
    }

    /// Get the number of words in the overlay dictionary
    pub fn overlay_size(&self) -> usize {
        self.overlay.len()
    }

    /// Get the connection cost between two context IDs
    ///
    /// # Arguments
    ///
    /// * `right_id` - Right context ID of the left node
    /// * `left_id` - Left context ID of the right node
    #[inline]
    pub fn connection_cost(
        &self,
        left_node: &LatticeNode<'_>,
        right_node: &LatticeNode<'_>,
    ) -> i32 {
        self.matrix.cost(left_node, right_node)
    }

    /// Get connection cost without bounds checking (hot path optimization)
    ///
    /// # Safety
    ///
    /// Context IDs must be from valid dictionary entries.
    #[inline]
    pub unsafe fn connection_cost_unchecked(&self, right_id: u16, left_id: u16) -> i16 {
        // Safety: caller guarantees IDs are valid
        unsafe { self.matrix.cost_unchecked(right_id, left_id) }
    }

    /// Get the feature string for a token
    pub fn get_feature(&self, token: &Token) -> &str {
        self.sys_dic.get_feature(token)
    }

    /// Get character info for a character
    pub fn char_info(&self, c: char) -> CharInfo {
        self.char_def.get_char_info(c)
    }

    /// Get the character category for a character
    pub fn char_category(&self, c: char) -> CharCategory {
        self.char_def.get_char_info(c).category()
    }

    /// Get the charset of the dictionary
    pub fn charset(&self) -> &str {
        self.sys_dic.charset()
    }

    /// Get the number of entries in the dictionary
    pub fn size(&self) -> usize {
        self.sys_dic.lexicon_size()
    }
}

/// A dictionary entry returned from lookup
#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    /// Length of the matched surface in bytes
    pub length: usize,
    /// Word ID (token index in dictionary, used for word embeddings)
    pub word_id: u32,
    /// Left context ID (for connection matrix)
    pub left_id: u16,
    /// Right context ID (for connection matrix)
    pub right_id: u16,
    /// Part of speech ID
    pub pos_id: u16,
    /// Word cost
    pub wcost: i16,
    /// Feature string
    pub feature: String,
}

#[cfg(test)]
mod integration_tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_load_ipadic() {
        let path = Path::new("/var/lib/mecab/dic/ipadic-utf8");
        if !path.exists() {
            eprintln!("IPADIC not found, skipping test");
            return;
        }

        let dict = Dictionary::load(path).expect("Failed to load dictionary");

        eprintln!("Dictionary loaded:");
        eprintln!("  Size: {} entries", dict.size());
        eprintln!("  Charset: {}", dict.charset());

        // Test lookup for 'の'
        let entries = dict.lookup("の");
        eprintln!("\nLookup 'の': {} entries", entries.len());
        for entry in entries.iter().take(3) {
            eprintln!(
                "  length={}, wcost={}, feature={}",
                entry.length, entry.wcost, entry.feature
            );
        }
        assert!(!entries.is_empty(), "Expected to find 'の' in dictionary");

        // Test lookup for 'テスト'
        let entries = dict.lookup("テスト");
        eprintln!("\nLookup 'テスト': {} entries", entries.len());
        for entry in entries.iter().take(3) {
            eprintln!(
                "  length={}, wcost={}, feature={}",
                entry.length, entry.wcost, entry.feature
            );
        }
        assert!(
            !entries.is_empty(),
            "Expected to find 'テスト' in dictionary"
        );

        // Test lookup for '東京'
        let entries = dict.lookup("東京");
        eprintln!("\nLookup '東京': {} entries", entries.len());
        for entry in entries.iter().take(3) {
            eprintln!(
                "  length={}, wcost={}, feature={}",
                entry.length, entry.wcost, entry.feature
            );
        }
        assert!(!entries.is_empty(), "Expected to find '東京' in dictionary");
    }
}
