#![feature(new_range_api)]
pub mod align;
pub mod hirschberg;
pub mod pipeline;
pub mod processor;
pub mod segment;
pub mod transform;

use std::f64::INFINITY;
use std::fmt::Debug;
use std::range::Range;

use segment::IsSegment;
use segment::TextSegmenter;
use serde::Deserialize;

/// Defines functionality for a type that can hold segment information.
pub trait CanSegment<T: IsSegment + Debug> {
    type Source: Debug;

    fn segments(
        &self,
        segmenter: impl TextSegmenter<Feature = T>,
    ) -> impl Iterator<Item = Segment<T, Self::Source>>;

    fn source<'a>(&self, segment: &'a Segment<T, Self::Source>) -> &'a Self::Source;
}

impl<T> ProducesTimestamps<T> for Transcription
where
    T: IsSegment + PartialEq + Debug,
{
    fn produce(&self, segments: &[Segment<T, Self::Source>]) -> Timestamp {
        segments
            .iter()
            .fold((None, None), |prev, curr| match curr.source.timestamp {
                (None, None) => prev,
                (None, Some(t1)) => (prev.0, prev.1.map(|it| it.max(t1)).or(Some(t1))),
                (Some(t0), None) => (prev.0.map(|it| it.min(t0)).or(Some(t0)), prev.1),
                (Some(t0), Some(t1)) => (
                    prev.0.map(|it| it.min(t0)).or(Some(t0)),
                    prev.1.map(|it| it.max(t1)).or(Some(t1)),
                ),
            })
    }
}

impl<T> CanSegment<T> for Transcription
where
    T: IsSegment + PartialEq + Debug,
{
    type Source = TranscriptionSource;

    fn segments(
        &self,
        segmenter: impl TextSegmenter<Feature = T>,
    ) -> impl Iterator<Item = Segment<T, Self::Source>> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(line_index, line)| {
                let segments = segmenter.segment(&line.text);

                let mut word_idx = 0;
                let mut char_buf: [u8; 4] = [0; 4];

                let matches = segments
                    .into_iter()
                    .map(|it| {
                        let text = it.text();

                        // TODO: Figure out how to resolve UNKs
                        if text == "UNK" || line.words.is_empty() {
                            return (it, (None, None));
                        }

                        let mut t0 = INFINITY;
                        let mut t1 = 0_f64;

                        for c in text.chars() {
                            let char_str: &str = c.encode_utf8(&mut char_buf);

                            while word_idx < line.words.len()
                                && line.words[word_idx].word != char_str
                            {
                                word_idx += 1;
                            }

                            assert!(
                                line.words[word_idx].word == char_str,
                                "Failed to find match"
                            );

                            t0 = t0.min(line.words[word_idx].start.unwrap_or(t0));
                            t1 = t1.max(line.words[word_idx].end.unwrap_or(t1));
                        }

                        return (
                            it,
                            (
                                if t0 == INFINITY { None } else { Some(t0) },
                                if t1 == 0_f64 { None } else { Some(t1) },
                            ),
                        );
                    })
                    .collect::<Vec<_>>();

                matches.into_iter().map(move |segment| Segment {
                    data: segment.0,
                    source: TranscriptionSource {
                        line: line_index,
                        timestamp: segment.1,
                    },
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn source<'a>(&self, segment: &'a Segment<T, Self::Source>) -> &'a Self::Source {
        todo!()
    }
}

pub type Timestamp = (Option<f64>, Option<f64>);

pub trait AcceptsTimestamps<T>
where
    T: IsSegment + PartialEq + Debug,
    Self: CanSegment<T>,
{
    fn accept(&mut self, segment: &Segment<T, Self::Source>, timestamp: Timestamp);

    fn try_accept_ranged(
        &mut self,
        segment: &[Segment<T, Self::Source>],
        timestamp: Timestamp,
    ) -> bool;
}

pub trait ProducesTimestamps<T>
where
    T: IsSegment + PartialEq + Debug,
    Self: CanSegment<T>,
{
    fn produce(&self, segments: &[Segment<T, Self::Source>]) -> Timestamp;
}

pub trait HasTimestamp {
    fn timestamp(&self) -> Timestamp;
}

#[derive(Debug)]
pub struct TranscriptionSource {
    line: usize,
    timestamp: (Option<f64>, Option<f64>),
}

impl HasTimestamp for TranscriptionSource {
    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

#[derive(Debug, Eq)]
pub struct Segment<TData, TSource>
where
    TData: PartialEq + Debug + IsSegment,
{
    pub data: TData,
    pub source: TSource,
}

impl<TData: PartialEq + Debug + IsSegment, TSourceA, TSourceB> PartialEq<Segment<TData, TSourceB>>
    for Segment<TData, TSourceA>
{
    fn eq(&self, other: &Segment<TData, TSourceB>) -> bool {
        self.data == other.data
    }
}

/// The output from the WhisperX transcription process.
///
/// See `transcription.py` in this repository for more
/// information.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Transcription {
    pub lines: Vec<Line>,
}

/// An individual text segment that was transcribed.
#[derive(Debug, Deserialize)]
pub struct Line {
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
        value.to_owned().into()
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
    fn text_processors(&'_ self) -> TextProcessors<'_>;

    fn transform(&self, text: String) -> Transform;
}
