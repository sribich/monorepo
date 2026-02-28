use std::fs::read_to_string;
use std::path::PathBuf;

use epub::archive::EpubArchive;
use language_pack_jp::transcription::JapaneseTranscriptionContext;

/*
#[test]
fn real_example() {
    let home = std::env::var("HOME").unwrap();

    let ebook =
        PathBuf::from(&home).join("Japanese/bookworm/本好きの下剋上 01 第一部 兵士の娘I.epub");
    let audio = PathBuf::from(&home).join("Japanese/bookworm.json");

    let mut archive = EpubArchive::open(&ebook).unwrap();
    let text = archive.segments().unwrap();

    let timing_data = read_to_string(&audio).unwrap();

    let transcriber = JapaneseTranscriptionContext {};
    transcriber.test(text, &timing_data);
}
*/

/*

[
    word: "",
    source: [segment, word]
    dest: [segment, word]
]


*/
