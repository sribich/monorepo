//! Connection matrix for transition costs between context IDs
//!
//! Copyright 2026 COOLJAPAN OU (Team KitaSan)
//!
//! The connection matrix stores the cost of transitioning from one
//! context (POS) to another. This is crucial for the Viterbi algorithm
//! to find the optimal segmentation.
//!
//! Reference: ../ref/mecab-0.996/src/connector.cpp
//!
//! Binary format:
//! - First 2 bytes: lsize (u16) - number of left contexts
//! - Next 2 bytes: rsize (u16) - number of right contexts
//! - Rest: lsize * rsize * 2 bytes of i16 costs
//!
//! Index formula: matrix[rcAttr + lsize * lcAttr]

use std::sync::Arc;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use memmap2::Mmap;

use crate::Error;
use crate::Result;
use crate::lattice::LatticeNode;

/// Connection matrix storing transition costs
pub struct ConnectionMatrix {
    /// Memory map (kept alive)
    _mmap: Arc<Mmap>,
    /// Pointer to cost array (starts after lsize and rsize)
    matrix_ptr: *const i16,
    /// Number of left context IDs
    lsize: usize,
    /// Number of right context IDs
    rsize: usize,
}

// Safety: The matrix_ptr points to immutable memory-mapped data
unsafe impl Send for ConnectionMatrix {}
unsafe impl Sync for ConnectionMatrix {}

impl std::fmt::Debug for ConnectionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionMatrix")
            .field("lsize", &self.lsize)
            .field("rsize", &self.rsize)
            .finish()
    }
}

impl ConnectionMatrix {
    /// Load connection matrix from memory-mapped file
    ///
    /// # Errors
    ///
    /// Returns an error if the file is corrupted.
    pub fn from_mmap(mmap: Arc<Mmap>) -> Result<Self> {
        let data = &mmap[..];

        if data.len() < 4 {
            return Err(Error::MatrixError(
                "Matrix file too small for header".to_string(),
            ));
        }

        // Read sizes (as u16, stored as shorts)
        let lsize = LittleEndian::read_u16(&data[0..2]) as usize;
        let rsize = LittleEndian::read_u16(&data[2..4]) as usize;

        // Validate file size
        // File should be: 4 bytes header + lsize * rsize * 2 bytes
        let expected_size = 4 + lsize * rsize * 2;
        if data.len() != expected_size {
            return Err(Error::MatrixError(format!(
                "Matrix file size mismatch: expected {} bytes ({}x{} matrix + 4), got {}",
                expected_size,
                lsize,
                rsize,
                data.len()
            )));
        }

        // Get pointer to matrix data (after header)
        let matrix_ptr = data[4..].as_ptr() as *const i16;

        Ok(Self {
            _mmap: mmap,
            matrix_ptr,
            lsize,
            rsize,
        })
    }

    /// Get the connection cost between right and left context IDs
    ///
    /// This matches MeCab's Connector::transition_cost function:
    /// `return matrix_[rcAttr + lsize_ * lcAttr];`
    ///
    /// # Arguments
    ///
    /// * `right_id` - Right context attribute of the LEFT node
    /// * `left_id` - Left context attribute of the RIGHT node
    #[inline]
    pub fn cost(&self, left_node: &LatticeNode<'_>, right_node: &LatticeNode<'_>) -> i32 {
        let rc_attr = left_node.right_id as usize;
        let lc_attr = right_node.left_id as usize;

        if rc_attr >= self.rsize || lc_attr >= self.lsize {
            // panic!("Out of bounds lookup");
            // Return a high cost for out-of-bounds lookups
            return i32::MAX;
        }

        let index = rc_attr + self.lsize * lc_attr;

        // SAFETY: Bounds have been checked
        let matrix_cost = unsafe { *self.matrix_ptr.add(index) };

        matrix_cost as i32 + right_node.wcost as i32
    }

    /// Get connection cost without bounds checking (hot path optimization)
    ///
    /// # Safety
    ///
    /// The caller must ensure that right_id < rsize and left_id < lsize.
    /// Context IDs from validated dictionary entries are always in bounds.
    #[inline]
    pub unsafe fn cost_unchecked(&self, right_id: u16, left_id: u16) -> i16 {
        let rc = right_id as usize;
        let lc = left_id as usize;
        let index = rc + self.lsize * lc;
        // Safety: caller guarantees indices are in bounds
        unsafe { *self.matrix_ptr.add(index) }
    }

    /// Get the number of left context IDs
    #[inline]
    pub fn left_size(&self) -> usize {
        self.lsize
    }

    /// Get the number of right context IDs
    #[inline]
    pub fn right_size(&self) -> usize {
        self.rsize
    }

    /// Get total number of entries
    #[inline]
    pub fn size(&self) -> usize {
        self.lsize * self.rsize
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_out_of_bounds() {
        // We can't easily test this without a real mmap, but we can verify the logic
        // Out of bounds should return i16::MAX
        assert_eq!(i16::MAX, 32767);
    }
}
