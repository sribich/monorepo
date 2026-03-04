use core::fmt::Debug;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::pin::Pin;
use std::range::Range;

use language_pack::segment::IsSegment;
use language_pack::segment::TextSegmenter;
use lazy_static::lazy_static;
use mecrab::MeCrab;
use mecrab::OutputFormat;
use memchr::memchr_iter;
use serde::Deserialize;

use crate::text::get_single_char;
use crate::text::is_punctuation;

#[derive(Debug)]
pub enum Morpheme {
    Unk,
    Untagged(String),
    Tagged(TaggedMorpheme),
}

impl IsSegment for Morpheme {
    fn text(&self) -> &str {
        match self {
            Morpheme::Unk => unreachable!(),
            Morpheme::Untagged(data) => data,
            Morpheme::Tagged(tag) => tag.surface,
        }
    }
}

impl Morpheme {
    pub fn to_kata(&self) -> &str {
        match self {
            Morpheme::Unk => unreachable!(),
            Morpheme::Untagged(data) => data,
            Morpheme::Tagged(tag) => tag.feature.l_form,
        }
    }

    pub fn to_full(&self) -> String {
        match self {
            Morpheme::Unk => unreachable!(),
            Morpheme::Untagged(data) => data.clone(),
            Morpheme::Tagged(tag) => format!("{:#?}", tag.feature),
        }
    }

    pub fn pos(&self) -> &str {
        match self {
            Morpheme::Unk => unreachable!(),
            Morpheme::Untagged(_) => "",
            Morpheme::Tagged(tag) => tag.feature.pos1,
        }
    }

    pub fn pos2(&self) -> &str {
        match self {
            Morpheme::Unk => unreachable!(),
            Morpheme::Untagged(_) => "",
            Morpheme::Tagged(tag) => tag.feature.pos2,
        }
    }
}

impl Display for Morpheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Morpheme::Unk => unreachable!(),
            Morpheme::Untagged(data) => write!(f, "{data}"),
            Morpheme::Tagged(tag) => write!(f, "{}", tag.surface),
        }
    }
}

lazy_static! {
    pub static ref ORTH_MAPPING: HashMap<&'static str, &'static str> =
        [("わたし", "私"),].into_iter().collect();
}

impl PartialEq for Morpheme {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tagged(l0), Self::Tagged(r0)) => {
                l0.feature.l_form == r0.feature.l_form
                    || l0.feature.orth_base == r0.feature.orth_base
                    || ORTH_MAPPING.get(l0.feature.orth_base) == Some(&r0.feature.orth_base)
                    || ORTH_MAPPING.get(r0.feature.orth_base) == Some(&l0.feature.orth_base)
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Deserialize<'static> for Morpheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'static>,
    {
        unimplemented!()
    }
}

impl Default for Morpheme {
    fn default() -> Self {
        Self::Unk
    }
}

/// # Safety
///
/// This type is self-referential. As it stands, it can not be [`clone`] as
/// we are not storing any range information. If we stored range data along
/// with the slices, we could manually implement clone.
#[derive(Deserialize)]
pub struct TaggedMorpheme {
    surface: &'static str,
    feature: Feature,
    /// # Safety
    ///
    /// This field MUST come last to prevent undefined behavior when the
    /// struct is dropped, as rust drops structs in order.
    #[serde(skip)]
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

/// https://clrd.ninjal.ac.jp/unidic/faq.html
#[derive(Debug, Deserialize)]
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

pub struct JapaneseTextSegmenter {
    tagger: MeCrab,
}

impl JapaneseTextSegmenter {
    pub fn new() -> Self {
        // TODO: We need to figure out where to get this from.
        let home = std::env::var("HOME").unwrap();

        /*
        let tagger = mecab::Tagger::new(format!(
            "-Ounidic --dicdir="
        ));*/

        let tagger = mecrab::MeCrabBuilder::new()
            .dicdir(Some(PathBuf::from(format!(
                "{home}/Projects/sribich/_/unidic-cwj-202302"
            ))))
            .output_format(OutputFormat::Default)
            .build()
            .unwrap();

        Self { tagger }
    }

    fn line_to_pinned_ranges(line: &str) -> (Pin<Box<[u8]>>, [Range<usize>; 11]) {
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
            end: line.len(),
        };

        (
            Pin::from(line.as_bytes().to_owned().into_boxed_slice()),
            ranges,
        )
    }

