#![no_main]
use std::path::PathBuf;

use libfuzzer_sys::fuzz_target;
use mecrab::MeCrab;

fuzz_target!(|data: &str| {
    let dicdir = std::env::var("DICTIONARY_DIR").unwrap();
    let dicdir = Some(PathBuf::from(dicdir));

    if let Ok(mecrab) = MeCrab::builder().dicdir(dicdir).build() {
        let _ = mecrab.parse(data);
    }
});
