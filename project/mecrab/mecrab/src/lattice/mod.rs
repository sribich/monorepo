//! Lattice module for building the word graph
//!
//! Copyright 2026 COOLJAPAN OU (Team KitaSan)
//!
//! The lattice represents all possible segmentations of the input text
//! as a directed acyclic graph (DAG). Each node represents a potential
//! word, and edges connect adjacent words.

// mod visualize;

use std::cell::Cell;

use id_arena::Arena;
use id_arena::Id;

// pub use visualize::DotBuilder;
// pub use visualize::DotConfig;
// pub use visualize::NodeShape;
// pub use visualize::RankDir;
use crate::Result;
use crate::dict::CharCategory;
use crate::dict::Dictionary;
use crate::dict::DictionaryEntry;

/// A node in the lattice representing a potential word/token
#[derive(Debug, Clone)]
pub struct LatticeNode<'a> {
    /// Surface form (slice into original text)
    pub surface: &'a str,

    /// Start position in bytes
    pub start: usize,
    /// End position in bytes
    pub end: usize,
    /// Word ID (token index in dictionary, used for embeddings)
    pub word_id: u32,
    /// Left context ID for connection matrix
    pub left_id: u16,
    /// Right context ID for connection matrix
    pub right_id: u16,
    /// Part-of-speech ID
    pub pos_id: u16,
    /// Word cost from dictionary
    pub wcost: i16,
    /// Feature string (lazily loaded)
    pub feature: String,
    /// Whether this is an unknown word
    pub is_unknown: bool,

    ///
    pub cost: Cell<i64>,
    pub lnode_id: Cell<Option<Id<LatticeNode<'a>>>>,
    pub is_best: Cell<bool>,
}

impl<'a> LatticeNode<'a> {
    /// Create a BOS (Beginning of Sentence) node
    pub fn bos() -> Self {
        Self {
            surface: "",
            start: 0,
            end: 0,
            word_id: u32::MAX, // BOS/EOS don't have word_id
            left_id: 0,
            right_id: 0,
            pos_id: 0,
            wcost: 0,
            feature: "BOS/EOS".to_string(),
            is_unknown: false,

            //
            cost: Cell::new(0),
            lnode_id: Cell::new(None),
            is_best: Cell::new(false),
        }
    }

    /// Create a new lattice node from an owned dictionary entry (moves feature)
    pub fn from_entry(text: &'a str, start: usize, entry: DictionaryEntry) -> Self {
        let end = start + entry.length;

        Self {
            surface: &text[start..end],
            start,
            end,
            word_id: entry.word_id,
            left_id: entry.left_id,
            right_id: entry.right_id,
            pos_id: entry.pos_id,
            wcost: entry.wcost,
            feature: entry.feature,
            is_unknown: false,
            //
            cost: Cell::new(0),
            lnode_id: Cell::new(None),
            is_best: Cell::new(false),
        }
    }

    /// Create an EOS (End of Sentence) node
    pub fn eos(position: usize) -> Self {
        Self {
            surface: "",
            start: position,
            end: position,
            word_id: u32::MAX, // BOS/EOS don't have word_id
            left_id: 0,
            right_id: 0,
            pos_id: 0,
            wcost: 0,
            feature: "BOS/EOS".to_string(),
            is_unknown: false,
            //
            cost: Cell::new(0),
            lnode_id: Cell::new(None),
            is_best: Cell::new(false),
        }
    }

    /// Create an unknown word node
    pub fn unknown(
        text: &'a str,
        start: usize,
        length: usize,
        entry: &DictionaryEntry,
        feature: String,
    ) -> Self {
        Self {
            surface: &text[start..start + length],
            start,
            end: start + length,
            word_id: entry.word_id,
            left_id: entry.left_id,
            right_id: entry.right_id,
            pos_id: entry.pos_id,
            wcost: entry.wcost,
            feature,
            is_unknown: true,
            //
            cost: Cell::new(0),
            lnode_id: Cell::new(None),
            is_best: Cell::new(false),
        }
    }
}

