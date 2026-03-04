use std::fmt::Debug;

use crate::CanSegment;
use crate::segment::IsSegment;
use crate::segment::TextSegmenter;

pub fn create_pipeline<TSegmenter, TFeature>(
    segmenter: TSegmenter,
) -> LanguagePipeline<TSegmenter, TFeature>
where
    TSegmenter: TextSegmenter<Feature = TFeature>,
    TFeature: IsSegment + Debug + Send,
{
    LanguagePipeline { segmenter }
}

pub struct LanguagePipeline<TSegmenter, TFeature>
where
    TSegmenter: TextSegmenter<Feature = TFeature>,
    TFeature: IsSegment + Debug + Send,
{
    segmenter: TSegmenter,
}

impl<TSegmenter, TFeature> LanguagePipeline<TSegmenter, TFeature>
where
    TSegmenter: TextSegmenter<Feature = TFeature> + Sync,
    TFeature: IsSegment + Debug + Send,
{
    pub fn run<CTiming, CText>(&self, timing_data: &CTiming, text_data: &CText)
    where
        CTiming: CanSegment<TFeature>,
        CText: CanSegment<TFeature>,
    {
        let timing_segments = timing_data.segments(&self.segmenter);
        let text_segments = text_data.segments(&self.segmenter);
    }
}
