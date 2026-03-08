//! Lattice module for building the word graph
//!
//! Copyright 2026 COOLJAPAN OU (Team KitaSan)
//!
//! The lattice represents all possible segmentations of the input text
//! as a directed acyclic graph (DAG). Each node represents a potential
//! word, and edges connect adjacent words.

mod visualize;

pub use visualize::DotBuilder;
pub use visualize::DotConfig;
pub use visualize::NodeShape;
pub use visualize::RankDir;

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
}

impl<'a> LatticeNode<'a> {
    /// Create a BOS (Beginning of Sentence) node
    pub fn bos() -> Self {
        Self {
            surface: "BOS",
            start: 0,
            end: 0,
            word_id: u32::MAX, // BOS/EOS don't have word_id
            left_id: 0,
            right_id: 0,
            pos_id: 0,
            wcost: 0,
            feature: "BOS/EOS".to_string(),
            is_unknown: false,
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
        }
    }
}

/// The lattice structure representing all possible segmentations
#[derive(Debug)]
pub struct Lattice<'a> {
    /// Original input text
    pub text: &'a str,
    /// Nodes at each byte position
    /// Index 0 contains BOS, last index contains EOS

    /// The list of notes, ordered by byte position.
    pub nodes: Vec<Vec<LatticeNode<'a>>>,
}

impl<'a> Lattice<'a> {
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
        let text_len = text.len();

        let mut nodes: Vec<Vec<LatticeNode<'a>>> = vec![Vec::new(); text_len + 2];

        nodes[0].push(LatticeNode::bos());

        // Build lattice by scanning through text
        for (char_byte_pos, c) in text.char_indices() {
            let subtext = &text[char_byte_pos..];

            let entries = dict.lookup(subtext);

            if entries.is_empty() {
                Self::push_unknown_nodes(text, char_byte_pos, c, dict, &mut nodes);
            } else {
                for entry in entries {
                    let node = LatticeNode::from_entry(text, char_byte_pos, entry);
                    let end_pos = node.end;

                    // Add node to the end position + 1 (shifted for BOS)
                    if end_pos <= text_len {
                        nodes[end_pos].push(node);
                    }
                }
            }
        }

        // Add EOS node at the final position
        nodes[text_len + 1].push(LatticeNode::eos(text_len));

        // Handle case where no nodes reach the end
        // This can happen with unknown characters at the end
        let final_pos = text.len();
        if nodes[final_pos + 1].is_empty() && !nodes[final_pos].is_empty() {
            println!("{:#?}", nodes);
            panic!("trailing");
            // Check if we need to handle trailing characters
        }

        Ok(Self { text, nodes })
    }

    /// Add unknown word nodes for a character
    fn push_unknown_nodes(
        text: &'a str,
        pos: usize,
        c: char,
        dict: &Dictionary,
        nodes_at: &mut [Vec<LatticeNode<'a>>],
    ) {
        let category = dict.char_category(c);

        // Get unknown entries for this category
        let entries = dict.unknown.generate_entries(category, c.len_utf8());

        if entries.is_empty() {
            // Create a default unknown entry if none exist
            let char_len = c.len_utf8();
            let end_pos = pos + char_len;

            if end_pos <= text.len() {
                nodes_at[end_pos].push(LatticeNode {
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
                });
            }
        } else {
            for entry in &entries {
                let feature = entry.feature.clone();

                let node = LatticeNode::unknown(text, pos, entry.length, entry, feature);
                let end_pos = node.end;

                if end_pos <= text.len() {
                    nodes_at[end_pos].push(node);
                }
            }
        }

        // Handle grouping for consecutive characters of the same category
        if dict.char_def.should_group(category) {
            Self::add_grouped_unknown(text, pos, category, dict, nodes_at);
        }
    }

    /// Add grouped unknown word nodes (consecutive characters of same category)
    fn add_grouped_unknown(
        text: &'a str,
        start: usize,
        category: CharCategory,
        dict: &Dictionary,
        nodes_at: &mut [Vec<LatticeNode<'a>>],
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
                    let end_pos = start + length;

                    if end_pos <= text.len()
                        && !nodes_at[end_pos + 1]
                            .iter()
                            .any(|n| n.start == start && n.end == end_pos)
                    {
                        nodes_at[end_pos + 1].push(node);
                    }
                }
            }
        }
    }

    /// Get the number of byte positions in the lattice
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the lattice is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the nodes that begin at the given position.
    pub fn get(&self, pos: usize) -> &[LatticeNode<'a>] {
        debug_assert!(
            pos < self.nodes.len(),
            "Attempted to fetch node at position outside bounds."
        );

        &self.nodes[pos]
    }

    /// Get nodes ending at a specific position
    pub fn nodes_ending_at(&self, pos: usize) -> &[LatticeNode<'a>] {
        if pos < self.nodes.len() {
            &self.nodes[pos]
        } else {
            &[]
        }
    }

    pub fn get_bos_node(&self) -> &[LatticeNode<'a>] {
        let nodes = self.get(0);
        let num_nodes = nodes.len();

        assert!(
            num_nodes == 1,
            "Only 1 BOS segment should exist, but found {num_nodes}",
        );

        for node in nodes {
            assert!(
                node.start == node.end,
                "BOS nodes should have the same start/end position.",
            );
        }

        nodes
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
