use std::fmt::Display;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcription {
    pub segments: Vec<Segment>,
    pub word_segments: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub text: String,
    pub start: f64,
    pub end: f64,
    pub words: Vec<Word>,
    pub chars: Vec<Char>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub score: Option<f64>,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.word)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Char {
    pub char: String,

    pub start: Option<f64>,
    pub end: Option<f64>,
    pub score: Option<f64>,
}
