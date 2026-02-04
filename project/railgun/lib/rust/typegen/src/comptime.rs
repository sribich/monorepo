use crate::{id::TypeId, typegen::id::SourceLocation};

pub const fn source_location(location: &'static str) -> SourceLocation {
    SourceLocation(location)
}

pub const fn type_id(type_name: &'static str, type_location: &'static str) -> TypeId {
    let hash = fnv64(type_name.as_bytes(), None);
    let hash = fnv64(type_location.as_bytes(), Some(hash));

    TypeId {
        name: type_name,
        hash,
    }
}

pub const fn fnv64(bytes: &[u8], hash: Option<u64>) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
    const FNV_PRIME: u64 = 1_099_511_628_211;

    let mut hash = match hash {
        Some(value) => value,
        None => FNV_OFFSET_BASIS,
    };

    let mut i = 0;

    #[expect(clippy::as_conversions, reason = "Casting to a larger number")]
    #[expect(clippy::indexing_slicing, reason = "Needed in const context")]
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        i += 1;
    }

    hash
}

pub const fn fnv128(bytes: &[u8], hash: Option<u128>) -> u128 {
    const FNV_OFFSET_BASIS: u128 = 144_066_263_297_769_815_596_495_629_667_062_367_629;
    const FNV_PRIME: u128 = 309_485_009_821_345_068_724_781_371;

    let mut hash = match hash {
        Some(value) => value,
        None => FNV_OFFSET_BASIS,
    };

    let mut i = 0;

    #[expect(clippy::as_conversions, reason = "Casting to a larger number")]
    #[expect(clippy::indexing_slicing, reason = "Needed in const context")]
    while i < bytes.len() {
        hash ^= bytes[i] as u128;
        hash = hash.wrapping_mul(FNV_PRIME);

        i += 1;
    }

    hash
}
