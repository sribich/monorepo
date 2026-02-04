//! A rust implementation of [uuidv47](https://github.com/stateless-me/uuidv47).
//!
//! The goal of this library is to prevent leaking the timestamp component
//! of a uuidv7 while maintaining its ability to improve database performance
//! due to its monotonic nature.
//!
//! This is done by SipHashing the random bytes of a uuidv7 and XORing it with
//! the timestamp to mask it as a uuidv4. This process is deterministic and can
//! be reversed. The masked uuid can be presented to clients without leaking the
//! timestamp.
//!
//! # Example
//!
//! ```rust
//! use uuid47::BlindUuid;
//! use uuid47::SipHashKey;
//!
//! let key = SipHashKey::new(0x0123_4567_89ab_cdef_u64, 0xfedc_ba98_7654_3210_u64);
//!
//! let uuid_v7 = BlindUuid::new();
//!
//! // The facade returns a string to prevent us from accidentally using the
//! // facade internally.
//! let facade: String = uuid_v7.to_facade(&key);
//!
//! let uuid_v7 = BlindUuid::from_facade(facade, &key);
//! ```
//!
//! # Notes
//!
//!  UUIDv7 generator based on the RFC4122 update proposal (draft-07).
//!
//!    https://datatracker.ietf.org/doc/draft-ietf-uuidrev-rfc4122bis/07/
//!
//!  - unix_ts_ms: Milliseconds elapsed since the Unix epoch – 48 bit big-endian unsigned number
//!  - ver: UUID version (7) – 4 bits
//!  - rand_a: Monotonic sequence counter for more precise sorting – 12 bits
//!  - var: UUID variant (0b10) – 2 bits
//!  - rand_b: Cryptographically strong random data – 62 bits
//!
//!   1               2               3               4
//!   0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
//!  ┌─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┐
//!  │                          unix_ts_ms                           │
//!  ├─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┼─┴─┴─┴─┼─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┤
//!  │          unix_ts_ms           │  ver  │        rand_a         │
//!  ├─┴─┼─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┤
//!  │var│                        rand_b                             │
//!  ├─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┤
//!  │                            rand_b                             │
//!  └─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
mod key;

use std::str::FromStr;

pub use key::SipHashKey;
use siphasher::sip::SipHasher24;
use uuid::Uuid;

pub struct BlindUuid(Uuid);

impl Default for BlindUuid {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for BlindUuid {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl BlindUuid {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn to_facade(&self, key: &SipHashKey) -> String {
        let bytes = self.0.as_bytes();

        let (k1, k2) = key.get();

        let hasher = SipHasher24::new_with_keys(k1, k2);

        let timestamp = mask_unmask_timestamp(bytes, &hasher);
        let output = recode_bytes(bytes, timestamp, 4);

        Uuid::from_bytes(output).to_string()
    }

    pub fn from_facade<S: AsRef<str>>(facade: S, key: &SipHashKey) -> Self {
        let bytes = *Uuid::from_str(facade.as_ref()).unwrap().as_bytes();

        let (k1, k2) = key.get();

        let hasher = SipHasher24::new_with_keys(k1, k2);

        let timestamp = mask_unmask_timestamp(&bytes, &hasher);
        let output = recode_bytes(&bytes, timestamp, 7);

        Self(Uuid::from_bytes(output))
    }
}

fn recode_bytes(bytes: &[u8; 16], timestamp: u64, version: u8) -> [u8; 16] {
    let mut bytes = *bytes;

    bytes[0..6].copy_from_slice(&u64_to_timestamp(timestamp));

    set_version(&mut bytes, version);
    set_variant(&mut bytes);

    bytes
}

///
fn mask_unmask_timestamp(bytes: &[u8; 16], hasher: &SipHasher24) -> u64 {
    let input = random_bytes(bytes);

    let mask = hasher.hash(&input) & 0x0000_FFFF_FFFF_FFFF;
    let timestamp = timestamp_to_u64(bytes);

    timestamp ^ mask
}

/// Extracts the random bytes from a UUID.
///
/// Byte structure is maintained by zeroing out the ver and var fields.
fn random_bytes(bytes: &[u8; 16]) -> [u8; 10] {
    let mut random_bytes = [0; 10];

    random_bytes[0] = bytes[6] & 0x0F;
    random_bytes[1] = bytes[7];
    random_bytes[2] = bytes[8] & 0x3F;

    random_bytes[3..10].copy_from_slice(&bytes[9..16]);

    random_bytes
}

fn timestamp_to_u64(bytes: &[u8; 16]) -> u64 {
    let mut out = [0_u8; 8];
    out[2..].copy_from_slice(&bytes[0..6]);

    #[expect(clippy::big_endian_bytes, reason = "UUID v7 timestamp is big endian.")]
    u64::from_be_bytes(out)
}

fn u64_to_timestamp(value: u64) -> [u8; 6] {
    let mut out = [0_u8; 6];

    #[expect(clippy::big_endian_bytes, reason = "UUID v7 timestamp is big endian.")]
    out.copy_from_slice(&value.to_be_bytes()[2..]);
    out
}

fn set_version(bytes: &mut [u8; 16], version: u8) {
    bytes[6] = (bytes[6] & 0x0F) | ((version & 0x0F) << 4_u8);
}

fn set_variant(bytes: &mut [u8; 16]) {
    bytes[8] = (bytes[8] & 0x3F) | 0x80;
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::BlindUuid;
    use crate::SipHashKey;

    #[test]
    fn known_value_works() {
        let key = SipHashKey::new(0x0123_4567_89ab_cdef_u64, 0xfedc_ba98_7654_3210_u64);

        let start = BlindUuid::from_str("00000000-0000-7000-8000-000000000000").unwrap();
        assert_eq!(
            start.to_facade(&key),
            "22d97126-9609-4000-8000-000000000000"
        );

        let end = BlindUuid::from_facade("22d97126-9609-4000-8000-000000000000", &key);
        assert_eq!(end.0.to_string(), "00000000-0000-7000-8000-000000000000");
    }

    #[test]
    fn roundtrip_works() {
        let key = SipHashKey::new(0x0123_4567_89ab_cdef_u64, 0xfedc_ba98_7654_3210_u64);

        let start = BlindUuid::new();
        let facade = start.to_facade(&key);

        assert_eq!(
            start.0.to_string(),
            BlindUuid::from_facade(facade, &key).0.to_string()
        );
    }
}
