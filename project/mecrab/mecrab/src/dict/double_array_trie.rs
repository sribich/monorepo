use std::fmt::Debug;

use railgun_error::ensure;

use crate::error::CorruptContext;
use crate::error::IncorrectSizeContext;
use crate::error::InvalidAlignmentContext;
use crate::error::ParseError;

/// Result from a trie lookup containing value and matched length
#[derive(Debug, Clone, Copy, Default)]
pub struct DartsResult {
    /// The value stored in the trie (-1 if not found)
    pub value: i32,
    /// Length of the matched key in bytes
    pub length: usize,
}

/// Double-Array Trie unit structure
/// Each unit is 8 bytes: base (i32) + check (u32)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Unit {
    /// Base value for state transitions
    /// Negative value indicates a leaf with value = -base - 1
    base: i32,
    /// Check value to validate transitions
    check: u32,
}

/// Double-Array Trie for fast word lookup (Darts compatible)
pub struct DoubleArrayTrie<'a> {
    /// Raw pointer to memory-mapped data
    units: &'a [Unit],
    /// Number of units
    size: usize,
}

impl<'a> Debug for DoubleArrayTrie<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoubleArrayTrie")
            .field("size_in_bytes", &(self.size * std::mem::size_of::<Unit>()))
            .field("num_units", &self.size)
            .finish()
    }
}

impl<'a> DoubleArrayTrie<'a> {
    /// Creates a Double-Array Trie from raw bytes.
    pub fn from_bytes(data: &'a [u8], size_in_bytes: usize) -> Result<Self, ParseError> {
        ensure!(
            data.len() == size_in_bytes,
            IncorrectSizeContext {
                expected_size: size_in_bytes,
                actual_size: data.len()
            }
        );

        ensure!(
            size_in_bytes.is_multiple_of(std::mem::size_of::<Unit>()),
            CorruptContext {
                reason: format!(
                    "DAT size '{}' is not a multiple of Unit '{}'",
                    size_in_bytes,
                    std::mem::size_of::<Unit>()
                )
            }
        );

        let units_ptr = data.as_ptr() as *const Unit;
        ensure!(units_ptr.is_aligned(), InvalidAlignmentContext {});

        let size = size_in_bytes / std::mem::size_of::<Unit>();

        // SAFETY: The pointer has been checked for alignment and `size` has
        //         been verified to be a divisor of `data.len()`, which is
        //         what are casting into.
        let units = unsafe { std::slice::from_raw_parts(units_ptr, size) };

        Ok(Self { units, size })
    }

    /// Attempts to find an exact match for the given `key`.
    pub fn exact_match_search(&self, key: &[u8]) -> Option<DartsResult> {
        let mut b = match self.get(0) {
            Some(unit) => unit.base,
            None => return None,
        };

        for &byte in key.iter() {
            let p = (b as usize).wrapping_add(byte as usize).wrapping_add(1);

            match self.get(p) {
                Some(unit) if b as u32 == unit.check => {
                    b = unit.base;
                }
                _ => return None,
            }
        }

        let p = b as usize;

        if let Some(unit) = self.get(p)
            && unit.check == b as u32
            && unit.base < 0
        {
            return Some(DartsResult {
                value: -unit.base - 1,
                length: key.len(),
            });
        }

        None
    }

    /// Finds all values that share the given prefix, returning the number of
    /// prefixes found.
    ///
    /// The found values will be inserted into `results`. If `results` would
    /// overflow due to an added prefix, it will be silently skipped. The total
    /// number of prefixes found, including skipped values, will be returned.
    pub fn common_prefix_search(&self, key: &[u8], results: &mut [DartsResult]) -> usize {
        let max_results = results.len();
        let mut num_results = 0;

        let mut b = match self.get(0) {
            Some(unit) => unit.base,
            None => return 0,
        };

        for (byte_pos, &byte) in key.iter().enumerate() {
            let p = b as usize;

            if let Some(unit) = self.get(p)
                && b as u32 == unit.check
                && unit.base < 0
            {
                if num_results < max_results {
                    results[num_results] = DartsResult {
                        value: -unit.base - 1,
                        length: byte_pos,
                    };
                }

                num_results += 1;
            }

            let p = (b as usize).wrapping_add(byte as usize).wrapping_add(1);

            match self.get(p) {
                Some(unit) if b as u32 == unit.check => {
                    b = unit.base;
                }
                _ => return num_results,
            }
        }

        let p = b as usize;

        if let Some(unit) = self.get(p)
            && b as u32 == unit.check
            && unit.base < 0
        {
            if num_results < max_results {
                results[num_results] = DartsResult {
                    value: -unit.base - 1,
                    length: key.len(),
                };
            }

            num_results += 1;
        }

        num_results
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&Unit> {
        self.units.get(index)
    }
}
