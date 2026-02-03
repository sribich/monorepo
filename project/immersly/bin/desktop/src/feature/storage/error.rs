// use thiserror::Error;
//
// #[derive(Debug, Error)]
// pub enum FsError {
// #[error("Expected the path '{0}' to be a relative path. Found an absolute path")]
// NotRelative(String),
//
// #[error("Expected the path '{0}' to be a directory. Found a file instead")]
// NotADirectory(String),
//
// #[error("Expected the path '{0}' to be a file. Found a directory instead")]
// NotAFile(String),
//
// #[error(transparent)]
// Other(#[from] color_eyre::eyre::Error),
// }
//
