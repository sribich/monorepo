//! Viterbi algorithm implementation for finding the optimal path
//!
//! Copyright 2026 COOLJAPAN OU (Team KitaSan)
//!
//! The Viterbi algorithm finds the path through the lattice with the
//! minimum total cost (word costs + connection costs).

pub mod analysis;
pub mod nbest;
#[cfg(feature = "simd")]
pub mod simd;

use std::cell::Cell;

pub use analysis::ConnectionMatrixStats;
pub use analysis::LatticeStats;
pub use analysis::MorphemeCost;
pub use analysis::PathAnalysis;
pub use analysis::PathComparison;
pub use analysis::SegmentationReport;
use id_arena::Id;
pub use nbest::NBestIter;
pub use nbest::NBestSearch;
pub use nbest::PathDiversity;
pub use nbest::ScoredPath;

use crate::Error;
use crate::Result;
use crate::dict::Dictionary;
use crate::lattice::Lattice;
use crate::lattice::LatticeNode;

/// Result node after Viterbi path finding
#[derive(Debug, Clone)]
pub struct PathNode {
    /// Surface form
    pub surface: String,
    /// Word ID (token index in dictionary, used for embeddings)
    pub word_id: u32,
    /// Part-of-speech ID
    pub pos_id: u16,
    /// Word cost
    pub wcost: i16,
    /// Feature string
    pub feature: String,
}

/// Entry in the Viterbi table
#[derive(Debug, Clone)]
struct ViterbiEntry<'a> {
    /// The lattice node
    node: &'a LatticeNode<'a>,
    /// Cumulative cost to reach this node
    cost: i64,
    /// Back pointer to the previous entry
    prev: Option<usize>,
    /// Position in the nodes_at vector for backtracking
    pos: usize,
}

/// Viterbi solver for finding the optimal path through the lattice
pub struct ViterbiSolver<'a> {
    dictionary: &'a Dictionary,
}

impl<'a> ViterbiSolver<'a> {
    /// Create a new Viterbi solver
    pub const fn new(dictionary: &'a Dictionary) -> Self {
        Self { dictionary }
    }

