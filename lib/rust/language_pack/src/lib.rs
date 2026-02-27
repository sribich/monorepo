#![feature(new_range_api)]
use std::range::Range;

use serde::Deserialize;

/// Defines functionality for a type that can hold segment information.
pub trait CanSegment {
    /// Returns
    fn text(&self) -> &str;

    fn segments(&self) -> impl Iterator<Item = ()>;
}

pub trait TextSegmenter {
    type Feature;

    fn segment<S: AsRef<str>>(&self, text: S) -> Vec<Self::Feature>;
}

/// The output from the WhisperX transcription process.
///
/// See `transcription.py` in this repository for more
/// information.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Transcription<T: Default> {
    pub segments: Vec<Segment<T>>,
}

/// An individual text segment that was transcribed.
#[derive(Debug, Deserialize)]
pub struct Segment<T> {
    /// The line of text as it was transcribed.
    pub text: String,
    /// The start time in <sec>.<msec> format.
    pub start: f64,
    /// The end time in <sec>.<msec> format.
    pub end: f64,
    /// Word level timestamps from WhisperX's forced alignment.
    pub words: Vec<Word>,
    #[serde(skip, default)]
    pub segments: Vec<T>,
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
