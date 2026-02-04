//! Lexorank is an algorithm used
//!
//! 3 buckets, 0 1 2.
//!   Only 1 bucket is active at a time.
//!   During a rebalancing operation, a new bucket will be used
//!
//!   0 -> 1 rebalance from bottom of the list (highest number)
//!   1 -> 2 rebalance from bottom of the list (highest number)
//!   2 -> 0 rebalance from top of the list (lowest number)
//!
//! Marker row (type 0 min, type 1 normal, type 1 max)
mod error;
mod radix;

use core::future::Future;

use error::ByteRangeContext;
use error::InvalidNumberContext;
use error::Result;
use error::TooManySlotsContext;
use radix::Base36;
use railgun::error::ResultExt;

/*
let a = LexoRank::new(Bucket::new(0).unwrap(), Rank::new("a").unwrap());
let b = LexoRank::new(Bucket::new(0).unwrap(), Rank::new("b").unwrap());

println!("{:#?}", a);
println!("{:#?}", b);
println!("{:#?}", a.between(&a.between(&b).unwrap()));
 */
pub trait Rankable {
    fn first(&self) -> impl Future<Output = Option<String>> + Send;
    fn last(&self) -> impl Future<Output = Option<String>> + Send;

    // pub fn rebalance(&self);
}

pub struct LexoRank {
    bytes: u8,
    marker: Marker,
}

impl Default for LexoRank {
    fn default() -> Self {
        Self::new(12, u32::MAX).unwrap()
    }
}

/// TODO: We want to trigger a rebalance when there is fewer than 8 slots left
///       to sort by. By default there are 19.
impl LexoRank {
    fn new(bytes: u8, slots: u32) -> Result<Self> {
        if !(4..=12).contains(&bytes) {
            return ByteRangeContext { bytes }.fail();
        }

        if u64::from(slots) > 36_u64.pow(u32::from(bytes)) {
            return TooManySlotsContext { slots, bytes }.fail();
        }

        Ok(Self {
            bytes,
            marker: Marker::new(bytes, slots)?,
        })
    }

    pub async fn prev(&self, ranker: impl Rankable) -> Rank {
        if let Some(rank) = ranker.first().await {
            let mut rank = Rank::from_str(&rank);
            rank.rank = self.marker.prev(rank.rank).unwrap();
            rank
        } else {
            Rank {
                bucket: 0,
                rank: self.marker.first(),
            }
        }
    }

    pub async fn next(&self, ranker: impl Rankable) -> Rank {
        if let Some(rank) = ranker.last().await {
            let mut rank = Rank::from_str(&rank);
            rank.rank = self.marker.next(rank.rank).unwrap();
            rank
        } else {
            Rank {
                bucket: 0,
                rank: self.marker.first(),
            }
        }
    }

    fn rebalance() {}
}

pub struct Rank {
    bucket: u8,
    rank: u64,
}

impl Rank {
    pub fn from_str<S: AsRef<str>>(data: S) -> Self {
        let value = data.as_ref();
        let value = value.split('|').collect::<Vec<_>>();

        assert!(value.len() == 2, "LexoRank value is not 2 parts");

        let bucket = u8::from_str_radix(value[0], 10).unwrap();
        let rank = u64::from_str_radix(value[1], 36).unwrap();

        Rank { bucket, rank }
    }

    pub fn to_string(&self) -> String {
        format!("{}|{}", self.bucket, Base36::new(self.rank))
    }
}

struct Marker {
    min: u64,
    max: u64,
    slot_size: u64,
}

impl Marker {
    fn new(bytes: u8, slots: u32) -> Result<Self> {
        let min = u64::from_str_radix(&"0".repeat(bytes as usize), 36)
            .context(InvalidNumberContext { number: 0_u64 })?;
        let max = u64::from_str_radix(&"z".repeat(bytes as usize), 36)
            .context(InvalidNumberContext { number: 0_u64 })?;

        let slot_size = max / u64::from(slots);

        Ok(Self {
            min,
            max,
            slot_size,
        })
    }

    fn first(&self) -> u64 {
        u64::midpoint(self.min, self.max)
    }

    fn prev(&self, from: u64) -> Option<u64> {
        let prev = from - self.slot_size;

        /// We've exhausted the amount of default slots, we must fall back
        /// to midpoint ranking.
        if prev <= self.min {
            Self::middlepoint(from, self.min)
        } else {
            Some(prev)
        }
    }

    fn next(&self, from: u64) -> Option<u64> {
        let next = from + self.slot_size;

        /// We've exhausted the amount of default slots, we must fall back
        /// to midpoint ranking.
        if next >= self.max {
            Self::middlepoint(from, self.max)
        } else {
            Some(next)
        }
    }

    fn middlepoint(left: u64, right: u64) -> Option<u64> {
        let difference = right - left;

        if difference < 2 {
            None
        } else {
            Some(left + difference / 2)
        }
    }
}

pub trait Ranked {
    /// Returns the rank for this item.
    fn rank(&self) -> Rank;
}
