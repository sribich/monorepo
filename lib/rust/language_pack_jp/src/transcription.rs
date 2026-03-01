use core::iter::Iterator;
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
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use epub::archive::EpubSegment;
use itertools::Itertools;
use language_pack::AcceptsTimestamps;
use language_pack::CanSegment;
use language_pack::HasTimestamp;
use language_pack::IsSegment;
use language_pack::Segment;
use language_pack::TextSegmenter;
use rand::Rng;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::hirschberg;
use crate::segment::JapaneseTextSegmenter;
use crate::segment::Morpheme;
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
    Fuzzy {
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
    ReplaceWith {
        new_splits: Vec<MatchKind>,
    },
    Tombstone,
    Split {
        from_timing_pos: usize,
        from_timing_length: usize,
        from_text_pos: usize,
        from_text_length: usize,

        match_timing_pos: usize,
        match_timing_length: usize,
        match_text_pos: usize,
        match_text_length: usize,
    },
    End {},
}

pub struct JapaneseTranscriptionContext {}

#[derive(Debug)]
pub struct EbookSegments(Vec<EpubSegment>);

impl EbookSegments {
    pub fn new(data: Vec<EpubSegment>) -> Self {
        Self(data)
    }
}

impl<T> AcceptsTimestamps<T> for EbookSegments
where
    T: IsSegment + PartialEq + Debug,
{
    fn accept(&mut self, segment: &Segment<T, Self::Source>, timestamp: language_pack::Timestamp) {
        let source = *self.source(segment);
        let segment = &mut self.0[source];

        segment.time = timestamp;
    }
}

impl<T> CanSegment<T> for EbookSegments
where
    T: IsSegment + PartialEq + Debug,
{
    type Source = usize;

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
                        source: index, // (), // TranscriptionSource { line: line_index },
                    })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn source<'a>(&self, segment: &'a Segment<T, Self::Source>) -> &'a Self::Source {
        &segment.source
    }
}

