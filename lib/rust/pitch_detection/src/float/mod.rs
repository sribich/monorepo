//! Generic [Float] type which acts as a stand-in for `f32` or `f64`.
use std::fmt::Debug;
use std::fmt::Display;

use rustfft::FftNum;
use rustfft::num_traits::float::FloatCore as NumFloatCore;

/// Signals are processed as arrays of [Float]s. A [Float] is normally `f32` or `f64`.
pub trait Float: Display + Debug + NumFloatCore + FftNum {}

impl Float for f64 {}
impl Float for f32 {}
