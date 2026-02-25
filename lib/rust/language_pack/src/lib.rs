#![feature(new_range_api)]
use std::range::Range;

use serde::Deserialize;

/// The output from our WhisperX transcription process.
///
/// See `transcription.py` in this repository for more
/// information.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Transcription {
    pub segments: Vec<Segment>,
}

/// An individual text segment that was transcribed.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub struct Word {
    /// The word as it was parsed in forced alignment.
    pub word: String,
    /// The start time in <sec>.<msec> format.
    pub start: Option<f64>,
    /// The end time in <sec>.<msec> format.
    pub end: Option<f64>,
}

/// A `Transform` represents a transformation that has been applied to some text.
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Transform {
    /// The transformed text.
    pub text: String,
    /// Mapping to the original text.
    ///
    /// This is done using byte-position and not character position.
    pub mappings: Vec<(Range<usize>, Range<usize>)>,
    /// The previous transform that was done.
    pub previous: Option<Box<Transform>>,
}

impl From<String> for Transform {
    fn from(value: String) -> Self {
        Transform {
            text: value.clone(),
            mappings: vec![((0..value.len()).into(), (0..value.len()).into())],
            previous: None,
        }
    }
}

impl From<&str> for Transform {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

pub trait TextProcessor {
    /// Checks whether the processor can be run over a given text.
    fn matches(&self, text: &str) -> bool;

    fn process(&self, transform: Transform) -> Transform;
}

pub struct TextProcessors<'a> {
    pub pre: &'a [&'a dyn TextProcessor],
    pub post: &'a [&'a dyn TextProcessor],
}

pub trait LanguageTransformer {
    fn text_processors(&self) -> TextProcessors;

    fn transform(&self, text: String) -> Transform;
}
