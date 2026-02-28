use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
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
use std::sync::atomic::AtomicUsize;

use epub::archive::EpubArchive;
use epub::archive::EpubNode;
use epub::archive::EpubSegment;
use itertools::Itertools;
use language_pack::CanSegment;
use language_pack::IsSegment;
use language_pack::Segment;
use language_pack::TextSegmenter;
use language_pack::Transcription;
use mecab::Tagger;
use rand::Rng;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;

use crate::segment::JapaneseTextSegmenter;
use crate::segment::Morpheme;
use crate::splitting;
use crate::text::get_single_char;
use crate::text::is_punctuation;

#[derive(Debug)]
enum MatchKind {
    Begin {},
    Exact {
        timing_pos: usize,
        timing_length: usize,
        text_pos: usize,
        text_length: usize,
    },
    Unknown {
        timing_pos: usize,
        timing_length: usize,
        text_pos: usize,
        text_length: usize,
    },
    Empty {
        timing_pos: usize,
        timing_length: usize,
        text_pos: usize,
        text_length: usize,
    },
    Split {
        from_timing_pos: usize,
        from_timing_length: usize,
        from_text_pos: usize,
        from_text_length: usize,

        match_timing_pos: usize,
        match_text_pos: usize,
        match_length: usize,
    },
    End {},
}

pub struct JapaneseTranscriptionContext {}

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
    /// The uninflected form of the word in its current written form.
    /// For example, for 彷徨った the lemma is さ迷う but the orthBase is
    /// 彷徨う.
    pub orth_base: String,
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

struct TimestampedMorpheme {
    pub t0: Option<u64>,
    pub t1: Option<u64>,
    pub morpheme: Morpheme,
}

//---- New Stuff ---------------------------------------------------------------

#[derive(Debug)]
pub struct EbookSegments(Vec<EpubSegment>);

impl EbookSegments {
    pub fn new(data: Vec<EpubSegment>) -> Self {
        Self(data)
    }
}

