/// # Mecab Types
///
///   - 名詞 - Noun
///     - 固有名詞 - Proper noun
///       - 人名 - Name
///         - 名 - Given Name (First)
///         - 姓 - Family Name
///         - 一般 - General (??)
///       - 地名 - Place Name
///         - 国 - Region/State
///         - 一般 - General (??)
///       - 一般 - General
///     - 普通名詞 - Common Noun
///       - 一般 - General
///       - 副詞可能 - Adverb possible
///       - 助数詞可能 - Counter possible
///       - サ変可能 - Irregular
///       - 形状詞可能 - Adjectival (na-) noun
///       - サ変形状詞可能 - Iregular adjectival noun
///     - 数詞 - Numeral
///     - 助動詞語幹 - ???
///   - 形容詞 - Adjective
///     - 非自立可能 - ???
///     - 一般 - General (??)
///   - 動詞 - Verb
///     - 非自立可能 - ???
///     - 一般 - General (??)
///   - 副詞 - Adverb
use std::fs::read_to_string;
/// # Mecab Types
///
///   - 名詞 - Noun
///     - 固有名詞 - Proper noun
///       - 人名 - Name
///         - 名 - Given Name (First)
///         - 姓 - Family Name
///         - 一般 - General (??)
///       - 地名 - Place Name
///         - 国 - Region/State
///         - 一般 - General (??)
///       - 一般 - General
///     - 普通名詞 - Common Noun
///       - 一般 - General
///       - 副詞可能 - Adverb possible
///       - 助数詞可能 - Counter possible
///       - サ変可能 - Irregular
///       - 形状詞可能 - Adjectival (na-) noun
///       - サ変形状詞可能 - Iregular adjectival noun
///     - 数詞 - Numeral
///     - 助動詞語幹 - ???
///   - 形容詞 - Adjective
///     - 非自立可能 - ???
///     - 一般 - General (??)
///   - 動詞 - Verb
///     - 非自立可能 - ???
///     - 一般 - General (??)
///   - 副詞 - Adverb
use std::path::Path;

use epub::archive::EpubArchive;
use epub::archive::EpubNode;
use itertools::Itertools;
use mecab::Tagger;
use serde::Deserialize;
use serde::Serialize;
use transcription::{
    Segment,
    Transcription,
    TranscriptionContext,
    // whisper::{DtwMode, DtwModelPreset, WhisperContext, WhisperContextParameters},
};

use crate::splitting;

pub struct JapaneseTranscriptionContext {}

impl TranscriptionContext for JapaneseTranscriptionContext {
    fn language(&self) -> &'static str {
        "ja"
    }
}

impl From<JapaneseTranscriptionContext> for Box<dyn TranscriptionContext> {
    fn from(value: JapaneseTranscriptionContext) -> Self {
        Box::new(value)
    }
}

#[derive(Debug, Clone, Serialize)]
struct Test(Vec<(String, String, u64)>);

// https://clrd.ninjal.ac.jp/unidic/faq.html
#[derive(Debug, Clone)]
pub struct Word {
    pub ts: Option<u64>,
    pub unit: String,

    /// Part of speech field 1 -- Most general
    pub pos1: String,
    /// Part of speech field 2
    pub pos2: String,
    /// Part of speech field 3
    pub pos3: String,
    /// Part of speech field 4 -- Most specific
    pub pos4: String,
    /// Conjugation type (活用型). Will have a value like 五段-ラ行.
    pub c_type: String,
    /// Conjugation shape (活用形). Will have a value like 連用形-促音便.
    pub c_form: String,
    /// 語彙素読み, lemma reading. The reading of the lemma in katakana, this
    /// uses the same format as the kana field, not pron.
    pub l_form: String,
    /// 語彙素（＋語彙素細分類）. The lemma is a non-inflected "dictionary form"
    /// of a word. UniDic lemmas sometimes include extra info or have unusual
    /// forms, like using katakana for some place names.
    pub lemma: String,
    /// 書字形出現形, the word as it appears in text, this appears to be
    /// identical to the surface.
    pub orth: String,
    /// 発音形出現形, pronunciation. This is similar to kana except that long
    /// vowels are indicated with a ー, so 講師 is こーし.
    pub pron: String,
    /// The uninflected form of the word in its current written form.
    /// For example, for 彷徨った the lemma is さ迷う but the orthBase is
    /// 彷徨う.
    pub orth_base: String,
    /// 発音形基本形, the pronunciation of the base form. Like pron for the
    /// lemma or orthBase.
    pub pron_base: String,
    /// 語種, word type. Etymological category. In order of frequency, 和, 固,
    /// 漢, 外, 混, 記号, 不明. Defined for all dictionary words, blank for
    /// unks.
    pub goshu: String,
    /// 語頭変化化型, "i" is for "initial". This is the type of initial
    /// transformation the word undergoes when combining, for example 兵 is
    /// へ半濁 because it can be read as べい in combination. This is available
    /// for <2% of entries.
    pub i_type: String,
    /// 語頭変化形, this is the initial form of the word in context, such as
    /// 基本形 or 半濁音形.
    pub i_form: String,
    /// 語末変化化型, "f" is for "final", but otherwise as iType. For example
    /// 医学 is ク促 because it can change to いがっ (apparently). This is
    /// available for <0.1% of entries.
    pub f_type: String,
    /// 語末変化形, as iForm but for final transformations.
    pub f_form: String,
    /// 語頭変化結合型, initial change fusion type. Describes phonetic change at
    /// the start of the word in counting expressions. Only available for a few
    /// hundred entries, mostly numbers. Values are N followed by a letter or
    /// number; most entries with this value are numeric.
    pub i_con_type: String,
    /// 語末変化結合型, final change fusion type. This is also used for counting
    /// expressions, and like iConType it is only available for a few hundred
    /// entries. Unlike iConType the values are very complicated, like
    /// B1S6SjShS,B1S6S8SjShS.
    pub f_con_type: String,
    /// Not entirely clear what this is, seems to have some overlap with POS.
    pub r#type: String,
    /// 読みがな, this is the typical representation of a word in kana, unlike
    /// pron. 講師 is こうし.
    pub kana: String,
    /// 仮名形基本形, this is the typical kana representation of the lemma.
    pub kana_base: String,
    /// 語形出現形, seems to be the same as pron.
    pub form: String,
    /// 語形基本形 seems to be the same as pronBase.
    pub form_base: String,
    /// Accent type. This is a (potentially) comma-separated field which has the
    /// number of the mora taking the accent in 標準語 (standard language). When
    /// there are multiple values, more common accent patterns come first.
    pub a_type: String,
    /// This describes how the accent shifts when the word is used in a counter
    /// expression. It uses complicated notation.
    pub a_con_type: String,
    /// Presumably accent related but unclear use. Available for <25% of entries
    /// and only has 6 non-default values.
    pub a_mod_type: String,
}

