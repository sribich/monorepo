use std::num::ParseIntError;

use railgun::error::Error;
use railgun::error::Location;

pub type Result<T> = core::result::Result<T, LexoRankError>;

#[derive(Error)]
pub enum LexoRankError {
    #[error(display("Unable to parse number '{number}' into base36."))]
    InvalidNumber {
        number: u64,
        error: ParseIntError,
        location: Location,
    },
    #[error(display("LexoRank only supports between 4-13 bytes, '{bytes}' were requested."))]
    ByteRange { bytes: u8, location: Location },

    #[error(display("Too many slots ({slots}) for a LexoRank size of {bytes}."))]
    TooManySlots {
        slots: u32,
        bytes: u8,
        location: Location,
    },
}
