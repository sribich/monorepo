use std::fmt::Debug;
use std::iter::Enumerate;
use std::pin::Pin;
use std::range::Range;
use std::str::Split;

use memchr::memchr_iter;

#[derive(Debug)]
pub enum Morpheme {
    Unk,
    Tagged(TaggedMorpheme),
}

/// # Safety
///
/// This type is self-referential. As it stands, it can not be [`clone`] as
/// we are not storing any range information. If we stored range data along
/// with the slices, we could manually implement clone.
pub struct TaggedMorpheme {
    surface: &'static str,
    feature: Feature,
    /// # Safety
    ///
    /// This field MUST come last to prevent undefined behavior when the
    /// struct is dropped, as rust drops structs in order.
    data: Pin<Box<[u8]>>,
}

impl TaggedMorpheme {
    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn feature(&self) -> &Feature {
        &self.feature
    }
}

impl Debug for TaggedMorpheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaggedData")
            .field("surface", &self.surface)
            .field("feature", &self.feature)
            .finish()
    }
}

#[derive(Debug)]
pub struct Feature {
    /// 品詞大分類 - Part of speech field 1 -- Most general
    pub pos1: &'static str,
    /// 品詞中分類 - Part of speech field 2
    pub pos2: &'static str,
    /// 品詞小分類 - Part of speech field 3
    pub pos3: &'static str,
    /// 品詞細分類 - Part of speech field 4 -- Most specific
    pub pos4: &'static str,
    /// 活用型 - Conjugation type. Will have a value like 五段-ラ行.
    /// Inflection type (???)
    pub c_type: &'static str,
    /// 活用形 - Conjugation shape (活用形). Will have a value like 連用形-促音便.
    /// Inflection form (???)
    pub c_form: &'static str,
    /// 語彙素読み, lemma reading. The reading of the lemma in katakana, this
    /// uses the same format as the kana field, not pron.
    ///
    /// lemma reading is useful for text matching as it is not reliant on the form
    /// of the word, which transcription may get wrong.
    pub l_form: &'static str,
    /// 語彙素（＋語彙素細分類）. The lemma is a non-inflected "dictionary form"
    /// of a word. UniDic lemmas sometimes include extra info or have unusual
    /// forms, like using katakana for some place names.
    pub lemma: &'static str,
    /// 書字形出現形, the word as it appears in text, this appears to be
    /// identical to the surface.
    pub orth: &'static str,
    /// 書字形基本形 - The uninflected form of the word in its current written form.
    /// For example, for 彷徨った the lemma is さ迷う but the orthBase is
    /// 彷徨う.
    pub orth_base: &'static str,
}

pub trait TextSegmenter {
    type Feature;

    fn segment<S: AsRef<str>>(&self, text: S) -> Vec<Self::Feature>;
}

pub struct JapaneseTextSegmenter {
    tagger: mecab::Tagger,
}

impl JapaneseTextSegmenter {
    pub fn new() -> Self {
        Self {
            tagger: JapaneseTextSegmenter::get_tagger(),
        }
    }

    // TODO: This needs to be fetchable from somewhere.
    fn get_tagger() -> mecab::Tagger {
        let home = std::env::var("HOME").unwrap();

        let tagger = mecab::Tagger::new(format!(
            "-Ounidic --dicdir={}/Projects/sribich/_/unidic-cwj-202302",
            home
        ));

        tagger
    }
}

impl TextSegmenter for JapaneseTextSegmenter {
    type Feature = Morpheme;

    fn segment<S: AsRef<str>>(&self, text: S) -> Vec<Self::Feature> {
        let output = self.tagger.parse_str(text.as_ref());
        let bytes = output.as_bytes();

        println!("{}", output);

        let mut result = Vec::with_capacity(memchr_iter(b'\n', bytes).count());

        let mut line_start = 0;

        for line_end in memchr_iter(b'\n', bytes) {
            let line = &output[line_start..line_end];
            line_start = line_end + 1;

            if line.is_empty() || line == "BOS" || line == "EOS" {
                continue;
            }

            if line == "UNK" {
                result.push(Morpheme::Unk);
                continue;
            }

            let mut ranges: [Range<usize>; 11] = std::array::repeat((0_usize..0_usize).into());

            let mut start = 0;
            let mut i = 0;

            for end in memchr_iter(b',', line.as_bytes()) {
                ranges[i] = Range { start, end };
                start = end + 1;
                i += 1;
            }

            ranges[i] = Range {
                start,
                end: line.as_bytes().len(),
            };

            let data = Pin::from(line.as_bytes().to_owned().into_boxed_slice());

            // SAFETY: See [`TaggedMorpheme`]. It owns the data.
            #[allow(unsafe_code)]
            let tagged_data = unsafe {
                TaggedMorpheme {
                    surface: std::mem::transmute(&data[ranges[0]]),
                    feature: Feature {
                        pos1: std::mem::transmute(&data[ranges[1]]),
                        pos2: std::mem::transmute(&data[ranges[2]]),
                        pos3: std::mem::transmute(&data[ranges[3]]),
                        pos4: std::mem::transmute(&data[ranges[4]]),
                        c_type: std::mem::transmute(&data[ranges[5]]),
                        c_form: std::mem::transmute(&data[ranges[6]]),
                        l_form: std::mem::transmute(&data[ranges[7]]),
                        lemma: std::mem::transmute(&data[ranges[8]]),
                        orth: std::mem::transmute(&data[ranges[9]]),
                        orth_base: std::mem::transmute(&data[ranges[10]]),
                    },
                    // TODO(sr): We can store the entire `output` instead of cloning each `line`.
                    data,
                }
            };

            result.push(Morpheme::Tagged(tagged_data))
        }

        result
    }
}
