//! Error types for MeCrab
//!
//! Copyright 2026 COOLJAPAN OU (Team KitaSan)

use std::io;
use std::path::PathBuf;

use railgun_error::Location;
use thiserror::Error;

#[derive(railgun_error::Error)]
pub enum ParseError {
    #[error(display("Dictionary could not be found at '{path:?}'"))]
    NotFound { path: PathBuf, location: Location },
    #[error(display(
        "Dictionary file too small. Expected a minimum of '{min_bytes}' bytes, got '{actual_bytes}' bytes"
    ))]
    FileTooSmall {
        min_bytes: usize,
        actual_bytes: usize,
        location: Location,
    },
    #[error(display(
        "Incorrect dictionary size. Expected '{expected_size}' bytes, got '{actual_size}' bytes"
    ))]
    IncorrectSize {
        expected_size: usize,
        actual_size: usize,
        location: Location,
    },
    #[error(display(
        "Incompatible dictionary version. Expected version '{expected_version}', got '{actual_version}'"
    ))]
    IncorrectVersion {
        expected_version: usize,
        actual_version: usize,
        location: Location,
    },
    #[error(display("Incorrect dictionary type. Expected '{expected_type}', got '{actual_type}'"))]
    IncorrectType {
        expected_type: usize,
        actual_type: usize,
        location: Location,
    },
    #[error(display("Dictionary pointer was not aligned"))]
    InvalidAlignment { location: Location },

    #[error(display("Data is corrupt: {reason}"))]
    Corrupt { reason: String, location: Location },

    //
    #[error(display("..."))]
    General {
        #[error(impl_from)]
        error: Error,
        location: Location,
    },
    #[error(transparent)]
    Other {
        #[error(impl_from)]
        error: Box<dyn core::error::Error>,
        location: Location,
    },
}

/// Result type alias for MeCrab operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for MeCrab operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("The provided dictionary could not be found at: {0}")]
    DictionaryNotFound(PathBuf),

    #[error("Error parsing dictionary: {0}")]
    DictionaryParseError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Memory mapping error
    #[error("Memory mapping error: {0}")]
    MmapError(String),

    /// Double-Array Trie lookup failed
    #[error("Trie lookup error: {0}")]
    TrieLookupError(String),

    /// Connection matrix error
    #[error("Connection matrix error: {0}")]
    MatrixError(String),

    /// Character definition error
    #[error("Character definition error: {0}")]
    CharDefError(String),

    /// Unknown word handling error
    #[error("Unknown word error: {0}")]
    UnknownWordError(String),

    /// Lattice construction error
    #[error("Lattice construction error: {0}")]
    LatticeError(String),

    /// Viterbi algorithm error
    #[error("Viterbi algorithm error: {0}")]
    ViterbiError(String),

    /// Encoding error (for EUC-JP dictionaries)
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// Default dictionary not found
    #[error("A dictionary directory was not set")]
    DictionaryNotSet,

    /// Feature string parsing error
    #[error("Feature parsing error: {0}")]
    FeatureParseError(String),

    /// Format error (for DOT output, etc.)
    #[error("Format error: {0}")]
    FormatError(String),

    #[error(transparent)]
    Any(#[from] Box<dyn core::error::Error>),
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Error::FormatError(e.to_string())
    }
}