    fn resolve_unks(segments: &mut [<Self as TextSegmenter>::Feature], text: &str) {
        let segments_len = segments.len();
        let mut text = text;

        for index in 0..segments_len {
            match &segments[index] {
                Morpheme::Unk => {
                    if index + 1 >= segments_len {
                        segments[index] = Morpheme::Untagged(text.to_owned());
                        continue;
                    }

                    let next_segment = &segments[index + 1];

                    if let Some(pos) =
                        memchr::memmem::find(text.as_bytes(), next_segment.text().as_bytes())
                    {
                        segments[index] = Morpheme::Untagged(text[..pos].trim().to_owned());

                        text = &text[pos..];
                    } else {
                        unreachable!("Failed to resolve UNK. Unable to find morpheme in text");
                    }
                }
                Morpheme::Tagged(tag) => {
                    if let Some(pos) = memchr::memmem::find(text.as_bytes(), tag.surface.as_bytes())
                    {
                        text = &text[(pos + tag.surface.len())..];
                    } else {
                        panic!("Failed to resolve UNK. Morpheme did not match expected text");
                    }
                }
                Morpheme::Untagged(_) => {
                    unreachable!("Untagged values should not exist before UNK resolution")
                }
            }
        }
    }
}

impl TextSegmenter for JapaneseTextSegmenter {
    type Feature = Morpheme;

    fn segment<S: AsRef<str>>(&self, text: S) -> Vec<Self::Feature> {
        let output = self.tagger.parse(text.as_ref()).unwrap();

        println!(
            "{:#?}",
            self.tagger.parse(
                // 暑ければ暑いほど
                // 新鮮館に僕の亜種のセンタ機械豚肉にシャアアをしますこんにちはあGoGoGoこんにちはあ
                "こんにちは"
            )
        );
        // output.morphemes

        // println!("{:#?}", output);
        panic!();

        vec![]
        /*
        let occurrances = memchr_iter(b'\n', output.as_bytes());

        let mut line_start = 0;
        let mut has_unk = false;

        let mut result = Vec::with_capacity(occurrances.clone().count());

        for line_end in occurrances {
            let line = &output[line_start..line_end];

            line_start = line_end + 1; // +1 to skip newline

            if line.is_empty() || line == "BOS" || line == "EOS" {
                continue;
            }

            // We collapse all consequitive UNKs into a single UNK so that
            // we can cleanly map them to their source text.
            if line == "UNK" {
                if !matches!(result.last(), Some(Morpheme::Unk)) {
                    result.push(Morpheme::Unk);
                }

                has_unk = true;

                continue;
            }

            let (data, ranges) = JapaneseTextSegmenter::line_to_pinned_ranges(line);

            #[expect(
                unsafe_code,
                clippy::transmute_bytes_to_str,
                reason = "Too many allocations tanks performance"
            )]
            // SAFETY: This type is owned by `data` which is stored in the resulting `tagged_data`
            //         struct. If the below `matches` is satisfied, then we will return early and
            //         drop `surface` before dropping `data`.
            let surface = unsafe { std::mem::transmute::<&[u8], &'static str>(&data[ranges[0]]) };

            if matches!(get_single_char(surface), Some(c) if is_punctuation(c)) {
                continue;
            }

            #[expect(
                unsafe_code,
                clippy::multiple_unsafe_ops_per_block,
                clippy::transmute_bytes_to_str,
                reason = "See safety comment"
            )]
            // SAFETY: TaggedMorpheme owns its own data in the form of a
            //         `Pin<Box[u8]>`. The box is created from a `String`
            //         which has already been UTF-8 validated, so we can
            //         transmute without worry.
            let tagged_data = unsafe {
                TaggedMorpheme {
                    surface,
                    feature: Feature {
                        pos1: std::mem::transmute::<&[u8], &'static str>(&data[ranges[1]]),
                        pos2: std::mem::transmute::<&[u8], &'static str>(&data[ranges[2]]),
                        pos3: std::mem::transmute::<&[u8], &'static str>(&data[ranges[3]]),
                        pos4: std::mem::transmute::<&[u8], &'static str>(&data[ranges[4]]),
                        c_type: std::mem::transmute::<&[u8], &'static str>(&data[ranges[5]]),
                        c_form: std::mem::transmute::<&[u8], &'static str>(&data[ranges[6]]),
                        l_form: std::mem::transmute::<&[u8], &'static str>(&data[ranges[7]]),
                        lemma: std::mem::transmute::<&[u8], &'static str>(&data[ranges[8]]),
                        orth: std::mem::transmute::<&[u8], &'static str>(&data[ranges[9]]),
                        orth_base: std::mem::transmute::<&[u8], &'static str>(&data[ranges[10]]),
                    },
                    data,
                }
            };

            result.push(Morpheme::Tagged(tagged_data));
        }

        if has_unk {
            JapaneseTextSegmenter::resolve_unks(&mut result, text.as_ref());
        }

        result
         */
    }
}

#[cfg(test)]
mod test {
    use language_pack::segment::TextSegmenter;

    use super::JapaneseTextSegmenter;

    #[test]
    fn mecab() {
        let tagger = JapaneseTextSegmenter::new();

        println!("{:#?}", tagger.segment("買い殺しと"));
        println!("{:#?}", tagger.segment("飼い殺しとどちら"));

        println!("{:#?}", tagger.segment("裏野の"));
        println!("{:#?}", tagger.segment("麗乃の"));

        println!("{:#?}", tagger.segment("一日 "));
        println!("{:#?}", tagger.segment("一日"));
    }
}
