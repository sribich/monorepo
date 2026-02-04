use mecab::Tagger;
use serde::Serialize;
use tracing::trace;
use typegen::Typegen;

#[derive(Debug, Clone, Serialize, Typegen)]
#[serde(tag = "kind", content = "content")]
pub enum Segment {
    Raw(String),
    Tagged(Word),
}

// https://clrd.ninjal.ac.jp/unidic/faq.html
#[derive(Debug, Clone, Serialize, Typegen)]
pub struct Word {
    /// The original morpheme as it exists in the input.
    surface: String,
    /// The tagged MeCab information.
    feature: Feature,
}

#[derive(Debug, Clone, Serialize, Typegen)]
pub struct Feature {
    /// Part of speech field 1 -- Most general
    pos1: String,
    /// Part of speech field 2
    pos2: String,
    /// Part of speech field 3
    pos3: String,
    /// Part of speech field 4 -- Most specific
    pos4: String,
    /// Conjugation type (活用型). Will have a value like 五段-ラ行.
    /// Inflection type (???)
    c_type: String,
    /// Conjugation shape (活用形). Will have a value like 連用形-促音便.
    /// Inflection form (???)
    c_form: String,
    /// 語彙素読み, lemma reading. The reading of the lemma in katakana, this
    /// uses the same format as the kana field, not pron.
    l_form: String,
    /// 語彙素（＋語彙素細分類）. The lemma is a non-inflected "dictionary form"
    /// of a word. UniDic lemmas sometimes include extra info or have unusual
    /// forms, like using katakana for some place names.
    lemma: String,
    /// 書字形出現形, the word as it appears in text, this appears to be
    /// identical to the surface.
    orth: String,
    /// 発音形出現形, pronunciation. This is similar to kana except that long
    /// vowels are indicated with a ー, so 講師 is こーし.
    pron: String,
    /// The uninflected form of the word in its current written form.
    /// For example, for 彷徨った the lemma is さ迷う but the orthBase is
    /// 彷徨う.
    orth_base: String,
    /// 発音形基本形, the pronunciation of the base form. Like pron for the
    /// lemma or orthBase.
    pron_base: String,
    /// 語種, word type. Etymological category. In order of frequency, 和, 固,
    /// 漢, 外, 混, 記号, 不明. Defined for all dictionary words, blank for
    /// unks.
    goshu: String,
    /// 語頭変化化型, "i" is for "initial". This is the type of initial
    /// transformation the word undergoes when combining, for example 兵 is
    /// へ半濁 because it can be read as べい in combination. This is available
    /// for <2% of entries.
    i_type: String,
    /// 語頭変化形, this is the initial form of the word in context, such as
    /// 基本形 or 半濁音形.
    i_form: String,
    /// 語末変化化型, "f" is for "final", but otherwise as iType. For example
    /// 医学 is ク促 because it can change to いがっ (apparently). This is
    /// available for <0.1% of entries.
    f_type: String,
    /// 語末変化形, as iForm but for final transformations.
    f_form: String,
    /// 語頭変化結合型, initial change fusion type. Describes phonetic change at
    /// the start of the word in counting expressions. Only available for a few
    /// hundred entries, mostly numbers. Values are N followed by a letter or
    /// number; most entries with this value are numeric.
    i_con_type: String,
    /// 語末変化結合型, final change fusion type. This is also used for counting
    /// expressions, and like iConType it is only available for a few hundred
    /// entries. Unlike iConType the values are very complicated, like
    /// B1S6SjShS,B1S6S8SjShS.
    f_con_type: String,
    /// Not entirely clear what this is, seems to have some overlap with POS.
    r#type: String,
    /// 読みがな, this is the typical representation of a word in kana, unlike
    /// pron. 講師 is こうし.
    kana: String,
    /// 仮名形基本形, this is the typical kana representation of the lemma.
    kana_base: String,
    /// 語形出現形, seems to be the same as pron.
    form: String,
    /// 語形基本形 seems to be the same as pronBase.
    form_base: String,
    /// Accent type. This is a (potentially) comma-separated field which has the
    /// number of the mora taking the accent in 標準語 (standard language). When
    /// there are multiple values, more common accent patterns come first.
    a_type: String,
    /// This describes how the accent shifts when the word is used in a counter
    /// expression. It uses complicated notation.
    a_con_type: String,
    /// Presumably accent related but unclear use. Available for <25% of entries
    /// and only has 6 non-default values.
    a_mod_type: String,
}