/// The lattice structure representing all possible segmentations
#[derive(Debug)]
pub struct Lattice<'a> {
    arena: Arena<LatticeNode<'a>>,

    nodes_starting_at: Vec<Vec<Id<LatticeNode<'a>>>>,
    nodes_ending_at: Vec<Vec<Id<LatticeNode<'a>>>>,

    /// Original input text
    pub text: &'a str,
}

impl<'a> Lattice<'a> {
    /// Returns the BOS node.
    pub fn bos_node(&self) -> &LatticeNode<'a> {
        assert!(
            !self.nodes_starting_at.is_empty(),
            "Lattice should be initialized"
        );

        let nodes = &self.nodes_ending_at.first().unwrap();
        let num_nodes = nodes.len();

        assert!(
            num_nodes == 1,
            "Only 1 BOS segment should exist, but found {num_nodes}",
        );

        let node = &self.arena[nodes[0]];

        assert!(
            node.start == node.end,
            "BOS nodes should have the same start/end position"
        );

        node
    }

    /// Returns the EOS node.
    pub fn eos_node(&self) -> &LatticeNode<'a> {
        assert!(
            !self.nodes_starting_at.is_empty(),
            "Lattice should be initialized"
        );

        let nodes = &self.nodes_starting_at.last().unwrap();
        let num_nodes = nodes.len();

        assert!(
            num_nodes == 1,
            "Only 1 EOS segment should exist, but found {num_nodes}",
        );

        let node = &self.arena[nodes[0]];

        assert!(
            node.start == node.end,
            "EOS nodes should have the same start/end position."
        );

        node
    }

    /// Build a lattice from the input text using the dictionary
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to analyze
    /// * `dict` - The dictionary to use for word lookup
    ///
    /// # Errors
    ///
    /// Returns an error if lattice construction fails.
    pub fn build(text: &'a str, dict: &Dictionary) -> Result<Self> {
        let mut arena = Arena::<LatticeNode<'a>>::new();

        let text_len = text.len();

        let mut nodes_starting_at = vec![Vec::new(); text_len + 1];
        let mut nodes_ending_at = vec![Vec::new(); text_len + 1];

        let bos_id = arena.alloc(LatticeNode::bos());
        nodes_ending_at[0].push(bos_id);

        for (char_byte_pos, char) in text.char_indices() {
            let entries = dict.lookup(&text[char_byte_pos..]);

            if entries.is_empty() {
                Self::push_unknown_nodes(
                    text,
                    char_byte_pos,
                    char,
                    dict,
                    &mut arena,
                    &mut nodes_starting_at,
                );
            } else {
                for entry in entries {
                    let node_id = arena.alloc(LatticeNode::from_entry(text, char_byte_pos, entry));
                    let node = &arena[node_id];

                    let end_pos = node.end;

                    if end_pos <= text_len {
                        nodes_starting_at[char_byte_pos].push(node_id);
                    }
                }
            }
        }

        // Add EOS node at the final position
        let eos_id = arena.alloc(LatticeNode::eos(text_len));
        nodes_starting_at[text_len].push(eos_id);

        // TODO:
        // let final_pos = text.len();
        // if nodes[final_pos + 1].is_empty() && !nodes[final_pos].is_empty() {
        //     panic!("Trailing UNK: {nodes:#?}");
        // }

        Ok(Self {
            arena,
            nodes_starting_at,
            nodes_ending_at,
            text,
        })
    }

    /// Add unknown word nodes for a character
    fn push_unknown_nodes(
        text: &'a str,
        pos: usize,
        c: char,
        dict: &Dictionary,
        arena: &mut Arena<LatticeNode<'a>>,
        nodes_starting_at: &mut [Vec<Id<LatticeNode<'a>>>],
    ) {
        let category = dict.char_category(c);

        // Get unknown entries for this category
        let entries = dict.unknown.generate_entries(category, c.len_utf8());

        if entries.is_empty() {
            // Create a default unknown entry if none exist
            let char_len = c.len_utf8();
            let end_pos = pos + char_len;

            if end_pos <= text.len() {
                let node_id = arena.alloc(LatticeNode {
                    surface: &text[pos..end_pos],
                    start: pos + 1,
                    end: end_pos,
                    word_id: u32::MAX, // Unknown words don't have dictionary word_id
                    left_id: 0,
                    right_id: 0,
                    pos_id: 0,
                    wcost: 10000, // High cost for unknown
                    feature: format!("未知語,{category:?}"),
                    is_unknown: true,
                    //
                    cost: Cell::new(0),
                    lnode_id: Cell::new(None),
                    is_best: Cell::new(false),
                });

                nodes_starting_at[pos].push(node_id);
            }
        } else {
            for entry in &entries {
                let feature = entry.feature.clone();

                let node = LatticeNode::unknown(text, pos, entry.length, entry, feature);
                let end_pos = node.end;

                let node_id = arena.alloc(node);

                if end_pos <= text.len() {
                    nodes_starting_at[pos].push(node_id);
                }
            }
        }

        // Handle grouping for consecutive characters of the same category
        if dict.char_def.should_group(category) {
            Self::add_grouped_unknown(text, pos, category, dict, arena, nodes_starting_at);
        }
    }

    /// Add grouped unknown word nodes (consecutive characters of same category)
    fn add_grouped_unknown(
        text: &'a str,
        start: usize,
        category: CharCategory,
        dict: &Dictionary,
        arena: &mut Arena<LatticeNode<'a>>,
        nodes_starting_at: &mut [Vec<Id<LatticeNode<'a>>>],
    ) {
        let remaining = &text[start..];
        let mut length = 0;
        let mut char_count = 0;

        for c in remaining.chars() {
            if dict.char_category(c) != category {
                break;
            }
            length += c.len_utf8();
            char_count += 1;

            // Limit group length
            if char_count > 1 {
                let entries = dict.unknown.generate_entries(category, length);

                for entry in &entries {
                    let feature = entry.feature.clone();

                    let node = LatticeNode::unknown(text, start, length, entry, feature);
                    let node_id = arena.alloc(node);

                    let end_pos = start + length;

                    if end_pos <= text.len() {
                        nodes_starting_at[start].push(node_id);
                    }
                }
            }
        }
    }

    /// Get the number of byte positions in the lattice
    pub fn len(&self) -> usize {
        self.nodes_starting_at.len()
    }

    /// Check if the lattice is empty
    pub fn is_empty(&self) -> bool {
        self.nodes_starting_at.is_empty() && self.nodes_ending_at.is_empty()
    }

    pub fn get(&self, id: Id<LatticeNode<'a>>) -> &LatticeNode<'a> {
        &self.arena[id]
    }

    /// Returns the nodes that begin at the given position.
    pub fn nodes_starting_at(&self, pos: usize) -> Vec<Id<LatticeNode<'a>>> {
        debug_assert!(
            pos < self.nodes_starting_at.len(),
            "Attempted to fetch node at position outside bounds."
        );

        self.nodes_starting_at[pos].clone()
    }

    /// Get nodes ending at a specific position
    pub fn nodes_ending_at(&self, pos: usize) -> Vec<Id<LatticeNode<'a>>> {
        debug_assert!(
            pos < self.nodes_ending_at.len(),
            "Attempted to fetch node at position outside bounds."
        );

        self.nodes_ending_at[pos].clone()
    }

    pub fn nodes_ending_at_mut(&mut self, pos: usize) -> &mut Vec<Id<LatticeNode<'a>>> {
        &mut self.nodes_ending_at[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bos_eos_nodes() {
        let bos = LatticeNode::bos();
        assert_eq!(bos.surface, "");
        assert_eq!(bos.start, 0);
        assert_eq!(bos.end, 0);

        let eos = LatticeNode::eos(10);
        assert_eq!(eos.surface, "");
        assert_eq!(eos.start, 10);
        assert_eq!(eos.end, 10);
    }
}
