use std::ffi::CStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Mul;
use std::path::PathBuf;
use std::pin::Pin;

use blart::TreeMap;
use insta::assert_snapshot;
use insta::assert_yaml_snapshot;
use language_pack_jp::transform::transform_japanese_text;

fn load_dictionaries() -> (
    Pin<Box<[u8]>>,
    TreeMap<&'static CStr, Option<usize>>,
    TreeMap<&'static CStr, Option<usize>>,
) {
    let home = std::env::var("HOME").unwrap();
    let path = PathBuf::from(format!("{home}/opt/dictionary_entries.csv"));

    let lines = BufReader::new(File::open(&path).unwrap()).lines();
    let len = std::fs::metadata(path).unwrap().len();

    let mut buf = Pin::new(
        vec![0; usize::try_from(len.mul(2)).expect("usize should always fit within u64")]
            .into_boxed_slice(),
    );

    let mut view = &mut buf[..];

    let mut adaptive = TreeMap::<&CStr, Option<usize>>::new();
    let mut adaptive_readings = TreeMap::<&CStr, Option<usize>>::new();

    #[expect(clippy::indexing_slicing, unsafe_code, reason = "See safety comment")]
    for line in lines.map_while(Result::ok) {
        if line == r#""""# {
            continue;
        }

        if let Some((left, right)) = line.split_once(',')
            && let Some((middle, right)) = right.split_once(',')
        {
            let freq = right.parse::<usize>().ok();

            if left != r#""""# {
                assert!(
                    memchr::memchr(0, left.as_bytes()).is_none(),
                    "string with null byte"
                );

                let left_len = left.len();

                view[..left_len].copy_from_slice(left.as_bytes());
                view[left_len] = 0;

                // SAFETY: The type is coerced into a static str so that
                let left: &'static CStr = unsafe {
                    std::mem::transmute(CStr::from_bytes_with_nul_unchecked(&view[..=left_len]))
                };
                adaptive.try_insert(left, freq).unwrap();

                view = &mut view[(left_len + 1)..];
            }

            if middle != r#""""# {
                assert!(
                    memchr::memchr(0, middle.as_bytes()).is_none(),
                    "string with null byte"
                );

                let middle_len = middle.len();

                view[..middle_len].copy_from_slice(middle.as_bytes());
                view[middle_len] = 0;

                let middle = unsafe {
                    std::mem::transmute(CStr::from_bytes_with_nul_unchecked(&view[..=middle_len]))
                };
                adaptive_readings.try_insert(middle, freq).unwrap();

                view = &mut view[(middle_len + 1)..];
            }
        }
    }

    (buf, adaptive, adaptive_readings)
}

#[test]
fn thing() {
    let (buf, dictionary, dictionary_readings) = load_dictionaries();

    let result = transform_japanese_text("響いてた", &dictionary, &dictionary_readings);

    assert_yaml_snapshot!(result, @"");
}

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

/*
トゥーリ != トゥー + リ == トゥーリ
駆け寄ってくる != 駆け + 寄 + っ + てくる == 駆け寄って + くる
響いてた != 響い + てた == 響いてた
*/