impl<T> CanSegment<T> for EbookSegments
where
    T: IsSegment + PartialEq + Debug,
{
    type Source = ();

    fn segments(
        &self,
        segmenter: impl TextSegmenter<Feature = T>,
    ) -> impl Iterator<Item = Segment<T, Self::Source>> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(index, item)| {
                segmenter
                    .segment(item.text())
                    .into_iter()
                    .map(move |segment| Segment {
                        data: segment,
                        source: (), // TranscriptionSource { line: line_index },
                    })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

struct SegmentAligner<T, ASource, BSource>
where
    T: IsSegment + Debug,
{
    a: Vec<Segment<T, ASource>>,
    b: Vec<Segment<T, BSource>>,
    options: SegmentAlignerOptions,
    matches: Vec<MatchKind>,
}

struct SegmentAlignerOptions {}

fn directed_range(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let mut start = a;
    let end = b;
    std::iter::from_fn(move || {
        use std::cmp::Ordering::*;
        match start.cmp(&end) {
            Less => {
                start += 1;
                Some(start - 1)
            }
            Equal => None,
            Greater => {
                start -= 1;
                Some(start + 1)
            }
        }
    })
}

impl<T, ASource, BSource> SegmentAligner<T, ASource, BSource>
where
    T: IsSegment + Send + Sync + Debug,
    ASource: Send + Sync,
    BSource: Send + Sync,
{
    pub fn new(
        a: Vec<Segment<T, ASource>>,
        b: Vec<Segment<T, BSource>>,
        options: SegmentAlignerOptions,
    ) -> Self {
        let a_len = a.len();
        let b_len = b.len();

        Self {
            a,
            b,
            options,
            matches: vec![MatchKind::Unknown {
                timing_pos: 0,
                timing_length: a_len,
                text_pos: 0,
                text_length: b_len,
            }],
        }
    }

    fn sort(&mut self) {
        self.matches.sort_by(|a, b| {
            let (a_timing_pos, a_text_pos) = match a {
                MatchKind::Exact {
                    timing_pos,
                    text_pos,
                    ..
                } => (timing_pos, text_pos),
                MatchKind::Unknown {
                    timing_pos,
                    text_pos,
                    ..
                } => (timing_pos, text_pos),
                MatchKind::Empty {
                    timing_pos,
                    text_pos,
                    ..
                } => (timing_pos, text_pos),
                _ => unreachable!(),
            };
            let (b_timing_pos, b_text_pos) = match b {
                MatchKind::Exact {
                    timing_pos,
                    text_pos,
                    ..
                } => (timing_pos, text_pos),
                MatchKind::Unknown {
                    timing_pos,
                    text_pos,
                    ..
                } => (timing_pos, text_pos),
                MatchKind::Empty {
                    timing_pos,
                    text_pos,
                    ..
                } => (timing_pos, text_pos),
                _ => unreachable!(),
            };

            a_timing_pos
                .cmp(b_timing_pos)
                .then(a_text_pos.cmp(b_text_pos))
        });
    }

    fn ensure_chunks_are_aligned(&self) {
        let mut curr_timing = 0;
        let mut curr_text = 0;

        for (idx, item) in self.matches.iter().enumerate() {
            let (timing_pos, timing_length, text_pos, text_length) = match item {
                MatchKind::Exact {
                    timing_pos,
                    timing_length,
                    text_pos,
                    text_length,
                } => (timing_pos, timing_length, text_pos, text_length),
                MatchKind::Unknown {
                    timing_pos,
                    timing_length,
                    text_pos,
                    text_length,
                } => (timing_pos, timing_length, text_pos, text_length),
                MatchKind::Empty {
                    timing_pos,
                    timing_length,
                    text_pos,
                    text_length,
                } => (timing_pos, timing_length, text_pos, text_length),
                _ => unreachable!(),
            };

            if *timing_pos == curr_timing && *text_pos == curr_text {
                curr_timing = timing_pos + timing_length;
                curr_text = text_pos + text_length;
            } else {
                println!("ERROR:");
                println!("PREV:");
                println!("{:#?}", self.matches[idx - 1]);

                println!("CURR:");
                println!("{:#?}", self.matches[idx]);

                println!("NEXT:");
                println!("{:#?}", self.matches[idx + 1]);

                panic!("Mismatch");
            }
        }
    }

    /// DEBUG
    fn print_empties(&self) {
        for (idx, item) in self.matches.iter().enumerate() {
            if let MatchKind::Empty { .. } = item {
                println!("");
                println!("{:#?}", item);
                println!("{:#?}", self.matches[idx - 1]);
                println!("{:#?}", self.matches[idx + 1]);
                println!("");
            }
        }
    }

    pub fn align(mut self) {
        let mut splits = AtomicUsize::new(0);

        let end_timing = self.a.len();
        let end_text = self.b.len();

        for i in 0..1000 {
            self.partial_align(&mut splits);
        }

        self.sort();
        self.ensure_chunks_are_aligned();

        let mut unknowns: HashMap<String, HashMap<String, usize>> = HashMap::new();

        for i in self.matches.iter() {
            match i {
                MatchKind::Unknown {
                    timing_pos,
                    timing_length,
                    text_pos,
                    text_length,
                } => {
                    if *timing_length == 1 && *text_length == 1 {
                        // let timing_value = &self.a[*timing_pos];
                        // let text_value = &self.b[*text_pos];
                        // misheard_map.entry(&)

                        println!(
                            "1 -> 1 = {} -> {}",
                            &self.a[*timing_pos].data.text(),
                            &self.b[*text_pos].data.text()
                        );
                    }
                    if *timing_length == 1 && *text_length == 2 {
                        println!(
                            "1 -> 2 = {} -> {}",
                            &self.a[*timing_pos].data.text(),
                            &self.b[*text_pos..=(text_pos + 1)]
                                .iter()
                                .map(|it| it.data.text())
                                .join("")
                        );
                        // unknowns.entry(self.a[*timing_pos].data.text().to_owned()).and_modify(||);
                    }
                    if *timing_length == 2 && *text_length == 1 {
                        println!(
                            "2 -> 1 = {} -> {}",
                            &self.a[*timing_pos..=(timing_pos + 1)]
                                .iter()
                                .map(|it| it.data.text())
                                .join(""),
                            &self.b[*text_pos].data.text()
                        );
                    }
                }
                _ => {}
            }
        }

        // panic!();
        // println!("{:#?}", self.matches);

        let debug_matches = |m: &MatchKind| match m {
            MatchKind::Exact {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } => {
                println!(
                    "    {:#?}",
                    self.a[*timing_pos..(timing_pos + timing_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
                println!(
                    "    {:#?}",
                    self.b[*text_pos..(text_pos + text_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
            }
            MatchKind::Unknown {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } => {
                println!(
                    "    {:#?}",
                    self.a[*timing_pos..(timing_pos + timing_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
                println!(
                    "    {:#?}",
                    self.b[*text_pos..(text_pos + text_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
            }
            _ => unreachable!(),
        };

        for (idx, i) in self.matches.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            if idx == self.matches.len() - 1 {
                continue;
            }

            match i {
                MatchKind::Unknown {
                    timing_pos,
                    timing_length,
                    text_pos,
                    text_length,
                } => {
                    if *timing_length < 5_usize {
                        println!("============================");
                        println!("  BEFORE");
                        debug_matches(&self.matches[idx - 1]);
                        println!("");
                        println!(
                            "{:#?}",
                            self.a[*timing_pos..(timing_pos + timing_length)]
                                .iter()
                                .map(|x| x.data.text().to_owned())
                                .join("")
                        );
                        println!(
                            "{:#?}",
                            self.b[*text_pos..(text_pos + text_length)]
                                .iter()
                                .map(|x| x.data.text().to_owned())
                                .join("")
                        );
                        println!("");
                        println!("  AFTER");
                        debug_matches(&self.matches[idx + 1]);
                        println!("============================");
                        println!("");
                        println!("");
                    }
                }
                _ => {}
            }
        }

        println!(
            "{:#?}",
            self.matches
                .iter()
                .fold(BTreeMap::<usize, usize>::new(), |mut prev, curr| {
                    match curr {
                        MatchKind::Unknown {
                            timing_pos,
                            timing_length,
                            text_pos,
                            text_length,
                        } => {
                            prev.get_mut(&timing_length)
                                .map(|mut it| *it += 1)
                                .or_else(|| {
                                    prev.insert(*timing_length, 1);
                                    Some(())
                                });
                        }
                        _ => {}
                    }

                    prev
                })
        );

        println!("{:#?}", self.matches.len());

        println!(
            "{:#?}",
            self.matches.iter().fold((0_usize, 0_usize), |prev, curr| {
                match curr {
                    MatchKind::Exact {
                        timing_pos,
                        timing_length,
                        text_pos,
                        text_length,
                    } => (prev.0 + timing_length, prev.1),
                    MatchKind::Unknown {
                        timing_pos,
                        timing_length,
                        text_pos,
                        text_length,
                    } => (prev.0, prev.1 + timing_length),
                    _ => prev,
                }
            })
        );
    }

    fn partial_align(&mut self, splits: &mut AtomicUsize) {
        self.matches.par_iter_mut().for_each(|item| {
            if let MatchKind::Unknown {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } = &item
            {
                let sample = rand::rng().random_range(*timing_pos..(timing_pos + timing_length));
                let timing_segment = &self.a[sample];

                let timing_limit = timing_pos + timing_length;
                let text_limit = text_pos + text_length;

                let waystones = self.b[*text_pos..(text_pos + text_length)]
                    .iter()
                    .positions(|it| it.data == timing_segment.data)
                    .map(|position| {
                        let position = position + text_pos;

                        let mut fwd_matches = 0_usize;
                        let mut rev_matches = 0_usize;

                        for _ in (sample..(timing_pos + timing_length)) {
                            let timing_pos = sample + fwd_matches;
                            let text_pos = position + fwd_matches;

                            if text_pos >= text_limit {
                                break;
                            }

                            if self.a[timing_pos].data == self.b[text_pos].data {
                                fwd_matches += 1;
                            } else {
                                if timing_pos + 1 >= timing_limit {
                                    break;
                                }

                                if self.a[timing_pos + 1].data == self.b[text_pos + 1].data {
                                    fwd_matches += 1;
                                    continue;
                                }

                                break;
                            }
                        }

                        for _ in directed_range(sample, *timing_pos) {
                            if rev_matches > position || rev_matches > sample {
                                break;
                            }

                            let timing_pos = sample - rev_matches;
                            let text_pos = position - rev_matches;

                            if self.a[timing_pos].data == self.b[text_pos].data {
                                rev_matches += 1;
                            } else {
                                // Allow single gap
                                if self.a[timing_pos - 1].data == self.b[text_pos - 1].data {
                                    rev_matches += 1;
                                    continue;
                                }

                                break;
                            }
                        }

                        return (
                            sample - rev_matches.saturating_sub(1),
                            position - rev_matches.saturating_sub(1),
                            fwd_matches + rev_matches.saturating_sub(1),
                        );
                    })
                    .filter(|it| it.2 > 25)
                    .collect::<Vec<_>>();

                assert!(
                    waystones.len() <= 1,
                    "Found text matching at more than 1 position"
                );

                if waystones.len() == 1 {
                    assert!(waystones[0].0 >= *timing_pos);
                    assert!(waystones[0].1 >= *text_pos);
                    assert!(waystones[0].2 <= *timing_length);

                    *item = MatchKind::Split {
                        from_timing_pos: *timing_pos,
                        from_timing_length: *timing_length,
                        from_text_pos: *text_pos,
                        from_text_length: *text_length,
                        match_timing_pos: waystones[0].0,
                        match_text_pos: waystones[0].1,
                        match_length: waystones[0].2,
                    };
                }
            }
        });

        let mut splits = vec![];

        self.matches = std::mem::take(&mut self.matches)
            .into_iter()
            .map(|it| {
                if let MatchKind::Split {
                    from_timing_pos,
                    from_timing_length,
                    from_text_pos,
                    from_text_length,
                    match_timing_pos,
                    match_text_pos,
                    match_length,
                } = it
                {
                    // First Segment
                    if from_timing_pos != match_timing_pos || from_text_pos != match_text_pos {
                        if (match_timing_pos - from_timing_pos == 0)
                            || (match_text_pos - from_text_pos == 0)
                        {
                            splits.push(MatchKind::Empty {
                                timing_pos: from_timing_pos,
                                timing_length: match_timing_pos - from_timing_pos,
                                text_pos: from_text_pos,
                                text_length: match_text_pos - from_text_pos,
                            });
                        } else {
                            splits.push(MatchKind::Unknown {
                                timing_pos: from_timing_pos,
                                timing_length: match_timing_pos - from_timing_pos,
                                text_pos: from_text_pos,
                                text_length: match_text_pos - from_text_pos,
                            });
                        }
                    }

                    // Second Segment
                    if (match_timing_pos + match_length < from_timing_pos + from_timing_length)
                        || (match_text_pos + match_length < from_text_pos + from_text_length)
                    {
                        if match_timing_pos + match_length == from_timing_pos + from_timing_length
                            || match_text_pos + match_length == from_text_pos + from_text_length
                        {
                            splits.push(MatchKind::Empty {
                                timing_pos: match_timing_pos + match_length,
                                timing_length: from_timing_pos + from_timing_length
                                    - match_timing_pos
                                    - match_length,
                                text_pos: match_text_pos + match_length,
                                text_length: from_text_pos + from_text_length
                                    - match_text_pos
                                    - match_length,
                            });
                        } else {
                            splits.push(MatchKind::Unknown {
                                timing_pos: match_timing_pos + match_length,
                                timing_length: from_timing_pos + from_timing_length
                                    - match_timing_pos
                                    - match_length,
                                text_pos: match_text_pos + match_length,
                                text_length: from_text_pos + from_text_length
                                    - match_text_pos
                                    - match_length,
                            });
                        }
                    }

                    MatchKind::Exact {
                        timing_pos: match_timing_pos,
                        timing_length: match_length,
                        text_pos: match_text_pos,
                        text_length: match_length,
                    }
                } else {
                    it
                }
            })
            .collect::<Vec<_>>();

        self.matches.append(&mut splits);
    }
}

/// Creates an aligned _ where a fits within b.
pub fn align_segments<T: IsSegment, ASource, BSource>(
    a: impl Iterator<Item = Segment<T, ASource>>,
    b: impl Iterator<Item = Segment<T, BSource>>,
    options: SegmentAlignerOptions,
) -> ()
where
    T: Send + Sync + Debug,
    ASource: Send + Sync,
    BSource: Send + Sync,
{
    let a = a.collect::<Vec<_>>();
    let b = b.collect::<Vec<_>>();

    let a_len = a.len();
    let b_len = b.len();

    let mut aligner = SegmentAligner::new(a, b, options);

    // aligner.align();

    println!("{:#?}", aligner.align());

    ()
}

impl JapaneseTranscriptionContext {
    pub fn test<CTiming, CText>(&self, text_data: &CText, timing_data: &CTiming)
    where
        CTiming: CanSegment<Morpheme>,
        CTiming::Source: Send + Sync,
        CText: CanSegment<Morpheme>,
        CText::Source: Send + Sync,
    {
        let segmenter = JapaneseTextSegmenter::new();

        let timing_segments = timing_data.segments(&segmenter);
        let text_segments = text_data.segments(&segmenter);

        println!("Timing Segments: {:?}", timing_segments.size_hint());
        println!("Text Segments: {:?}", text_segments.size_hint());

        let timing_segments = timing_segments.filter(|it| match &it.data {
            Morpheme::Unk => true,
            Morpheme::Tagged(tag) => {
                let surface = tag.surface();
                match get_single_char(surface) {
                    Some(c) => !is_punctuation(c),
                    None => true,
                }
            }
        });

        let text_segments = text_segments.filter(|it| match &it.data {
            Morpheme::Unk => true,
            Morpheme::Tagged(tag) => {
                let surface = tag.surface();
                match get_single_char(surface) {
                    Some(c) => !is_punctuation(c),
                    None => true,
                }
            }
        });

        align_segments(timing_segments, text_segments, SegmentAlignerOptions {});

        /*
                // We want to try mapping the audio_segment_list to the segment_list as it should
                // contain the smaller amount of data.
                const N_SAMPLES: usize = 500;

                // Find N random points in audio_segment_list
                let mut samples = [0_usize; N_SAMPLES];

                samples.iter_mut().for_each(|sample| {
                    for i in 0..20 {
                        *sample = rand::rng().random_range(0..audio_segment_list.len());

                        if let Morpheme::Tagged(_) = audio_segment_list[*sample].0 {
                            return;
                        }
                    }

                    panic!("Unable to find waystone");
                });

                // For each sample we want to find the longest sequence that we can find in `segment_list`.
                let mut longest_sequence = samples
                    .iter()
                    .flat_map(|sample| {
                        let audio_segment = audio_segment_list[*sample].0;

                        segment_list
                            .iter()
                            .positions(|item| item.0 == audio_segment)
                            .map(|position| {
                                let mut matches = 0;
                                let mut rev_matches = 0;

                                loop {
                                    let audio_pos = *sample + matches;
                                    let text_pos = position + matches;

                                    // We're at the end of a buffer
                                    if audio_pos >= audio_segment_list.len()
                                        || text_pos >= segment_list.len()
                                    {
                                        break;
                                    }

                                    if audio_segment_list[audio_pos].0 == segment_list[text_pos].0 {
                                        matches += 1;
                                    } else {
                                        // // Allow a single word gap
                                        // if let Some(a) = audio_segment_list.get(audio_pos + 1)
                                        //     && let Some(b) = segment_list.get(text_pos + 1)
                                        // {
                                        //     if a.0 == b.0 {
                                        //         let key = segment_list[text_pos].0.to_string();
                                        //         let val = audio_segment_list[audio_pos].0.to_string();
                                        //         if let Some(v) = map.get_mut(&key) {
                                        //             v.push(val);
                                        //         } else {
                                        //             map.insert(key, vec![val]);
                                        //         }
                                        //         matches += 1;
                                        //         continue;
                                        //     }
                                        // }

                                        break;
                                    }
                                }

                                loop {
                                    let audio_pos = *sample - rev_matches;
                                    let text_pos = position - rev_matches;

                                    // We're at the end of a buffer
                                    if audio_pos == 0 || text_pos == 0 {
                                        if audio_segment_list[audio_pos].0 == segment_list[text_pos].0 {
                                            rev_matches += 1;
                                        }

                                        return (audio_pos + 1, text_pos + 1, matches + rev_matches - 1);
                                    }

                                    if audio_segment_list[audio_pos].0 == segment_list[text_pos].0 {
                                        rev_matches += 1;
                                    } else {
                                        // // Allow a single word gap
                                        // if let Some(a) = audio_segment_list.get(audio_pos - 1)
                                        //     && let Some(b) = segment_list.get(text_pos - 1)
                                        // {
                                        //     if a.0 == b.0 {
                                        //         rev_matches += 1;
                                        //         continue;
                                        //     }
                                        // }

                                        return (audio_pos + 1, text_pos + 1, matches + rev_matches - 1);
                                    }
                                }
                            })
                            .filter(|it| it.2 > 75) // Collect all matches > 75 chars
                            .collect::<Vec<_>>()
                    })
                    .fold(vec![], |mut prev, curr| {
                        let curr = (
                            curr,
                            std::range::Range {
                                start: curr.0,
                                end: curr.0 + curr.2,
                            },
                        );

                        if prev.is_empty() {
                            prev.push(curr);
                            return prev;
                        }

                        let value = prev.iter_mut().find(|prev| {
                            if prev.0.0 < curr.0.0 {
                                prev.1.contains(&curr.0.0)
                            } else {
                                curr.1.contains(&prev.0.0)
                                // prev.1.contains(&(curr.0.0 + curr.0.2))
                            }
                        });

                        // Because of how the algorithm works, if one value is a subset of another
                        // the lowest positions and largest range will always be the winner. The larger
                        // string will ALWAYS contain the full subet.
                        match value {
                            Some((prev, _)) => {
                                prev.0 = std::cmp::min(prev.0, curr.0.0);
                                prev.1 = std::cmp::min(prev.1, curr.0.1);
                                prev.2 = std::cmp::max(prev.2, curr.0.2);
                            }
                            None => prev.push(curr),
                        }

                        return prev;
                    });

                longest_sequence.sort_by(|a, b| Ord::cmp(&a.0, &b.0));

                let resolved_gaps = longest_sequence
                    .iter()
                    .map(|it| it.0)
                    .map(Some)
                    .chain([None])
                    .tuple_windows()
                    .filter_map(|(a, b)| Some((a?, b)))
                    .fold(vec![MatchKind::Begin {}], |mut prev, (curr, next)| {
                        prev.push(MatchKind::Exact {
                            transcription_pos: curr.0,
                            text_pos: curr.1,
                            length: curr.2,
                        });

                        match next {
                            Some(next) => {
                                println!("{:?} {:?}", curr, next);

                                prev.push(MatchKind::Unknown {
                                    transcription_pos: curr.0 + curr.2,
                                    text_pos: curr.1 + curr.2,
                                    length: next.0 - (curr.0 + curr.2),
                                });
                            }
                            None => {
                                prev.push(MatchKind::End {});
                            }
                        }

                        prev
                    });


                // We need to hold an array of mappings...
                // The mapping will be:
                //
                //     [[audio_segment_pos, audio_word_pos], [text_segment_pos, text_word_pos]]

                // Re-iterate over this and try to further resolve the gaps. We should basically run the same
                // algorithm over the gaps to further narrow them down.

                println!("{:#?}", resolved_gaps);

                for gap in &resolved_gaps {
                    match gap {
                        MatchKind::Begin {} => {}
                        MatchKind::Exact { .. } => {}
                        MatchKind::Unknown {
                            transcription_pos,
                            text_pos,
                            length,
                        } => {
                            println!("");
                            println!("{}", audio_segment_list[*transcription_pos..(*transcription_pos + *length)].iter().map(|it| it.0.to_string()).join(""));
                            println!("{}", segment_list[*text_pos..(*text_pos + *length)].iter().map(|it| it.0.to_string()).join(""));
                            println!("");

                            // println!("{}",
                        },
                        MatchKind::End {} => {}
                    }
                }

                // println!("{:#?}", map);

                println!("{}", longest_sequence.iter().fold(0_usize, |prev, curr| { prev + curr.0.2 }));
        */

        panic!();
    }

    pub fn fit_new(
        &self,
        chapters: Vec<(String, Vec<EpubNode>)>,
        audio: String,
    ) -> Vec<TimestampedSegments> {
        let transcription: Transcription = serde_json::from_str(&audio).unwrap();

        println!("{:#?}", transcription);
        panic!();

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

    fn fit_transcription(&self, transcription: Transcription) -> Vec<FitWord> {
        let home = std::env::var("HOME").unwrap();
        let tagger = Tagger::new(format!(
            "-Ounidic --dicdir={}/Projects/sribich/_/unidic-cwj-202302",
            home
        ));

        let segmenter = JapaneseTextSegmenter::new();

        let mut list = vec![];

        for mut line in transcription.lines {
            /// Hallucination
            if line.words.is_empty() {
                continue;
            }

            let output = tagger.parse_str(line.text.as_str());

            let output2 = segmenter.segment(line.text.as_str());

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
                    orth_base: items.nth(0).unwrap_or("").to_string(),
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

                'inner: for i in char_idx..=(line.words.len() - morpheme_length) {
                    let words = &line.words[i..i + morpheme_length];
                    let word_str = words.iter().map(|it| &it.word).join("");

                    // println!("  - {:?} ({})", strtest, i);

                    if morpheme.unit == word_str {
                        if i != char_idx {
                            list.push(FitWord {
                                text: line.words[char_idx..i].iter().map(|it| &it.word).join(""),
                                t0: line.words[char_idx].start.map(|it| (it * 1000.0) as u64),
                                t1: line.words[i - 1].end.map(|it| (it * 1000.0) as u64),
                                word: None,
                            });
                        }

                        morpheme.ts = line.words[i].start.map(|it| (it * 1000.0) as u64);

                        list.push(FitWord {
                            text: word_str,
                            t0: line.words[i].start.map(|it| (it * 1000.0) as u64),
                            t1: line.words[i + morpheme_length - 1]
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
        let home = std::env::var("HOME").unwrap();
        let tagger = Tagger::new(format!(
            "-Ounidic --dicdir={}/Projects/sribich/_/unidic-cwj-202302",
            home
        ));

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
            orth_base: items.nth(0).unwrap_or("").to_string(),
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