#[derive(Debug)]
struct TimestampedNode {
    t0: Option<u64>,
    t1: Option<u64>,
    node: EpubNode,
    words: Vec<Word>,
}

#[derive(Debug)]
struct TimestampedNodeB {
    t0: Option<u64>,
    t1: Option<u64>,
    node: EpubNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SegmentKind {
    Chapter,
    Paragraph,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampedSegments {
    pub t0: Option<u64>,
    pub t1: Option<u64>,
    pub kind: SegmentKind,
    pub segments: Vec<splitting::Segment>,
}

impl JapaneseTranscriptionContext {
    pub fn transcribe(&self, transcription_path: &Path, novel_path: &Path) -> String {
        let text = read_to_string(transcription_path).unwrap();

        let transcription: Transcription = serde_json::from_str(&text).unwrap();
        let words = self.fit_transcription(transcription);

        let text = EpubArchive::open(novel_path).unwrap().rendered;

        self.timestamp(text, words)
    }

    pub fn fit_new(
        &self,
        chapters: Vec<(String, Vec<EpubNode>)>,
        audio: String,
    ) -> Vec<TimestampedSegments> {
        let transcription: Transcription = serde_json::from_str(&audio).unwrap();
        let words = self.fit_transcription(transcription);

        self.timestamp_new(chapters, words)
            .into_iter()
            .map(|it| TimestampedSegments {
                t0: it.t0,
                t1: it.t1,
                kind: match it.node {
                    EpubNode::Chapter(_) => SegmentKind::Chapter,
                    EpubNode::Header(_) => SegmentKind::Chapter,
                    EpubNode::Paragraph(_) => SegmentKind::Paragraph,
                },
                segments: match it.node {
                    EpubNode::Chapter(text) => splitting::segment_text(text),
                    EpubNode::Header(text) => splitting::segment_text(text),
                    EpubNode::Paragraph(text) => splitting::segment_text(text),
                },
            })
            .collect::<Vec<_>>()
    }

    pub fn fit(&self, text: String, audio: String) -> String {
        let transcription: Transcription = serde_json::from_str(&audio).unwrap();
        let words = self.fit_transcription(transcription);

        self.timestamp(text, words)
    }

    fn fit_transcription(&self, transcription: Transcription) -> Vec<FitWord> {
        let tagger = Tagger::new("-Ounidic --dicdir=~/Projects/sribich/_/unidic-cwj-202302");

        let mut list = vec![];

        for mut segment in transcription.segments {
            /// Hallucination
            if segment.words.is_empty() {
                continue;
            }

            // The model will sometimes put significantly more emphasis on
            // extended vowel sounds than actually exists and mecab will
            // often parse them individually, creating a long chain of
            // vowels that break our lookahead algorithm.
            Self::remove_duplicate_chunks(&mut segment);

            let output = tagger.parse_str(segment.text.clone().into_bytes());

            let mut morphemes = vec![];

            for line in output.split('\n') {
                if line == "UNK" || line == "EOS" || line == "BOS" || line.is_empty() {
                    continue;
                }

                let mut items = line.split(',');

                morphemes.push(Word {
                    ts: None,
                    unit: items.nth(0).unwrap_or("").to_string(),
                    pos1: items.nth(0).unwrap_or("").to_string(),
                    pos2: items.nth(0).unwrap_or("").to_string(),
                    pos3: items.nth(0).unwrap_or("").to_string(),
                    pos4: items.nth(0).unwrap_or("").to_string(),
                    c_type: items.nth(0).unwrap_or("").to_string(),
                    c_form: items.nth(0).unwrap_or("").to_string(),
                    l_form: items.nth(0).unwrap_or("").to_string(),
                    lemma: items.nth(0).unwrap_or("").to_string(),
                    orth: items.nth(0).unwrap_or("").to_string(),
                    pron: items.nth(0).unwrap_or("").to_string(),
                    orth_base: items.nth(0).unwrap_or("").to_string(),
                    pron_base: items.nth(0).unwrap_or("").to_string(),
                    goshu: items.nth(0).unwrap_or("").to_string(),
                    i_type: items.nth(0).unwrap_or("").to_string(),
                    i_form: items.nth(0).unwrap_or("").to_string(),
                    f_type: items.nth(0).unwrap_or("").to_string(),
                    f_form: items.nth(0).unwrap_or("").to_string(),
                    i_con_type: items.nth(0).unwrap_or("").to_string(),
                    f_con_type: items.nth(0).unwrap_or("").to_string(),
                    r#type: items.nth(0).unwrap_or("").to_string(),
                    kana: items.nth(0).unwrap_or("").to_string(),
                    kana_base: items.nth(0).unwrap_or("").to_string(),
                    form: items.nth(0).unwrap_or("").to_string(),
                    form_base: items.nth(0).unwrap_or("").to_string(),
                    a_type: items.nth(0).unwrap_or("").to_string(),
                    a_con_type: items.nth(0).unwrap_or("").to_string(),
                    a_mod_type: items.nth(0).unwrap_or("").to_string(),
                });
            }

            let mut char_idx = 0;

            'outer: for mut morpheme in morphemes {
                let morpheme_length = morpheme.unit.chars().count();

                // println!(
                //     "Searching for {:?} -- char_idx={}/{} | morphene_length={}",
                //     morpheme.unit,
                //     char_idx,
                //     segment.words.len(),
                //     morphene_length
                // );

                'inner: for i in char_idx..=(segment.words.len() - morpheme_length) {
                    let words = &segment.words[i..i + morpheme_length];
                    let word_str = words.iter().join("");

                    // println!("  - {:?} ({})", strtest, i);

                    if morpheme.unit == word_str {
                        if i != char_idx {
                            list.push(FitWord {
                                text: segment.words[char_idx..i].iter().join(""),
                                t0: segment.words[char_idx].start.map(|it| (it * 1000.0) as u64),
                                t1: segment.words[i - 1].end.map(|it| (it * 1000.0) as u64),
                                word: None,
                            });
                        }

                        morpheme.ts = segment.words[i].start.map(|it| (it * 1000.0) as u64);

                        list.push(FitWord {
                            text: word_str,
                            t0: segment.words[i].start.map(|it| (it * 1000.0) as u64),
                            t1: segment.words[i + morpheme_length - 1]
                                .end
                                .map(|it| (it * 1000.0) as u64),
                            word: Some(morpheme),
                        });

                        char_idx = i + morpheme_length;
                        continue 'outer;
                    }
                }

                panic!("No match found");
            }
        }

        for i in 0..list.len() - 1 {
            if let Some(first) = list[i].t1
                && let Some(second) = list[i + 1].t0
            {
                if second > first {
                    list[i].t1 = Some(first + (second - first) / 2);
                }
            }
        }

        list
    }

