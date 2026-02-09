//! Query graph builder module.

mod builder;
mod error;
mod extractors;

pub(crate) mod inputs;
pub(crate) mod read;
pub(crate) mod write;
pub use builder::QueryGraphBuilder;
pub use error::*;
pub(crate) use extractors::*;

/// Query graph builder sub-result type.
pub type QueryGraphBuilderResult<T> = Result<T, QueryGraphBuilderError>;
