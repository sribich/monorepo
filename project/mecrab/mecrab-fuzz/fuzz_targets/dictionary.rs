#![no_main]
use std::path::PathBuf;

use libfuzzer_sys::fuzz_target;
use mecrab::dict::Dictionary;

fuzz_target!(|data: &str| {
    let dicdir = std::env::var("DICTIONARY_DIR").unwrap();
    let dicdir = PathBuf::from(dicdir);

    if let Ok(dict) = Dictionary::load(&dicdir) {
        let _ = dict.lookup(data);

        for c in data.chars() {
            let _ = dict.char_info(c);
            let _ = dict.char_category(c);
        }
    }
});
