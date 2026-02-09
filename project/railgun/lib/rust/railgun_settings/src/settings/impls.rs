use std::cmp::Reverse;
use std::ffi::CString;
use std::ffi::OsString;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::num::Wrapping;
use std::ops::Range;
use std::path::PathBuf;
use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;

use super::Settings;

macro_rules! impl_noop {
    ( $( $impl_desc:tt )* ) => {
        impl $( $impl_desc )* {
            #[inline]
            fn add_docs(
                &self,
                _parent_key: &[String],
                _docs: &mut std::collections::HashMap<Vec<String>, &'static [&'static str]>,
            ) {
            }
        }
    };
}

impl_noop!(<T> Settings for PhantomData<T> where T: 'static);
impl_noop!(<T> Settings for [T; 0] where T: Debug + Clone + 'static);
impl_noop!(<Idx> Settings for Range<Idx> where Idx: Debug + Serialize + DeserializeOwned + Clone + Default + 'static);
impl_noop!(<T> Settings for Reverse<T> where T: Settings);
impl_noop!(<T> Settings for Wrapping<T> where T: Settings);

macro_rules! impl_for_non_generic {
    ( $( $Ty:ty ),* ) => {
        $( impl_noop!(Settings for $Ty); )*
    };
}

impl_for_non_generic! {
    bool,
    char,
    f32,
    f64,
    i128,
    i16,
    i32,
    i64,
    i8,
    isize,
    u128,
    u16,
    u32,
    u64,
    u8,
    usize,
    String,
    (),
    CString,
    OsString,
    Duration,
    PathBuf
}
