use std::fmt::Debug;

use crate::AcceptsTimestamps;
use crate::CanSegment;
use crate::HasTimestamp;
use crate::ProducesTimestamps;
use crate::align::Alignment;
use crate::align::align_segments;
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
    TFeature: IsSegment + Debug + Send + Sync,
{
    pub fn run<CTiming, CText>(&self, timing_data: &CTiming, text_data: &mut CText)
    where
        CTiming: CanSegment<TFeature> + ProducesTimestamps<TFeature> + Sync,
        CTiming::Source: Send + Sync + HasTimestamp,
        CText: CanSegment<TFeature> + AcceptsTimestamps<TFeature> + Sync,
        CText::Source: Send + Sync,
    {
        let timing_segments = timing_data.segments(&self.segmenter);
        let text_segments = text_data.segments(&self.segmenter);

        let alignments = align_segments(&timing_segments, &text_segments);

        for Alignment([(timing_pos, timing_len), (text_pos, text_len)]) in alignments {
            if timing_len == text_len {
                for i in 0..timing_len {
                    let timing_segment = &timing_segments[timing_pos + i];
                    let text_segment = &text_segments[text_pos + i];

                    text_data.accept(text_segment, timing_segment.source.timestamp());
                }
            } else {
                let timing_segments = &timing_segments[timing_pos..(timing_pos + timing_len)];
                let text_segments = &text_segments[text_pos..(text_pos + text_len)];

                // MAYBE: Look into matching these over boundaries
                text_data.try_accept_ranged(text_segments, timing_data.produce(timing_segments));

                // println!("Success: {}, Failed: {} -- ({}, {}) -> ({}, {})", s, f, timing_pos, timing_len, text_pos, text_len);
            }
        }
    }
}
