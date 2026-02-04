use railgun::error::Error;
use railgun::error::Location;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error)]
pub enum Error {
    #[error(display("Failed to analyze the type of dictionary at '{path}'"))]
    UnknownType { path: String, location: Location },
}

/*
use railgun::error::{Error, Location};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error)]
pub enum Error {
    #[error(transparent)]
    Fs {
        error: std::io::Error,
        location: Location,
    },
}

*/