    fn timestamp_new(
        &self,
        chapters: Vec<(String, Vec<EpubNode>)>,
        mut words: Vec<FitWord>,
    ) -> Vec<TimestampedNodeB> {
        let tagger = Tagger::new("-Ounidic --dicdir=~/Projects/sribich/_/unidic-cwj-202302");

        let mut chapters = chapters
            .into_iter()
            .map(|it| it.1)
            .concat()
            .into_iter()
            .filter(|it| {
                if let EpubNode::Paragraph(data) = it
                    && data.is_empty()
                {
                    return false;
                }

                true
            })
            .map(|it| {
                let text = match &it {
                    EpubNode::Chapter(text) => text,
                    EpubNode::Header(text) => text,
                    EpubNode::Paragraph(text) => text,
                };

                let result = tagger.parse_str(&**text);

                let mut words = vec![];

                for result_line in result.split('\n') {
                    let word = match Self::text_to_word(result_line) {
                        Some(word) => word,
                        None => continue,
                    };

                    words.push(word);
                }

                TimestampedNode {
                    t0: None,
                    t1: None,
                    node: it,
                    words,
                }
            })
            .collect::<Vec<_>>();

        let start = self.find_start_new(&chapters, &words);

        let mut start_idx = start;

        for item in &mut chapters {
            Self::dowork_new(item, &mut words, &mut start_idx);
        }

        chapters
            .into_iter()
            .map(|it| TimestampedNodeB {
                t0: it.t0,
                t1: it.t1,
                node: it.node,
            })
            .collect::<Vec<_>>()

        /*




        meta_text
            .into_iter()
            .map(|item| match item {
                TextLine::Empty => "".to_owned(),
                TextLine::Multiple(lines) => {
                    lines
                        .iter()
                        .enumerate()
                        .map(|(i, line)| {
                            let cont_marker = if i == lines.len() - 1 { "" } else { "-" };

                            let timestamp = if line.t0.is_none() || line.t1.is_none() {
                                "UNK --> UNK".to_owned()
                            } else {
                                let start = line.t0.unwrap() as usize;
                                let end = line.t1.unwrap() as usize;
                                /*
                                let start =
                                    usize::from_str_radix(&format!("{}0", line.t0.unwrap()), 10)
                                        .map_err(|e| e)
                                        .unwrap();
                                let end =
                                    usize::from_str_radix(&format!("{}0", line.t1.unwrap()), 10)
                                        .map_err(|e| e)
                                        .unwrap();
                                 */

                                format!("{} --> {}", self.convert(start), self.convert(end))
                            };

                            let mut word_iter = line.words.iter();
                            let mut result = vec![];

                            if line.words.len() > 0 {
                                let first = word_iter.nth(0).unwrap();
                                result.push((
                                    first.unit.clone(),
                                    first.orth_base.clone(),
                                    first.ts.unwrap_or(0),
                                ));

                                for word in word_iter {
                                    if
                                    /* word.pos1 == "助動詞" || */
                                    (word.pos1 == "接尾辞")
                                    // || (word.pos1 == "助詞" && word._1 == "接続助詞")
                                    {
                                        let item = result.last_mut().unwrap();
                                        item.0 = format!("{}{}", item.0, word.unit);
                                    } else {
                                        result.push((
                                            word.unit.clone(),
                                            word.orth_base.clone(),
                                            word.ts.unwrap_or(0),
                                        ));
                                    }
                                }
                            }

