use std::fmt::Debug;

use crate::CanSegment;
use crate::segment::IsSegment;
use crate::segment::TextSegmenter;

pub fn create_pipeline<TSegmenter, TFeature>(
    segmenter: TSegmenter,
) -> LanguagePipeline<TSegmenter, TFeature>
where
    TSegmenter: TextSegmenter<Feature = TFeature>,
    TFeature: IsSegment + Debug,
{
    LanguagePipeline { segmenter }
}

///
pub struct LanguagePipeline<TSegmenter, TFeature>
where
    TSegmenter: TextSegmenter<Feature = TFeature>,
    TFeature: IsSegment + Debug,
{
    segmenter: TSegmenter,
}

impl<TSegmenter, TFeature> LanguagePipeline<TSegmenter, TFeature>
where
    TSegmenter: TextSegmenter<Feature = TFeature>,
    TFeature: IsSegment + Debug,
{
    pub fn run<CTiming, CText>(&self, timing_data: &CTiming, text_data: &CText)
    where
        CTiming: CanSegment<TFeature>,
        CText: CanSegment<TFeature>,
    {
        println!("hi!");

        let timing_segments = timing_data.segments(&self.segmenter);

        for segment in timing_segments {
            if segment.data.text() == "UNK" {
                println!("{:#?}", segment);
            }
        }

        /*
        let timing_segments = timing_data.segments(&segmenter);
        let text_segments = text_data.segments(&segmenter);
         */
    }
}
