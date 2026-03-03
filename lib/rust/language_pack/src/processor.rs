use std::range::Range;

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

pub trait Processor {
    fn processors(&'_ self) -> TextProcessors<'_>;

    fn transform(&self, text: String) -> Transform;
}