fn get_tagger() -> Tagger {
    Tagger::new("-Ounidic --dicdir=~/Projects/sribich/_/unidic-cwj-202302")
}

pub fn segment_text<S: AsRef<str>>(line: S) -> Vec<Segment> {
    let tagger = get_tagger();
    let output = tagger.parse_str(line.as_ref().as_bytes());

    let mut morphemes = vec![];
    let mut text = vec![];

    for segment in output.split('\n') {
        /// Mecab
        if segment == "UNK" || segment == "EOS" || segment == "BOS" || segment.is_empty() {
            continue;
        }

        let mut parts = segment.split(',');

        morphemes.push(Word {
            surface: parts.next().unwrap_or("").to_string(),
            feature: Feature {
                pos1: parts.next().unwrap_or("").to_string(),
                pos2: parts.next().unwrap_or("").to_string(),
                pos3: parts.next().unwrap_or("").to_string(),
                pos4: parts.next().unwrap_or("").to_string(),
                c_type: parts.next().unwrap_or("").to_string(),
                c_form: parts.next().unwrap_or("").to_string(),
                l_form: parts.next().unwrap_or("").to_string(),
                lemma: parts.next().unwrap_or("").to_string(),
                orth: parts.next().unwrap_or("").to_string(),
                pron: parts.next().unwrap_or("").to_string(),
                orth_base: parts.next().unwrap_or("").to_string(),
                pron_base: parts.next().unwrap_or("").to_string(),
                goshu: parts.next().unwrap_or("").to_string(),
                i_type: parts.next().unwrap_or("").to_string(),
                i_form: parts.next().unwrap_or("").to_string(),
                f_type: parts.next().unwrap_or("").to_string(),
                f_form: parts.next().unwrap_or("").to_string(),
                i_con_type: parts.next().unwrap_or("").to_string(),
                f_con_type: parts.next().unwrap_or("").to_string(),
                r#type: parts.next().unwrap_or("").to_string(),
                kana: parts.next().unwrap_or("").to_string(),
                kana_base: parts.next().unwrap_or("").to_string(),
                form: parts.next().unwrap_or("").to_string(),
                form_base: parts.next().unwrap_or("").to_string(),
                a_type: parts.next().unwrap_or("").to_string(),
                a_con_type: parts.next().unwrap_or("").to_string(),
                a_mod_type: parts.next().unwrap_or("").to_string(),
            },
        });
    }

    let line_chars = line.as_ref().chars().count();
    let mut char_idx = 0;

    'outer: for morpheme in morphemes {
        let morpheme_length = morpheme.surface.chars().count();

        trace!(
            "Searching for {} \t-- char_idx={}/{} | morphene_length={}",
            morpheme.surface, char_idx, line_chars, morpheme_length
        );
        // println!(
        //     "Searching for {:?} -- char_idx={}/{} | morphene_length={}",
        //     morpheme.unit,
        //     char_idx,
        //     segment.words.len(),
        //     morphene_length
        // );

        'inner: for i in char_idx..=(line_chars - morpheme_length) {
            let mut indices = line.as_ref().char_indices();
            let start_idx = indices.nth(i).map(|it| it.0).unwrap();
            let end_idx = indices.nth(morpheme_length - 1).map(|it| it.0);

            let word = &line.as_ref()[start_idx..end_idx.unwrap_or(line.as_ref().len())];

            if morpheme.surface == word {
                if i != char_idx {
                    let mut old_indices = line.as_ref().char_indices();
                    let old_start = old_indices.nth(char_idx).unwrap().0;

                    text.push(Segment::Raw(line.as_ref()[old_start..start_idx].to_owned()));
                }

                text.push(Segment::Tagged(morpheme));

                char_idx = i + morpheme_length;
                continue 'outer;
            }
        }

        panic!("No match found");
    }

    text
}