                            /*
                            let txt = Test(
                                line.words
                                    .iter()
                                    .map(|word| (word.unit.clone(), word.orth_base.clone()))
                                    .collect::<Vec<_>>(),
                            );
                            */
                            let txt = Test(result);
                            let encode = serde_json::to_string(&txt).unwrap();

                            format!(
                                "{}{}\n{}",
                                cont_marker, timestamp, encode /* line.text */
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n\n")

                    /*
                    let first = lines.first().unwrap();
                    let last = lines.last().unwrap();

                    let line = if first.t0.is_none() || last.t1.is_none() {
                        "UNK --> UNK".to_owned()
                    } else {
                        let start = usize::from_str_radix(&format!("{}0", first.t0.unwrap()), 10)
                            .map_err(|e| e)
                            .unwrap();
                        let end = usize::from_str_radix(&format!("{}0", last.t1.unwrap()), 10)
                            .map_err(|e| e)
                            .unwrap();

                        format!("{} --> {}", self.convert(start), self.convert(end))
                    };

                    format!(
                        "{}\n{}",
                        line,
                        lines
                            .iter()
                            .map(|it| it.text.clone())
                            .collect::<Vec<_>>()
                            .join("")
                    )
                    */
                },
                TextLine::WithMeta(meta) => {
                    let line = if meta.t1.is_none() || meta.t0.is_none() {
                        "UNK --> UNK".to_owned()
                    } else {
                        let start = meta.t0.unwrap() as usize;
                        let end = meta.t1.unwrap() as usize;

                        /*
                        let start = usize::from_str_radix(&format!("{}", t0), 10)
                            .map_err(|e| e)
                            .unwrap();
                        let end = usize::from_str_radix(&format!("{}", t1), 10)
                            .map_err(|e| e)
                            .unwrap();
                         */

                        format!("{} --> {}", self.convert(start), self.convert(end))
                    };

                    let mut word_iter = meta.words.iter();
                    let mut result = vec![];

                    if meta.words.len() > 0 {
                        let first = word_iter.nth(0).unwrap();
                        result.push((
                            first.unit.clone(),
                            first.orth_base.clone(),
                            first.ts.unwrap_or(0),
                        ));

                        for word in word_iter {
                            if
                            //word.pos1 == "助動詞" ||
                            word.pos1 == "接尾辞"
                            // || (word.pos1 == "助詞" && word._1 == "接続助詞")
                            {
                                let item = result.last_mut().unwrap();
                                item.0 = format!("{}{}", item.0, word.unit);
                            } else {
                                result.push((
                                    word.unit.clone(),
                                    word.orth_base.clone(),
                                    word.ts.unwrap_or(0),
                                ));
                            }
                        }
                    }

                    /*
                    let txt = Test(
                        line.words
                            .iter()
                            .map(|word| (word.unit.clone(), word.orth_base.clone()))
                            .collect::<Vec<_>>(),
                    );
                    */
                    let txt = Test(result);
                    let encode = serde_json::to_string(&txt).unwrap();

                    format!("{}\n{}", line, encode /* meta.text */)
                },
            })
            .collect::<Vec<_>>()
            .join("\n")
         */
    }

    fn timestamp(&self, text: String, mut words: Vec<FitWord>) -> String {
        let tagger = Tagger::new("-Ounidic --dicdir=~/Projects/sribich/_/unidic-cwj-202302");

        let mut meta_text = text
            .split('\n')
            .map(|line| {
                if line.is_empty() {
                    return TextLine::Empty;
                }

                let sublines = Self::split_lines(line)
                    .iter()
                    .map(|subline| {
                        let result = tagger.parse_str(*subline);

                        let mut words = vec![];

                        for result_line in result.split('\n') {
                            let word = match Self::text_to_word(result_line) {
                                Some(word) => word,
                                None => continue,
                            };

                            words.push(word);
                        }

                        Line {
                            text: (*subline).to_owned(),
                            words,
                            t0: None,
                            t1: None,
                        }
                    })
                    .collect::<Vec<_>>();

                match sublines.len() {
                    1 => TextLine::WithMeta(sublines[0].clone()),
                    _ => TextLine::Multiple(sublines),
                }
            })
            .collect::<Vec<_>>();

        let start = self.find_start(&meta_text, &words);

        let mut start_idx = start;

        for mut item in &mut meta_text {
            match &mut item {
                TextLine::Empty => continue,
                TextLine::Multiple(lines) => {
                    for line in lines {
                        Self::dowork(line, &mut words, &mut start_idx);
                    }
                }
                TextLine::WithMeta(line) => Self::dowork(line, &mut words, &mut start_idx),
            }
        }

        meta_text
            .into_iter()
            .map(|item| match item {
                TextLine::Empty => String::new(),
                TextLine::Multiple(lines) => {
                    lines
                        .iter()
                        .enumerate()
                        .map(|(i, line)| {
                            let cont_marker = if i == lines.len() - 1 { "" } else { "-" };

                            let timestamp = if line.t0.is_none() || line.t1.is_none() {
                                "UNK --> UNK".to_owned()
                            } else {
                                let start = line.t0.unwrap() as usize;
                                let end = line.t1.unwrap() as usize;
                                /*
                                let start =
                                    usize::from_str_radix(&format!("{}0", line.t0.unwrap()), 10)
                                        .map_err(|e| e)
                                        .unwrap();
                                let end =
                                    usize::from_str_radix(&format!("{}0", line.t1.unwrap()), 10)
                                        .map_err(|e| e)
                                        .unwrap();
                                 */

                                format!("{} --> {}", self.convert(start), self.convert(end))
                            };

                            let mut word_iter = line.words.iter();
                            let mut result = vec![];

                            if !line.words.is_empty() {
                                let first = word_iter.nth(0).unwrap();
                                result.push((
                                    first.unit.clone(),
                                    first.orth_base.clone(),
                                    first.ts.unwrap_or(0),
                                ));

                                for word in word_iter {
                                    if
                                    /* word.pos1 == "助動詞" || */
                                    word.pos1 == "\u{63a5}\u{5c3e}\u{8f9e}"
                                    // || (word.pos1 == "助詞" && word._1 == "接続助詞")
                                    {
                                        let item = result.last_mut().unwrap();
                                        item.0 = format!("{}{}", item.0, word.unit);
                                    } else {
                                        result.push((
                                            word.unit.clone(),
                                            word.orth_base.clone(),
                                            word.ts.unwrap_or(0),
                                        ));
                                    }
                                }
                            }

                            /*
                            let txt = Test(
                                line.words
                                    .iter()
                                    .map(|word| (word.unit.clone(), word.orth_base.clone()))
                                    .collect::<Vec<_>>(),
                            );
                            */
                            let txt = Test(result);
                            let encode = serde_json::to_string(&txt).unwrap();

                            format!("{cont_marker}{timestamp}\n{encode}" /* line.text */)
                        })
                        .collect::<Vec<_>>()
                        .join("\n\n")

                    /*
                    let first = lines.first().unwrap();
                    let last = lines.last().unwrap();

                    let line = if first.t0.is_none() || last.t1.is_none() {
                        "UNK --> UNK".to_owned()
                    } else {
                        let start = usize::from_str_radix(&format!("{}0", first.t0.unwrap()), 10)
                            .map_err(|e| e)
                            .unwrap();
                        let end = usize::from_str_radix(&format!("{}0", last.t1.unwrap()), 10)
                            .map_err(|e| e)
                            .unwrap();

                        format!("{} --> {}", self.convert(start), self.convert(end))
                    };

                    format!(
                        "{}\n{}",
                        line,
                        lines
                            .iter()
                            .map(|it| it.text.clone())
                            .collect::<Vec<_>>()
                            .join("")
                    )
                    */
                }
                TextLine::WithMeta(meta) => {
                    let line = if meta.t1.is_none() || meta.t0.is_none() {
                        "UNK --> UNK".to_owned()
                    } else {
                        let start = meta.t0.unwrap() as usize;
                        let end = meta.t1.unwrap() as usize;

                        /*
                        let start = usize::from_str_radix(&format!("{}", t0), 10)
                            .map_err(|e| e)
                            .unwrap();
                        let end = usize::from_str_radix(&format!("{}", t1), 10)
                            .map_err(|e| e)
                            .unwrap();
                         */

                        format!("{} --> {}", self.convert(start), self.convert(end))
                    };

                    let mut word_iter = meta.words.iter();
                    let mut result = vec![];

                    if !meta.words.is_empty() {
                        let first = word_iter.nth(0).unwrap();
                        result.push((
                            first.unit.clone(),
                            first.orth_base.clone(),
                            first.ts.unwrap_or(0),
                        ));

                        for word in word_iter {
                            if
                            //word.pos1 == "助動詞" ||
                            word.pos1 == "\u{63a5}\u{5c3e}\u{8f9e}"
                            // || (word.pos1 == "助詞" && word._1 == "接続助詞")
                            {
                                let item = result.last_mut().unwrap();
                                item.0 = format!("{}{}", item.0, word.unit);
                            } else {
                                result.push((
                                    word.unit.clone(),
                                    word.orth_base.clone(),
                                    word.ts.unwrap_or(0),
                                ));
                            }
                        }
                    }

                    /*
                    let txt = Test(
                        line.words
                            .iter()
                            .map(|word| (word.unit.clone(), word.orth_base.clone()))
                            .collect::<Vec<_>>(),
                    );
                    */
                    let txt = Test(result);
                    let encode = serde_json::to_string(&txt).unwrap();

                    format!("{line}\n{encode}" /* meta.text */)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn text_to_word(text: &str) -> Option<Word> {
        if text == "UNK" || text == "EOS" || text == "BOS" || text.is_empty() {
            return None;
        }

        let mut items = text.split(',');

        Some(Word {
            ts: None,
            unit: items.nth(0).unwrap_or("").to_string(),
            pos1: items.nth(0).unwrap_or("").to_string(),
            pos2: items.nth(0).unwrap_or("").to_string(),
            pos3: items.nth(0).unwrap_or("").to_string(),
            pos4: items.nth(0).unwrap_or("").to_string(),
            c_type: items.nth(0).unwrap_or("").to_string(),
            c_form: items.nth(0).unwrap_or("").to_string(),
            l_form: items.nth(0).unwrap_or("").to_string(),
            lemma: items.nth(0).unwrap_or("").to_string(),
            orth: items.nth(0).unwrap_or("").to_string(),
            pron: items.nth(0).unwrap_or("").to_string(),
            orth_base: items.nth(0).unwrap_or("").to_string(),
            pron_base: items.nth(0).unwrap_or("").to_string(),
            goshu: items.nth(0).unwrap_or("").to_string(),
            i_type: items.nth(0).unwrap_or("").to_string(),
            i_form: items.nth(0).unwrap_or("").to_string(),
            f_type: items.nth(0).unwrap_or("").to_string(),
            f_form: items.nth(0).unwrap_or("").to_string(),
            i_con_type: items.nth(0).unwrap_or("").to_string(),
            f_con_type: items.nth(0).unwrap_or("").to_string(),
            r#type: items.nth(0).unwrap_or("").to_string(),
            kana: items.nth(0).unwrap_or("").to_string(),
            kana_base: items.nth(0).unwrap_or("").to_string(),
            form: items.nth(0).unwrap_or("").to_string(),
            form_base: items.nth(0).unwrap_or("").to_string(),
            a_type: items.nth(0).unwrap_or("").to_string(),
            a_con_type: items.nth(0).unwrap_or("").to_string(),
            a_mod_type: items.nth(0).unwrap_or("").to_string(),
        })
    }

    fn split_lines(text: &str) -> Vec<&str> {
        let mut quoted = false;
        let mut lines = vec![];

        let mut start = 0;

        for (index, c) in text.chars().enumerate() {
            match c {
                '「' => quoted = true,
                '」' => quoted = false,
                '。' | '？' | '！' if !quoted => {
                    let mut indices = text.char_indices();

                    let start_idx = indices.nth(start).unwrap().0;
                    let end_idx = indices.nth(index - start - 1).unwrap();

                    lines.push(&text[start_idx..(end_idx.0 + end_idx.1.len_utf8())]);
                    start = index + 1;
                }
                _ => {}
            }
        }

        if start != text.chars().count() {
            let mut indices = text.char_indices();
            let start_idx = indices.nth(start).unwrap().0;

            lines.push(&text[start_idx..]);
        }

        lines
    }

    fn dowork_new(node: &mut TimestampedNode, words: &mut [FitWord], start_idx: &mut usize) {
        for word in &mut node.words {
            let mut matched = false;

            if word.pos1 == "\u{88dc}\u{52a9}\u{8a18}\u{53f7}" {
                matched = true;
                continue;
            }

            for i in 0..10 {
                let word_nth = words.get_mut(*start_idx + i);

                if word_nth.is_none() {
                    continue;
                }

                let word_nth = word_nth.unwrap();

                if word_nth.word.is_none() {
                    continue;
                }

                if word_nth.word.as_ref().unwrap().pos1 == "\u{88dc}\u{52a9}\u{8a18}\u{53f7}" {
                    continue;
                }

                // It's an exact match
                if (word.l_form == word_nth.word.as_ref().unwrap().l_form)
                    || (!word.unit.is_empty() && word.unit == word_nth.word.as_ref().unwrap().unit)
                {
                    matched = true;
                    *start_idx += i + 1;

                    /*
                                                    if i > 1 {
                                                        println!("{i:?}");
                                                        println!(
                                                            "Fault: {:?}",
                                                            (&word_slice[0..i + 1])
                                                                .iter()
                                                                .map(|it| (
                                                                    it.text.clone(),
                                                                    it.word.unit.clone(),
                                                                    it.word._6.clone()
                                                                ))
                                                                .collect::<Vec<_>>()
                                                        );
                                                        println!("Looking For: {:?} ({:?})", word.unit, word._6);
                                                    }
                    */

                    word.ts = word_nth.t0;

                    if node.t0.is_none() {
                        node.t0 = word_nth.t0;
                        node.t1 = word_nth.t1;
                    } else {
                        node.t1 = word_nth.t1;
                    }

                    break;
                }

                // Next token contains entire + some

                // Token contains not enough

                let current_unit = word_nth.word.as_ref().unwrap().unit.clone();

                /*
                let idx = token.text.find(&word.unit);

                                    if (should_print) {
                                        println!("   IDX: {:?}", idx);
                                    }

                                    if let Some(idx) = idx {
                */

                if word.unit.contains(&current_unit) {
                    loop {
                        let next_token = &mut words[*start_idx + i + 1];

                        // FIXME: This shit breaks shit
                        if next_token.word.is_none() {
                            *start_idx += 1;
                            continue;
                        }

                        if next_token.word.as_ref().unwrap().pos1
                            == "\u{88dc}\u{52a9}\u{8a18}\u{53f7}"
                        {
                            *start_idx += 1;
                            continue;
                        }

                        next_token.word.as_mut().unwrap().unit =
                            format!("{}{}", current_unit, next_token.word.as_ref().unwrap().unit);

                        break;
                    }
                }
            }
        }
    }

    fn dowork(line: &mut Line, words: &mut [FitWord], start_idx: &mut usize) {
        for word in &mut line.words {
            let mut matched = false;

            if word.pos1 == "\u{88dc}\u{52a9}\u{8a18}\u{53f7}" {
                matched = true;
                continue;
            }

            for i in 0..10 {
                let word_nth = words.get_mut(*start_idx + i);

                if word_nth.is_none() {
                    continue;
                }

                let word_nth = word_nth.unwrap();

                if word_nth.word.is_none() {
                    continue;
                }

                if word_nth.word.as_ref().unwrap().pos1 == "\u{88dc}\u{52a9}\u{8a18}\u{53f7}" {
                    continue;
                }

                // It's an exact match
                if (word.l_form == word_nth.word.as_ref().unwrap().l_form)
                    || (!word.unit.is_empty() && word.unit == word_nth.word.as_ref().unwrap().unit)
                {
                    matched = true;
                    *start_idx += i + 1;

                    /*
                                                    if i > 1 {
                                                        println!("{i:?}");
                                                        println!(
                                                            "Fault: {:?}",
                                                            (&word_slice[0..i + 1])
                                                                .iter()
                                                                .map(|it| (
                                                                    it.text.clone(),
                                                                    it.word.unit.clone(),
                                                                    it.word._6.clone()
                                                                ))
                                                                .collect::<Vec<_>>()
                                                        );
                                                        println!("Looking For: {:?} ({:?})", word.unit, word._6);
                                                    }
                    */

                    word.ts = word_nth.t0;

                    if line.t0.is_none() {
                        line.t0 = word_nth.t0;
                        line.t1 = word_nth.t1;
                    } else {
                        line.t1 = word_nth.t1;
                    }

                    break;
                }

                // Next token contains entire + some

                // Token contains not enough

                let current_unit = word_nth.word.as_ref().unwrap().unit.clone();

                /*
                let idx = token.text.find(&word.unit);

                                    if (should_print) {
                                        println!("   IDX: {:?}", idx);
                                    }

                                    if let Some(idx) = idx {
                */

                if word.unit.contains(&current_unit) {
                    loop {
                        let next_token = &mut words[*start_idx + i + 1];

                        // FIXME: This shit breaks shit
                        if next_token.word.is_none() {
                            *start_idx += 1;
                            continue;
                        }

                        if next_token.word.as_ref().unwrap().pos1
                            == "\u{88dc}\u{52a9}\u{8a18}\u{53f7}"
                        {
                            *start_idx += 1;
                            continue;
                        }

                        next_token.word.as_mut().unwrap().unit =
                            format!("{}{}", current_unit, next_token.word.as_ref().unwrap().unit);

                        break;
                    }
                }
            }
        }
    }

    // Attempts to find the start of the audio by matching
    fn find_start_new(&self, nodes: &Vec<TimestampedNode>, audio_words: &Vec<FitWord>) -> usize {
        let book_samples = 10;
        let audio_samples_per_book_sample = 3;

        let book_text = nodes;

        let mut audio_idx = 0;

        // book_id, run, word_id
        let mut best_score = (0, 0, 0);

        for (id, node) in nodes.iter().enumerate().take(book_samples) {
            audio_idx = 0;

            for i in 0..audio_samples_per_book_sample {
                let mut run = 0;

                for node_word in &node.words {
                    if run == 0 {
                        let result = audio_words
                            .iter()
                            .skip(audio_idx)
                            .enumerate()
                            .find(|(idx, word)| word.text == node_word.unit);

                        if let Some(res) = result {
                            audio_idx = res.0;
                            run += 1;
                        } else {
                            break;
                        }
                    } else {
                        if node_word.unit == audio_words[audio_idx + run].text {
                            audio_idx += 1;
                            run += 1;
                        } else {
                            println!("{:#?}", id);
                            println!("  {:#?}", run);
                            println!("  {:#?}", audio_idx);
                            println!("  {:#?}", audio_idx - (run - 1));
                            if best_score.1 < run {
                                best_score = (id, run, audio_idx - run);
                            }

                            break;
                        }
                    }
                }
            }
        }

        // println!("{:#?}", nodes[best_score.0].);

        println!("{:#?}", best_score);
        best_score.2

        /*
                // idx, start_id, run
                let mut longest_run = (0, 0, 0);
                let mut word_idx = 0;

                for (id, node) in nodes.iter().enumerate().take(n) {
                    for word in &node.words {
                        if word_idx != 0 {
                        } else {
                            let result = words.iter().enumerate().find(|(idx, word)| {});
                        }
                    }
                }

                for node in nodes {
                    for node_word in &node.words {
                        let result = audio_words
                            .iter()
                            .enumerate()
                            .find(|(idx, word)| word.text == node_word.unit);

                        if let Some(item) = result {
                            return item.0;
                        }
                    }
                }
        */

        // panic!("couldnt find start");
    }

    fn find_start(&self, meta_text: &Vec<TextLine>, words: &Vec<FitWord>) -> usize {
        for text in meta_text {
            match text {
                TextLine::Empty => continue,
                TextLine::Multiple(lines) => {
                    for line in lines {
                        for line_word in &line.words {
                            let result = words
                                .iter()
                                .enumerate()
                                .find(|(idx, word)| word.text == line_word.unit);

                            if let Some(item) = result {
                                return item.0;
                            }
                        }
                    }
                }
                TextLine::WithMeta(line) => {
                    for line in &line.words {
                        let result = words
                            .iter()
                            .enumerate()
                            .find(|(idx, word)| word.text == line.unit);

                        if let Some(item) = result {
                            return item.0;
                        }
                    }
                }
            }
        }

        panic!("couldnt find start");
    }

    fn timestampf(&self, start: &'_ str, end: &'_ str) -> String {
        let start = start.trim();
        let end = end.trim();

        let start = format!("{start}0");
        let end = format!("{end}0");

        let start = usize::from_str_radix(&start, 10)
            .inspect_err(|e| {
                println!("s {start} {end}");
            })
            .unwrap();
        let end = usize::from_str_radix(&end, 10)
            .inspect_err(|e| {
                println!("e {start} {end}");
            })
            .unwrap();

        format!("{} --> {}", self.convert(start), self.convert(end))
    }

    /// num is in ms
    fn convert(&self, timestamp: usize) -> String {
        let milliseconds = timestamp % 1000;
        let seconds = (timestamp / 1000) % 60;
        let minutes = (timestamp / (1000 * 60)) % 60;
        let hours = timestamp / (1000 * 60 * 60);

        // Format the string
        format!("{hours:02}:{minutes:02}:{seconds:02},{milliseconds:03}")
    }

    fn remove_duplicate_chunks(segment: &mut Segment) {
        // Remove mass duplicates
        let mut idx = 0;
        let mut char_info = ('-', 0);
        let mut remove_ranges = vec![];

        for (i, c) in segment.text.chars().enumerate() {
            idx = i;

            if c == char_info.0 {
                char_info.1 += 1;
                continue;
            }

            if char_info.1 > 6 {
                // Allow up to 6 repeats
                let start_idx = i - (char_info.1 - 6);
                // Ranges are not inclusive
                let end_idx = i;

                remove_ranges.push((start_idx, end_idx));
            }

            char_info = (c, 1);
        }

        if char_info.1 > 6 {
            // Allow up to 6 repeats
            let start_idx = idx - (char_info.1 - 6);
            // Ranges are not inclusive
            let end_idx = idx;

            remove_ranges.push((start_idx, end_idx));
        }

        if !remove_ranges.is_empty() {
            remove_ranges.reverse();

            for range in remove_ranges {
                let mut indices = segment.text.char_indices();

                let start = indices.nth(range.0).unwrap().0;
                let end = indices.nth(range.1 - range.0 - 1).unwrap().0;

                segment.text.replace_range(start..end, "");
            }
        }
    }
}

#[derive(Debug, Clone)]
enum TextLine {
    Empty,
    WithMeta(Line),
    Multiple(Vec<Line>),
}

#[derive(Debug, Clone)]
struct Line {
    text: String,
    words: Vec<Word>,
    t0: Option<u64>,
    t1: Option<u64>,
}

#[derive(Clone, Debug)]
struct FitWord {
    text: String,
    t0: Option<u64>,
    t1: Option<u64>,
    word: Option<Word>,
}

/*
私,代名詞,,,,,,ワタクシ,私-代名詞,私,ワタクシ,私,ワタクシ,和,,,,
も,助詞,係助詞,,,,,モ,も,も,モ,も,モ,和,,,,
いや,形状詞,一般,,,,,イヤ,嫌,いや,イヤ,いや,イヤ,和,,,,
だ,助動詞,,,,助動詞-ダ,終止形-一般,ダ,だ,だ,ダ,だ,ダ,和,,,,
！,補助記号,句点,,,,,,！,！,,！,,記号,,,,
と,助詞,格助詞,,,,,ト,と,と,ト,と,ト,和,,,,
同意,名詞,普通名詞,サ変可能,,,,ドウイ,同意,同意,ドーイ,同意,ドーイ,漢,,,,
する,動詞,非自立可能,,,サ行変格,終止形-一般,スル,為る,する,スル,する,スル,和,,,,
。,補助記号,句点,,,,,,。,。,,。,,記号,,,,
けれど,接続詞,,,,,,ケレド,けれど,けれど,ケレド,けれど,ケレド,和,,,,
、,補助記号,読点,,,,,,、,、,,、,,記号,,,,
幼い,形容詞,一般,,,形容詞,連体形-一般,オサナイ,幼い,幼い,オサナイ,幼い,オサナイ,和,,,,
声,名詞,普通名詞,一般,,,,コエ,声,声,コエ,声,コエ,和,コ濁,基本形,,
から,助詞,格助詞,,,,,カラ,から,から,カラ,から,カラ,和,,,,
の,助詞,格助詞,,,,,ノ,の,の,ノ,の,ノ,和,,,,
返事,名詞,普通名詞,サ変可能,,,,ヘンジ,返事,返事,ヘンジ,返事,ヘンジ,漢,ヘ半濁,基本形,,
は,助詞,係助詞,,,,,ハ,は,は,ワ,は,ワ,和,,,,
なかっ,形容詞,非自立可能,,,形容詞,連用形-促音便,ナイ,無い,なかっ,ナカッ,ない,ナイ,和,,,,
た,助動詞,,,,助動詞-タ,終止形-一般,タ,た,た,タ,た,タ,和,,,,
。,補助記号,句点,,,,,,。,。,,。,,記号,,,,
あまり,形状詞,一般,,,,,アマリ,余り,あまり,アマリ,あまり,アマリ,和,,,,
に,助動詞,,,,助動詞-ダ,連用形-ニ,ダ,だ,に,ニ,だ,ダ,和,,,,
暑く,形容詞,一般,,,形容詞,連用形-一般,アツイ,暑い,暑く,アツク,暑い,アツイ,和,,,,
て,助詞,接続助詞,,,,,テ,て,て,テ,て,テ,和,,,,
、,補助記号,読点,,,,,,、,、,,、,,記号,,,,
私,代名詞,,,,,,ワタクシ,私-代名詞,私,ワタクシ,私,ワタクシ,和,,,,
は,助詞,係助詞,,,,,ハ,は,は,ワ,は,ワ,和,,,,
布団,名詞,普通名詞,一般,,,,フトン,布団,布団,フトン,布団,フトン,漢,フ濁,基本形,,
の,助詞,格助詞,,,,,ノ,の,の,ノ,の,ノ,和,,,,
冷たい,形容詞,一般,,,形容詞,連体形-一般,ツメタイ,冷たい,冷たい,ツメタイ,冷たい,ツメタイ,和,,,,
部分,名詞,普通名詞,一般,,,,ブブン,部分,部分,ブブン,部分,ブブン,漢,,,,
を,助詞,格助詞,,,,,ヲ,を,を,オ,を,オ,和,,,,
探し,動詞,一般,,,五段-サ行,連用形-一般,サガス,探す,探し,サガシ,探す,サガス,和,,,,
て,助詞,接続助詞,,,,,テ,て,て,テ,て,テ,和,,,,
願い,動詞,非自立可能,,,五段-ワア行,連用形-一般,ネガウ,願う,願い,ネガイ,願う,ネガウ,和,,,,
まし,助動詞,,,,助動詞-マス,連用形-一般,マス,ます,まし,マシ,ます,マス,和,,,,
た,助動詞,,,,助動詞-タ,終止形-一般,タ,た,た,タ,た,タ,和,,,,
！,補助記号,句点,,,,,,！,！,,！,,記号,,,,
*/