struct SegmentAligner<'a, T, ASource, BSource>
where
    T: IsSegment + Debug,
    ASource: Debug,
    BSource: Debug,
{
    a: &'a Vec<Segment<T, ASource>>,
    b: &'a Vec<Segment<T, BSource>>,
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

impl<'a, T, ASource, BSource> SegmentAligner<'a, T, ASource, BSource>
where
    T: IsSegment + Send + Sync + Debug,
    ASource: Send + Sync + Debug,
    BSource: Send + Sync + Debug,
{
    pub fn new(
        a: &'a Vec<Segment<T, ASource>>,
        b: &'a Vec<Segment<T, BSource>>,
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
                MatchKind::Fuzzy {
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
                MatchKind::Fuzzy {
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
                MatchKind::Fuzzy {
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
    fn print_empty_matches(&self) {
        for (idx, item) in self.matches.iter().enumerate() {
            if let MatchKind::Empty { .. } = item {
                println!("=======================================");
                println!("  Previous Match");
                self.print_match(&self.matches[idx - 1], "    ");
                println!();

                self.print_match(item, "");
                println!();

                println!("  Next match");
                self.print_match(&self.matches[idx + 1], "    ");

                println!("=======================================");
                println!();
            }
        }
    }

    fn print_match(&self, m: &MatchKind, prefix: &'static str) {
        match m {
            MatchKind::Exact {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } => {
                println!(
                    "{prefix}{:#?}",
                    self.a[*timing_pos..(timing_pos + timing_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
                println!(
                    "{prefix}{:#?}",
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
                    "{prefix}{:#?}",
                    self.a[*timing_pos..(timing_pos + timing_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
                println!(
                    "{prefix}{:#?}",
                    self.b[*text_pos..(text_pos + text_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
            }
            MatchKind::Fuzzy {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } => {
                println!(
                    "{prefix}{:#?}",
                    self.a[*timing_pos..(timing_pos + timing_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
                println!(
                    "{prefix}{:#?}",
                    self.b[*text_pos..(text_pos + text_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
            }
            MatchKind::Empty {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } => {
                println!(
                    "{prefix}{:#?}",
                    self.a[*timing_pos..(timing_pos + timing_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
                println!(
                    "{prefix}{:#?}",
                    self.b[*text_pos..(text_pos + text_length)]
                        .iter()
                        .map(|x| x.data.text().to_owned())
                        .join("")
                );
            }
            _ => unreachable!(),
        }
    }

    pub fn print_debug(&self) {
        for (idx, i) in self.matches.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            if idx == self.matches.len() - 1 {
                continue;
            }

            if let MatchKind::Unknown {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } = i
                && *timing_length > 4_usize
            {
                println!("============================");
                println!("  BEFORE");
                self.print_match(&self.matches[idx - 1], "    ");
                println!();
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
                println!();
                println!("  AFTER");
                self.print_match(&self.matches[idx + 1], "    ");
                println!("============================");
                println!();
                println!();
            }
        }

        println!(
            "{:#?}",
            self.matches
                .iter()
                .fold(BTreeMap::<usize, usize>::new(), |mut prev, curr| {
                    if let MatchKind::Unknown { timing_length, .. } = curr {
                        prev.get_mut(timing_length).map(|it| *it += 1).or_else(|| {
                            prev.insert(*timing_length, 1);
                            Some(())
                        });
                    }

                    prev
                })
        );

        println!("{:#?}", self.matches.len());

        println!(
            "{:#?}",
            self.matches.iter().fold((0_usize, 0_usize), |prev, curr| {
                match curr {
                    MatchKind::Unknown { timing_length, .. } => (prev.0, prev.1 + timing_length),
                    MatchKind::Exact { timing_length, .. }
                    | MatchKind::Fuzzy { timing_length, .. } => (prev.0 + timing_length, prev.1),
                    _ => prev,
                }
            })
        );
    }

    fn hirschberg_align(&mut self) {
        self.matches.par_iter_mut().for_each(|x| {
            if let MatchKind::Unknown {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } = &x
            {
                // *timing_length > 30 ||
                if *timing_length < 5 {
                    return;
                }

                let timing = &self.a[*timing_pos..(timing_pos + timing_length)];
                let text = &self.b[*text_pos..(text_pos + text_length)];

                let matches = hirschberg::Config::default().compute(timing, text);

                enum A {
                    None,
                    Matched,
                    Mismatched,
                }

                let mut mode = A::None;
                let mut splits = vec![];

                let mut curr_timing_pos = *timing_pos;
                let mut curr_timing_length = 0;
                let mut curr_text_pos = *text_pos;
                let mut curr_text_length = 0;

                for alignment in matches.alignment() {
                    match alignment {
                        (Some(a), Some(b)) if a == b => {
                            if !matches!(mode, A::Matched) {
                                if !matches!(mode, A::None) {
                                    if curr_timing_length == 0 || curr_text_length == 0 {
                                        splits.push(MatchKind::Empty {
                                            timing_pos: curr_timing_pos,
                                            timing_length: curr_timing_length,
                                            text_pos: curr_text_pos,
                                            text_length: curr_text_length,
                                        });
                                    } else {
                                        splits.push(MatchKind::Unknown {
                                            timing_pos: curr_timing_pos,
                                            timing_length: curr_timing_length,
                                            text_pos: curr_text_pos,
                                            text_length: curr_text_length,
                                        });
                                    }
                                }
                                mode = A::Matched;
                                curr_timing_pos += curr_timing_length;
                                curr_text_pos += curr_text_length;
                                curr_timing_length = 0;
                                curr_text_length = 0;
                            }

                            curr_timing_length += 1;
                            curr_text_length += 1;
                        }
                        (a, b) => {
                            if !matches!(mode, A::Mismatched) {
                                if !matches!(mode, A::None) {
                                    if curr_timing_length == 0 || curr_text_length == 0 {
                                        splits.push(MatchKind::Empty {
                                            timing_pos: curr_timing_pos,
                                            timing_length: curr_timing_length,
                                            text_pos: curr_text_pos,
                                            text_length: curr_text_length,
                                        });
                                    } else {
                                        splits.push(MatchKind::Unknown {
                                            timing_pos: curr_timing_pos,
                                            timing_length: curr_timing_length,
                                            text_pos: curr_text_pos,
                                            text_length: curr_text_length,
                                        });
                                    }
                                }
                                mode = A::Mismatched;
                                curr_timing_pos += curr_timing_length;
                                curr_text_pos += curr_text_length;
                                curr_timing_length = 0;
                                curr_text_length = 0;
                            }

                            if a.is_some() {
                                curr_timing_length += 1;
                            }
                            if b.is_some() {
                                curr_text_length += 1;
                            }
                        }
                    }
                }

                if curr_timing_length == 0 || curr_text_length == 0 {
                    splits.push(MatchKind::Empty {
                        timing_pos: curr_timing_pos,
                        timing_length: curr_timing_length,
                        text_pos: curr_text_pos,
                        text_length: curr_text_length,
                    });
                } else {
                    match mode {
                        A::None => unreachable!(),
                        A::Matched => splits.push(MatchKind::Exact {
                            timing_pos: curr_timing_pos,
                            timing_length: curr_timing_length,
                            text_pos: curr_text_pos,
                            text_length: curr_text_length,
                        }),
                        A::Mismatched => splits.push(MatchKind::Unknown {
                            timing_pos: curr_timing_pos,
                            timing_length: curr_timing_length,
                            text_pos: curr_text_pos,
                            text_length: curr_text_length,
                        }),
                    }
                }

                *x = MatchKind::ReplaceWith { new_splits: splits };
            }
        });

        self.resolve_splits();
        self.sort();
        self.ensure_chunks_are_aligned();
    }

    pub fn align(mut self) -> Vec<MatchKind> {
        for _ in 0..100 {
            self.partial_align(false);
        }

        for _ in 0..25 {
            self.partial_align(true);
        }

        self.sort();
        self.ensure_chunks_are_aligned();

        // self.print_empty_matches();

        self.hirschberg_align();

        for _ in 0..25 {
            self.partial_align(true);
        }

        self.matches.iter_mut().for_each(|x| {
            if let MatchKind::Unknown {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } = &x
            {
                if *timing_length > 5 {
                    return;
                }

                *x = MatchKind::Fuzzy {
                    timing_pos: *timing_pos,
                    timing_length: *timing_length,
                    text_pos: *text_pos,
                    text_length: *text_length,
                };
            }
        });

        self.sort();
        self.ensure_chunks_are_aligned();

        // self.print_debug();

        self.matches
    }

    fn partial_align(&mut self, allow_single_gap: bool) {
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

                let timing_start = timing_pos;
                let timing_limit = timing_pos + timing_length;
                let text_start = text_pos;
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

                                if allow_single_gap
                                    && (self.a[timing_pos + 1].data == self.b[text_pos + 1].data)
                                {
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
                                if timing_pos == 0
                                    || text_pos == 0
                                    || timing_pos - 1 < *timing_start
                                    || text_pos - 1 < *text_start
                                {
                                    break;
                                }

                                if allow_single_gap
                                    && (self.a[timing_pos - 1].data == self.b[text_pos - 1].data)
                                {
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
                    .filter(|it| {
                        if *timing_length > 50 {
                            it.2 > 25
                        } else if *timing_length > 25 {
                            it.2 > 10.max(timing_length.div_ceil(2))
                        } else if *timing_length > 15 {
                            it.2 > 8.max(timing_length.div_ceil(2))
                        } else {
                            it.2 > 5.max(timing_length.div_ceil(2))
                        }
                    })
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
                        match_timing_length: waystones[0].2,
                        match_text_pos: waystones[0].1,
                        match_text_length: waystones[0].2,
                    };
                }
            }
        });

        self.resolve_splits();
    }


    fn resolve_splits(&mut self) {
        let mut splits = vec![];

        self.matches = std::mem::take(&mut self.matches)
            .into_iter()
            .map(|it| {
                match it {
                    MatchKind::ReplaceWith { mut new_splits } => {
                        splits.append(&mut new_splits);

                        MatchKind::Tombstone
                    }
                    MatchKind::Split {
                        from_timing_pos,
                        from_timing_length,
                        from_text_pos,
                        from_text_length,
                        match_timing_pos,
                        match_timing_length,
                        match_text_pos,
                        match_text_length,
                    } => {
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
                        if (match_timing_pos + match_timing_length
                            < from_timing_pos + from_timing_length)
                            || (match_text_pos + match_text_length
                                < from_text_pos + from_text_length)
                        {
                            if match_timing_pos + match_timing_length
                                == from_timing_pos + from_timing_length
                                || match_text_pos + match_text_length
                                    == from_text_pos + from_text_length
                            {
                                splits.push(MatchKind::Empty {
                                    timing_pos: match_timing_pos + match_timing_length,
                                    timing_length: from_timing_pos + from_timing_length
                                        - match_timing_pos
                                        - match_timing_length,
                                    text_pos: match_text_pos + match_text_length,
                                    text_length: from_text_pos + from_text_length
                                        - match_text_pos
                                        - match_text_length,
                                });
                            } else {
                                splits.push(MatchKind::Unknown {
                                    timing_pos: match_timing_pos + match_timing_length,
                                    timing_length: from_timing_pos + from_timing_length
                                        - match_timing_pos
                                        - match_timing_length,
                                    text_pos: match_text_pos + match_text_length,
                                    text_length: from_text_pos + from_text_length
                                        - match_text_pos
                                        - match_text_length,
                                });
                            }
                        }

                        if match_timing_length != match_text_length {
                            MatchKind::Fuzzy {
                                timing_pos: match_timing_pos,
                                timing_length: match_timing_length,
                                text_pos: match_text_pos,
                                text_length: match_text_length,
                            }
                        } else {
                            MatchKind::Exact {
                                timing_pos: match_timing_pos,
                                timing_length: match_timing_length,
                                text_pos: match_text_pos,
                                text_length: match_text_length,
                            }
                        }
                    }
                    _ => it,
                }
            })
            .filter(|it| !matches!(it, MatchKind::Tombstone))
            .collect::<Vec<_>>();

        self.matches.append(&mut splits);
    }
}

/// Creates an aligned _ where a fits within b.
pub fn align_segments<T, ASource, BSource>(
    a: &Vec<Segment<T, ASource>>,
    b: &Vec<Segment<T, BSource>>,
    options: SegmentAlignerOptions,
) -> Vec<[(usize, usize); 2]>
where
    T: IsSegment + Send + Sync + Debug,
    ASource: Send + Sync + Debug,
    BSource: Send + Sync + Debug,
{
    let aligner = SegmentAligner::new(a, b, options);

    let matches = aligner.align();

    let matches = matches
        .into_iter()
        .filter(|it| {
            matches!(
                it,
                MatchKind::Exact { .. } | MatchKind::Fuzzy { .. } | MatchKind::Empty { .. }
            )
        })
        .map(|it| match it {
            MatchKind::Exact {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            }
            | MatchKind::Fuzzy {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            }
            | MatchKind::Empty {
                timing_pos,
                timing_length,
                text_pos,
                text_length,
            } => [(timing_pos, timing_length), (text_pos, text_length)],
            _ => {
                unreachable!()
            }
        })
        .filter(|it| it[0].1 != 0 && it[1].1 != 0)
        .collect::<Vec<_>>();

    matches
}

impl JapaneseTranscriptionContext {
    pub fn test<CTiming, CText>(&self, text_data: &mut CText, timing_data: &CTiming)
    where
        CTiming: CanSegment<Morpheme>,
        CTiming::Source: Send + Sync + HasTimestamp,
        CText: CanSegment<Morpheme> + AcceptsTimestamps<Morpheme>,
        CText::Source: Send + Sync,
    {
        let segmenter = JapaneseTextSegmenter::new();

        let timing_segments = timing_data.segments(&segmenter);
        let text_segments = text_data.segments(&segmenter);

        let timing_segments = timing_segments
            .filter(|it| match &it.data {
                Morpheme::Unk => true,
                Morpheme::Tagged(tag) => {
                    let surface = tag.surface();
                    match get_single_char(surface) {
                        Some(c) => !is_punctuation(c),
                        None => true,
                    }
                }
            })
            .collect::<Vec<_>>();

        let text_segments = text_segments
            .filter(|it| match &it.data {
                Morpheme::Unk => true,
                Morpheme::Tagged(tag) => {
                    let surface = tag.surface();
                    match get_single_char(surface) {
                        Some(c) => !is_punctuation(c),
                        None => true,
                    }
                }
            })
            .collect::<Vec<_>>();

        let alignments = align_segments(&timing_segments, &text_segments, SegmentAlignerOptions {});

        for [(timing_pos, timing_len), (text_pos, text_len)] in alignments {
            if timing_len == text_len {
                for i in 0..timing_len {
                    let timing_segment = &timing_segments[timing_pos + i];
                    let text_segment = &text_segments[text_pos + i];

                    text_data.accept(text_segment, timing_segment.source.timestamp());
                }
            } else {
                println!("Skipping...");
            }
        }
    }
}
