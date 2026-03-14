use std::ffi::CStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Mul;
use std::path::PathBuf;
use std::pin::Pin;

use insta::assert_snapshot;
use insta::assert_yaml_snapshot;
use language_pack::segment::TextSegmenter;
use language_pack_jp::segment::JapaneseTextSegmenter;
use language_pack_jp::transform::group_inflected;
use language_pack_jp::transform::transform_japanese_text;
use trie_rs::map::Trie;

fn load_words() -> (Pin<Box<[u8]>>, Trie<u8, Option<usize>>) {
    let time = std::time::Instant::now();

    let home = std::env::var("HOME").unwrap();
    let path = PathBuf::from(format!("{home}/opt/dictionary_words.csv"));

    let lines = BufReader::new(File::open(&path).unwrap()).lines();
    let len = std::fs::metadata(path).unwrap().len();

    let mut buf = Pin::new(
        vec![0; usize::try_from(len).expect("usize should always fit within u64")]
            .into_boxed_slice(),
    );

    let mut words = trie_rs::map::TrieBuilder::<u8, Option<usize>>::new();

    let mut view = &mut buf[..];

    for line in lines.map_while(Result::ok) {
        if line == r#""""# {
            continue;
        }

        if let Some((word, freq)) = line.split_once(',') {
            let freq = freq.parse::<usize>().ok();

            if word != r#""""# {
                assert!(
                    memchr::memchr(0, word.as_bytes()).is_none(),
                    "string with null byte"
                );

                let len = word.len();

                view[..len].copy_from_slice(word.as_bytes());

                // SAFETY: The type is coerced into a static str so that
                let word: &'static str = unsafe { std::mem::transmute(&view[..len]) };

                words.push(word, freq);

                view = &mut view[len..];
            }
        }
    }

    let words = words.build();

    (buf, words)
}

fn load_readings() -> (Pin<Box<[u8]>>, Trie<u8, Option<usize>>) {
    let home = std::env::var("HOME").unwrap();
    let path = PathBuf::from(format!("{home}/opt/dictionary_readings.csv"));

    let lines = BufReader::new(File::open(&path).unwrap()).lines();
    let len = std::fs::metadata(path).unwrap().len();

    let mut buf = Pin::new(
        vec![0; usize::try_from(len).expect("usize should always fit within u64")]
            .into_boxed_slice(),
    );

    let mut readings = trie_rs::map::TrieBuilder::<u8, Option<usize>>::new();

    let mut view = &mut buf[..];

    for line in lines.map_while(Result::ok) {
        if line == r#""""# {
            continue;
        }

        if let Some((word, freq)) = line.split_once(',') {
            let freq = freq.parse::<usize>().ok();

            if word != r#""""# {
                assert!(
                    memchr::memchr(0, word.as_bytes()).is_none(),
                    "string with null byte"
                );

                let len = word.len();

                view[..len].copy_from_slice(word.as_bytes());

                // SAFETY: The type is coerced into a static str so that
                let word: &'static str = unsafe { std::mem::transmute(&view[..len]) };

                readings.push(word, freq);

                view = &mut view[len..];
            }
        }
    }

    let readings = readings.build();

    (buf, readings)
}

fn load_dictionaries() -> (
    (Pin<Box<[u8]>>, Pin<Box<[u8]>>),
    Trie<u8, Option<usize>>,
    Trie<u8, Option<usize>>,
) {
    let (words_buf, words) = load_words();
    let (readings_buf, readings) = load_readings();

    ((words_buf, readings_buf), words, readings)
}

fn get_test_segments() -> &'static [&'static str] {
    &[
        "響いてた",
        "くる",
        "駆け寄ってくる",
        "駆け寄って",
        "ついて",
        "たとえ雨が降っても、外に出よう。",
        "雨が降ったけれど、外に出た。",
        "漬けこんでやろう",
        "臭くない",
        "名前は",
        "ある",
        "本がすぐそこにあるのに",
        "なった",
        "泣きそうになった",
        "という",
        "すぐそこ",
        "すぐそこに",
        "おばあさん",
        "というおばあさん",
        "預けられていた", // Should fully match
        "死んだ",
        "本に潰されて死んだのは",
        "マインは蜘蛛の巣が怖いのか仕方がないな父さんが取ってやろう",
        "思いつかなかった",
        "保存食",
        "信じられない",
        "そう考えれば",
        // "「……行きたくない」",   - Mostly good, but ] is tacked on to the end in splitting
        // "もうしかして",
        // "面白くない",
        // 赤く染まった akaku is broken up
        // 気が付いた -- When we are doing word combinations, we are not basing them off of the
        //              roots at all, so we are missing when they are inflected.

        // トゥーリ != トゥー + リ == トゥーリ
        // Custom Dictionary Needed

        // TO FIX THIS, WE SHOULD MAKE SURE THAT PARTICLES ARE NOT EVALUATED
        //
        // Checks should be histeruics based. If a word was not inflicted and is all kana
        // then we should prefer a kana lookup if one exists, only falling back to reading
        // based lookups if one does not exist.
        //
        // If it was inflicted, :shrug:

        // TODO: Word Tree
        // - 入手不可能
        //   - 入手 - 不 - 可能
        //   - 入手 - 不可能
        //   - 入手不可能
    ]
}

#[test]
fn check_inflection_breakpoints() {
    let segmenter = JapaneseTextSegmenter::new();

    for test in get_test_segments() {
        let result = group_inflected(segmenter.segment(test, false));
        assert_yaml_snapshot!(result)
    }
}

#[test]
fn check_regressions() {
    let (_buf, dictionary, dictionary_readings) = load_dictionaries();

    for test in get_test_segments() {
        let result = transform_japanese_text(test, &dictionary, &dictionary_readings);
        assert_yaml_snapshot!(result)
    }
}
