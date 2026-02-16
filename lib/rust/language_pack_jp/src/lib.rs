#![feature(new_range_api, const_range, const_trait_impl)]

use itertools::Itertools;

pub mod scanner;
pub mod segment;
pub mod splitting;
pub mod transcription;
pub mod transform;

/// Checks whether a given `char` is valid hiragana, based on the
/// [Unicode definition of hiragana].
///
/// ```
/// use language_pack_jp::is_hiragana;
///
/// assert!(is_hiragana('あ'));
/// assert!(is_hiragana('ゟ'));
/// assert!(!is_hiragana('a'));
/// ```
///
/// [Unicode definition of hiragana]: https://www.unicode.org/charts/PDF/U3040.pdf
pub const fn is_hiragana(c: char) -> bool {
    ('\u{3040}'..='\u{309f}').contains(&c)
}

/// Checks whether a given `char` is valid katakana based on the
/// [Unicode definition of katakana].
///
/// ```
/// use language_pack_jp::is_katakana;
///
/// assert!(is_katakana('ン'));
/// assert!(is_katakana('ヿ'));
/// assert!(!is_katakana('a'));
/// ```
///
/// [Unicode definition of katakana]: https://www.unicode.org/charts/PDF/U30A0.pdf
pub const fn is_katakana(c: char) -> bool {
    ('\u{30a0}'..='\u{30ff}').contains(&c)
}

/// Kanji appear in the Unicode range 4e00 to 9ffc, with the final Kanji
/// being 9fef (鿯).
///
/// For a chart of the full official range, see [this pdf] from the Unicode
/// organization.
///
/// A number of Level Pre-One Kanji appear in the [CJK Compatibility
/// Ideographs][compat] list, so there is an extra check here for those.
///
/// ```
/// use language_pack_jp::is_kanji;
///
/// assert!(is_kanji('澄')); // Obviously a legal Kanji.
/// assert!(!is_kanji('a')); // Obviously not.
/// ```
///
/// [compat]: https://www.unicode.org/charts/PDF/UF900.pdf
/// [this pdf]: https://www.unicode.org/charts/PDF/U4E00.pdf
pub fn is_kanji(c: char) -> bool {
    ('\u{4e00}'..='\u{9ffc}').contains(&c) // Standard set.
        || ('\u{f900}'..='\u{faff}').contains(&c) // CJK Compatibility Ideographs.
        || ('\u{3400}'..='\u{4dbf}').contains(&c) // Extension A
        || ('\u{20000}'..='\u{2a6dd}').contains(&c) // Extension B
        || ('\u{2a700}'..='\u{2b734}').contains(&c) // Extension C
        || ('\u{2b740}'..='\u{2b81d}').contains(&c) // Extension D
        || ('\u{2b820}'..='\u{2cea1}').contains(&c) // Extension E
        || ('\u{2ceb0}'..='\u{2ebe0}').contains(&c) // Extension F
        || ('\u{30000}'..='\u{3134a}').contains(&c) // Extension G
}

pub fn get_reading_from_anki(s: String) -> String {
    let mut reading = String::new();

    for c in s.chars() {
        if is_hiragana(c) || is_katakana(c) {
            reading.push(c);
        }
    }

    reading
}

///
///
/// ```
/// use language_pack_jp::create_reading;
///
/// assert_eq!(create_reading("食い込む", "くいこむ"), "食[く]い込[こ]む")
/// ```
pub fn create_reading<S: AsRef<str>>(word: S, reading: S) -> String {
    let mut furigana = String::new();
    let mut reading = reading.as_ref();

    // Chunk the word based on its kanji boundaries
    let mut chunked_word = chunk_word(word)
        .into_iter()
        .circular_tuple_windows()
        .collect::<Vec<(String, String)>>();

    chunked_word.last_mut().unwrap().1 = String::new();

    for (left, right) in chunked_word {
        furigana.push_str(&left);

        if right.is_empty() {
            if is_kanji(left.chars().next().unwrap()) {
                furigana.push_str(&format!("[{reading}]"));
            }

            break;
        }

        if is_kanji(left.chars().next().unwrap()) {
            let mut indices = reading.match_indices(&right);

            // We only care about the first one for now, we need
            // to find cases where this fails.
            let (index, _) = indices.next().unwrap();

            furigana.push_str(&format!("[{}]", &reading[..index]));

            reading = &reading[index..];
        } else {
            reading = &reading[left.len()..];
        }
    }

    furigana
}

/// ```
/// use language_pack_jp::chunk_word;
///
/// assert_eq!(chunk_word("食い込む"), vec!["食", "い", "込", "む"]);
/// assert_eq!(chunk_word("文房具"), vec!["文房具"]);
/// assert_eq!(chunk_word("味わう"), vec!["味", "わう"]);
/// assert_eq!(chunk_word("お願い"), vec!["お", "願", "い"]);
/// ```
pub fn chunk_word<S: AsRef<str>>(word: S) -> Vec<String> {
    let mut result = vec![];
    let mut kanji = false;

    for c in word.as_ref().chars() {
        if result.is_empty() {
            kanji = is_kanji(c);
            result.push(String::new());
        }

        match (kanji, is_kanji(c)) {
            (true, true) => {
                result.last_mut().unwrap().push(c);
            }
            (true, false) => {
                result.push(c.to_string());
                kanji = false;
            }
            (false, false) => {
                result.last_mut().unwrap().push(c);
            }
            (false, true) => {
                result.push(c.to_string());
                kanji = true;
            }
        }
    }

    result
}
