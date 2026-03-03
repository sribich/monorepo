#![allow(clippy::indexing_slicing, reason = "Needed for alignment performance")]
#![feature(new_range_api, const_range, const_trait_impl, if_let_guard)]
use language_pack::pipeline::LanguagePipeline;
use language_pack::pipeline::create_pipeline;
use segment::JapaneseTextSegmenter;
use segment::Morpheme;

pub mod processor;
pub mod segment;
pub mod text;
pub mod transcription;
pub mod transform;

pub fn japanese_language_pipeline() -> LanguagePipeline<JapaneseTextSegmenter, Morpheme> {
    create_pipeline(JapaneseTextSegmenter::new())
}
