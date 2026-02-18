use serde::Deserialize;

/// The output from our WhisperX transcription process.
///
/// See `transcription.py` in this repository for more
/// information.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Transcription {
    pub segments: Vec<Segment>,
}

/// An individual text segment that was transcribed.
#[derive(Deserialize)]
pub struct Segment {
    /// The line of text as it was transcribed.
    pub text: String,
    /// The start time in <sec>.<msec> format.
    pub start: f64,
    /// The end time in <sec>.<msec> format.
    pub end: f64,
    /// Word level timestamps from WhisperX's forced alignment.
    pub words: Vec<Word>,
}

/// A word that was picked out of the forced alignment model.
#[derive(Deserialize)]
pub struct Word {
    /// The word as it was parsed in forced alignment.
    pub word: String,
    /// The start time in <sec>.<msec> format.
    pub start: Option<f64>,
    /// The end time in <sec>.<msec> format.
    pub end: Option<f64>,
}
