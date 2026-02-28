#![allow(clippy::indexing_slicing, reason = "Needed for alignment performance")]
#![feature(new_range_api, const_range, const_trait_impl, if_let_guard)]
pub mod segment;
pub mod splitting;
pub mod text;
pub mod transcription;
pub mod transform;
