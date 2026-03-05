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

/*
let segmenter = JapaneseTextSegmenter::new();

        let timing_segments = timing_data.segments(&segmenter);
        let text_segments = text_data.segments(&segmenter);

        let timing_segments = timing_segments
            .filter(|it| match &it.data {
                Morpheme::Untagged(data) => {
                    match get_single_char(data.as_str()) {
                        Some(c) => !is_punctuation(c),
                        None => true,
                    }
                },
                Morpheme::Tagged(tag) => {
                    let surface = tag.surface();
                    match get_single_char(surface) {
                        Some(c) => !is_punctuation(c),
                        None => true,
                    }
                }
            })
            .collect::<Vec<_>>();
                    let text_segments = text_segments
            .filter(|it| match &it.data {
                Morpheme::Untagged(data) => {
                    match get_single_char(data.as_str()) {
                        Some(c) => !is_punctuation(c),
                        None => true,
                    }
                },
                Morpheme::Tagged(tag) => {
                    let surface = tag.surface();
                    match get_single_char(surface) {
                        Some(c) => !is_punctuation(c),
                        None => true,
                    }
                }
            })
            .collect::<Vec<_>>();
*/
