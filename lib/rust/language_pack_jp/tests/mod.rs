use std::ffi::CStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Mul;
use std::path::PathBuf;
use std::pin::Pin;

use insta::assert_snapshot;
use insta::assert_yaml_snapshot;
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

#[test]
fn check_regressions() {
    let (_buf, dictionary, dictionary_readings) = load_dictionaries();

    let tests = [
        "響いてた",
        "くる",
        "駆け寄ってくる",
        "駆け寄って",
        "ついて",
        "たとえ雨が降っても、外に出よう。",
        "雨が降ったけれど、外に出た。",
        "漬けこんでやろう",
        "臭くない",
        "臭くない",
        "名前は",
        "ある",
        "本がすぐそこにあるのに",
        // BROKEN
        "なった",
        "泣きそうになった",
        "という",
        "すぐそこ",
        "すぐそこに",
        "おばあさん",
        "というおばあさん",
        "預けられていた", // Should fully match

                          // マインは蜘蛛の巣が怖いのか仕方がないな父さんが取ってやろう
                          // CURR: [父][さんが][取って][やろう]
                          // GOOD: [父さん][が][取って][やろう]
                          //
                          // TO FIX THIS, WE SHOULD MAKE SURE THAT PARTICLES ARE NOT EVALUATED
                          //
                          // Checks should be histeruics based. If a word was not inflicted and is all kana
                          // then we should prefer a kana lookup if one exists, only falling back to reading
                          // based lookups if one does not exist.
                          //
                          // If it was inflicted, :shrug:

                          // のところにはわたしと同じような子供が
                          // 溶かした鍋に[入れたり][出したり]する何度も
        //
        //
        // 面白くない   [面][白くない] -> [面白くない]
    ];

    for test in tests {
        let result = transform_japanese_text(test, &dictionary, &dictionary_readings);
        assert_yaml_snapshot!(result)
    }
}
/*
トゥーリ != トゥー + リ == トゥーリ
駆け寄ってくる != 駆け + 寄 + っ + てくる == 駆け寄って + くる
響いてた != 響い + てた == 響いてた
*/

// # Conjugation
//
//   - もうしかして -> もうしかして NOT もう_か_して
//
// # Word Separation
//
// - 入手不可能
//   - 入手 - 不 - 可能
//   - 入手 - 不可能
//   - 入手不可能
//
//

// 思いつかなかった。is broken. った is separated

// 保存食 shoku is split

// 信じられない is showing shinzuru not shinjiru
