/// A type that can segment a given into text into its basic
/// parts for further text processing.
pub trait TextSegmenter {
    type Feature: IsSegment;

    fn segment<S: AsRef<str>>(&self, text: S) -> Vec<Self::Feature>;
}

pub trait IsSegment: PartialEq {
    fn text(&self) -> &str;
}

impl<T: TextSegmenter> TextSegmenter for &T {
    type Feature = T::Feature;

    fn segment<S: AsRef<str>>(&self, text: S) -> Vec<Self::Feature> {
        T::segment(self, text)
    }
}
