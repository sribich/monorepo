#![no_main]
use std::path::PathBuf;

use libfuzzer_sys::fuzz_target;
use mecrab::dict::Dictionary;
use mecrab::lattice::Lattice;

fuzz_target!(|data: &str| {
    let dicdir = std::env::var("DICTIONARY_DIR").unwrap();
    let dicdir = PathBuf::from(dicdir);

    if let Ok(dict) = Dictionary::load(&dicdir) {
        let _ = Lattice::build(data, &dict);
    }
});