    /// Solve the lattice and return the optimal path
    ///
    /// # Arguments
    ///
    /// * `lattice` - The lattice to solve
    ///
    /// # Errors
    ///
    /// Returns an error if no valid path is found.
    pub fn solve<'b>(&self, mut lattice: Lattice<'b>) -> Result<(Lattice<'b>, Vec<PathNode>)> {
        if lattice.is_empty() {
            return Err(Error::LatticeError("Empty lattice".to_string()));
        }

        self.forward_pass(&mut lattice);

        let path = Self::backward_pass(&mut lattice)?;

        Ok((lattice, path))
    }

    /*
    /// Solve the lattice and return the N-best paths
    ///
    /// # Arguments
    ///
    /// * `lattice` - The lattice to solve
    /// * `n` - Number of best paths to return
    ///
    /// # Errors
    ///
    /// Returns an error if no valid path is found.
    pub fn solve_nbest<'b>(
        &self,
        lattice: &'b mut Lattice<'b>,
        n: usize,
    ) -> Result<Vec<(Vec<PathNode>, i64)>> {
        if lattice.is_empty() {
            return Err(Error::LatticeError("Empty lattice".to_string()));
        }

        // Forward pass to compute cumulative costs
        let entries = self.forward_pass(lattice);

        // Build the N-best paths using backward search
        let paths = self.nbest_backward(&entries, lattice, n);

        Ok(paths)
    }

    /// Find N-best paths using backward search through the entries
    fn nbest_backward<'b>(
        &self,
        entries: &[Vec<ViterbiEntry<'b>>],
        lattice: &'b Lattice<'b>,
        n: usize,
    ) -> Vec<(Vec<PathNode>, i64)> {
        use std::collections::BinaryHeap;

        let len = lattice.len();
        if len == 0 || entries.is_empty() {
            return vec![];
        }

        // State for search: (cost, position, entry_index, partial_path)
        #[derive(Clone)]
        struct SearchState<'a> {
            cost: i64,
            pos: usize,
            idx: usize,
            path: Vec<&'a LatticeNode<'a>>,
        }

        impl PartialEq for SearchState<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost
            }
        }
        impl Eq for SearchState<'_> {}
        impl PartialOrd for SearchState<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for SearchState<'_> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                // Min-heap: reverse ordering
                other.cost.cmp(&self.cost)
            }
        }

        let mut heap: BinaryHeap<SearchState<'b>> = BinaryHeap::new();
        let mut results = Vec::with_capacity(n);

        // Start from EOS entries
        let eos_pos = len - 1;
        for (idx, entry) in entries[eos_pos].iter().enumerate() {
            heap.push(SearchState {
                cost: entry.cost,
                pos: eos_pos,
                idx,
                path: vec![entry.node],
            });
        }

        while let Some(state) = heap.pop() {
            if results.len() >= n {
                break;
            }

            let entry = &entries[state.pos][state.idx];

            // Check if we've reached BOS
            if entry.prev.is_none() {
                // Construct path from nodes (skip BOS/EOS)
                let path_nodes: Vec<PathNode> = state
                    .path
                    .iter()
                    .rev()
                    .filter(|node| !node.surface.is_empty())
                    .map(|node| PathNode {
                        surface: node.surface.to_string(),
                        word_id: node.word_id,
                        pos_id: node.pos_id,
                        wcost: node.wcost,
                        feature: node.feature.clone(),
                    })
                    .collect();

                if !path_nodes.is_empty() || state.path.len() <= 2 {
                    results.push((path_nodes, state.cost));
                }
                continue;
            }

            // Expand to predecessor
            let prev_idx = entry.prev.unwrap();
            let prev_pos = entry.pos;

            if prev_pos < entries.len() && prev_idx < entries[prev_pos].len() {
                let prev_entry = &entries[prev_pos][prev_idx];
                let mut new_path = state.path.clone();
                new_path.push(prev_entry.node);

                heap.push(SearchState {
                    cost: state.cost,
                    pos: prev_pos,
                    idx: prev_idx,
                    path: new_path,
                });
            }
        }

        results
    }
    */

    /// Forward pass of the Viterbi algorithm
    fn forward_pass<'b>(&self, lattice: &mut Lattice<'b>) {
        assert!(lattice.len() == lattice.text.len() + 1);

        let lattice_len = lattice.len();

        for pos in 0..lattice_len {
            for right_node_id in lattice.nodes_starting_at(pos) {
                let right_node = lattice.get(right_node_id);

                let mut best_lnode_cost = i64::MAX;
                let mut best_lnode_id: Option<Id<LatticeNode<'b>>> = None;

                for left_node_id in lattice.nodes_ending_at(pos) {
                    let left_node = lattice.get(left_node_id);

                    let local_cost = self.dictionary.connection_cost(left_node, right_node);
                    let total_cost = local_cost as i64 + left_node.cost.get();

                    if total_cost < best_lnode_cost {
                        best_lnode_cost = total_cost;
                        best_lnode_id = Some(left_node_id);
                    }
                }

                if let Some(best_lnode_id) = best_lnode_id {
                    right_node.cost.swap(&Cell::new(best_lnode_cost));
                    right_node.lnode_id.swap(&Cell::new(Some(best_lnode_id)));

                    // TODO: Support the rnode->rlength + pos which accounts for whitespace on node end position

                    let nodes_ending_at = lattice.nodes_ending_at_mut(right_node.end);
                    nodes_ending_at.push(right_node_id);
                }
            }
        }
    }

    /// Backward pass: trace the optimal path
    fn backward_pass<'b>(lattice: &mut Lattice<'b>) -> Result<Vec<PathNode>> {
        let mut node = lattice.eos_node();

        let mut result = vec![];

        loop {
            let prev_node_id = if let Some(prev_node_id) = node.lnode_id.get() {
                prev_node_id
            } else {
                break;
            };

            let prev_node = lattice.get(prev_node_id);
            prev_node.is_best.swap(&Cell::new(true));

            node = prev_node;

            result.push(PathNode {
                surface: node.surface.to_owned(),
                word_id: node.word_id,
                pos_id: node.pos_id,
                wcost: node.wcost,
                feature: node.feature.clone(),
            });
        }

        result.reverse();
        /*
        // Load the EOS Segment

        // Find the best EOS entry
        let eos_entries = &entries[lattice.len() - 1];

        if eos_entries.is_empty() {
            return Err(Error::ViterbiError("No path to EOS found".to_string()));
        }

        let best_eos = eos_entries
            .iter()
            .min_by_key(|e| e.cost)
            .ok_or_else(|| Error::ViterbiError("No EOS entry found".to_string()))?;

        let mut path = vec![];
        let mut current_idx = best_eos.prev;
        let mut prev_pos = best_eos.pos;

        if !best_eos.node.surface.is_empty() {
            path.push(PathNode {
                surface: best_eos.node.surface.to_string(),
                word_id: best_eos.node.word_id,
                pos_id: best_eos.node.pos_id,
                wcost: best_eos.node.wcost,
                feature: best_eos.node.feature.clone(),
            });
        }

        while let Some(idx) = current_idx {
            if prev_pos > entries.len() || idx > entries[prev_pos].len() {
                break;
            }

            let entry = &entries[prev_pos][idx];

            // Skip BOS and EOS
            if !entry.node.surface.is_empty() {
                path.push(PathNode {
                    surface: entry.node.surface.to_string(),
                    word_id: entry.node.word_id,
                    pos_id: entry.node.pos_id,
                    wcost: entry.node.wcost,
                    feature: entry.node.feature.clone(),
                });
            }

            current_idx = entry.prev;
            prev_pos = entry.pos;
        }

        path.reverse();
        Ok(path)
         */

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_node_creation() {
        let node = PathNode {
            surface: "テスト".to_string(),
            word_id: 42,
            pos_id: 1,
            wcost: 100,
            feature: "名詞,一般".to_string(),
        };

        assert_eq!(node.surface, "テスト");
        assert_eq!(node.pos_id, 1);
    }
}
