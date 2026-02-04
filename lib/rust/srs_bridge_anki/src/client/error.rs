use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnkiError {
    #[error("...")]
    Unit,

    #[error("Error making request to anki: {0}")]
    Reqwest(reqwest::Error),

    #[error("Anki returned an error: {0}")]
    Anki(String),
}

pub type Result<T> = std::result::Result<T, AnkiError>;
