use core::iter::Iterator;
use std::collections::BTreeMap;
use std::fmt::Debug;

use epub::archive::EpubSegment;
use itertools::Itertools;
use language_pack::AcceptsTimestamps;
use language_pack::CanSegment;
use language_pack::HasTimestamp;
use language_pack::ProducesTimestamps;
use language_pack::Segment;
use language_pack::hirschberg;
use language_pack::segment::IsSegment;
use language_pack::segment::TextSegmenter;
use rand::Rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::segment::JapaneseTextSegmenter;
use crate::segment::Morpheme;
use crate::text::get_single_char;
use crate::text::is_punctuation;

#[derive(Debug)]
pub struct EbookSegments(pub Vec<EpubSegment>);

impl EbookSegments {
    pub fn new(data: Vec<EpubSegment>) -> Self {
        Self(data)
    }
}

impl<T> AcceptsTimestamps<T> for EbookSegments
where
    T: IsSegment + PartialEq + Debug + Send,
{
    fn accept(&mut self, segment: &Segment<T, Self::Source>, timestamp: language_pack::Timestamp) {
        let source = *self.source(segment);
        let segment = &mut self.0[source];

        let acceptee = timestamp;

        segment.time = (
            acceptee
                .0
                .map(|it| {
                    if let Some(t0) = segment.time.0 {
                        Some(it.min(t0))
                    } else {
                        None
                    }
                })
                .flatten()
                .or(timestamp.0),
            acceptee
                .1
                .map(|it| {
                    if let Some(t1) = segment.time.1 {
                        Some(it.max(t1))
                    } else {
                        None
                    }
                })
                .flatten()
                .or(timestamp.1),
        );
    }

    fn try_accept_ranged(
        &mut self,
        segment: &[Segment<T, Self::Source>],
        timestamp: language_pack::Timestamp,
    ) -> bool {
        let source = segment[0].source;

        if segment.iter().all(|it| it.source == source) {
            let segment = &mut self.0[source];

            let acceptee = timestamp;

            segment.time = (
                acceptee
                    .0
                    .map(|it| {
                        if let Some(t0) = segment.time.0 {
                            Some(it.min(t0))
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .or(timestamp.0),
                acceptee
                    .1
                    .map(|it| {
                        if let Some(t1) = segment.time.1 {
                            Some(it.max(t1))
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .or(timestamp.1),
            );

            true
        } else {
            false
        }
    }
}

impl<T> CanSegment<T> for EbookSegments
where
    T: IsSegment + PartialEq + Debug + Send,
{
    type Source = usize;

    fn segments(
        &self,
        segmenter: impl TextSegmenter<Feature = T> + Sync,
    ) -> Vec<Segment<T, Self::Source>> {
        self.0
            .par_iter()
            .enumerate()
            .flat_map(|(index, item)| {
                segmenter
                    .segment(item.text(), true)
                    .into_iter()
                    .map(move |segment| Segment {
                        data: segment,
                        source: index, // (), // TranscriptionSource { line: line_index },
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn source<'a>(&self, segment: &'a Segment<T, Self::Source>) -> &'a Self::Source {
        &segment.source
    }
}
