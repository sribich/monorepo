//! This library implements the EPUB specification as defined in the EPUB 3.4
//! revision.
//!
//! The specification can be found at <https://www.w3.org/TR/epub-34/>.
pub mod archive;
pub mod epub;
pub(crate) mod error;

pub use error::Error;
