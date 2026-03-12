#![no_main]
use std::path::PathBuf;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use mecrab::MeCrab;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    text: String,
    add_words: Vec<(String, String, i16)>,
}

fuzz_target!(|input: FuzzInput| {
    let dicdir = std::env::var("DICTIONARY_DIR").unwrap();
    let dicdir = Some(PathBuf::from(dicdir));

    if let Ok(mecrab) = MeCrab::builder().dicdir(dicdir).build() {
        for (surface, reading, cost) in input.add_words.iter().take(10) {
            if surface.len() < 100 && reading.len() < 100 {
                mecrab.add_word(surface, reading, reading, *cost);
            }
        }

        let _ = mecrab.parse(&input.text);
    }
});
