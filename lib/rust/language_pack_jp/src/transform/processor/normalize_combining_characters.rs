use itertools::Itertools;
use language_pack::TextProcessor;
use language_pack::Transform;

use crate::text::DAKUTEN_COMB;
use crate::text::DAKUTEN_FULL;
use crate::text::DAKUTEN_HALF;
use crate::text::DAKUTEN_MAP;
use crate::text::HANDAKUTEN_COMB;
use crate::text::HANDAKUTEN_FULL;
use crate::text::HANDAKUTEN_HALF;
use crate::text::HANDAKUTEN_MAP;
use crate::text::is_dakuten;

/// Normalizes text containing UTF-8 combining characters.
///
/// For example:
///
///   - ド -> ド (U+30C8 U+3099 -> U+30C9)
///
/// # Examples
///
/// ```rust
/// use language_pack::TextProcessor;
/// use language_pack_jp::transform::processor::NormalizeCombiningCharacters;
///
/// let transformer = NormalizeCombiningCharacters {};
///
/// assert_eq!(
///     "\u{30C9}",
///     transformer.process("\u{30C8}\u{3099}".into()).text
/// );
/// ```
pub struct NormalizeCombiningCharacters;

impl TextProcessor for NormalizeCombiningCharacters {
    fn matches(&self, text: &str) -> bool {
        text.chars().any(is_dakuten)
    }

    fn process(&self, transform: Transform) -> Transform {
        let mut line = String::with_capacity(transform.text.len());

        let mut src_idx = 0_usize;
        let mut dst_idx = 0_usize;
        let mut mappings = vec![];

        transform
            .text
            .chars()
            .map(Some)
            .chain([None])
            .tuple_windows()
            .filter_map(|(curr, next)| Some((curr?, next)))
            .fold(false, |prev, (curr, next)| {
                // The previous window's `next` was a voiced half and was merged into
                // a single character.
                if prev {
                    return false;
                }

                let curr_len = curr.len_utf8();

                if let Some(next) = next {
                    let merged = match next {
                        DAKUTEN_HALF | DAKUTEN_FULL | DAKUTEN_COMB
                            if let Some(v) = DAKUTEN_MAP.get(&curr) =>
                        {
                            Some(v)
                        }
                        HANDAKUTEN_HALF | HANDAKUTEN_FULL | HANDAKUTEN_COMB
                            if let Some(v) = HANDAKUTEN_MAP.get(&curr) =>
                        {
                            Some(v)
                        }
                        _ => None,
                    };

                    if let Some(merged) = merged {
                        let next_len = next.len_utf8();
                        let merged_len = merged.len_utf8();

                        line.push(*merged);

                        mappings.push((
                            (src_idx..(src_idx + merged_len)).into(),
                            (dst_idx..(dst_idx + curr_len + next_len)).into(),
                        ));

                        src_idx += merged_len;
                        dst_idx += curr_len + next_len;

                        return true;
                    }
                }

                line.push(curr);

                mappings.push((
                    (src_idx..(src_idx + curr_len)).into(),
                    (dst_idx..(dst_idx + curr_len)).into(),
                ));

                src_idx += curr_len;
                dst_idx += curr_len;

                false
            });

        Transform {
            text: line,
            mappings,
            previous: Some(Box::new(transform)),
        }
    }
}
