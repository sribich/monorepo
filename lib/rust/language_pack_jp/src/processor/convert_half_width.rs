use itertools::Itertools;
use language_pack::processor::TextProcessor;
use language_pack::processor::Transform;

use crate::text::DAKUTEN_HALF;
use crate::text::HALF_WIDTH_DAKUTEN_MAP;
use crate::text::HALF_WIDTH_HANDAKUTEN_MAP;
use crate::text::HALF_WIDTH_MAP;
use crate::text::HANDAKUTEN_HALF;
use crate::text::is_halfwidth;

/// Converts half-width katakana into full-width katakana.
///
/// Dakuten and handakuten will be merged into a single character
/// if the mapping is legal. Illegal mappings will remain separate.
///
/// For example:
///
///   - ﾖﾐﾁｬﾝ -> ヨミチャン
///   - ﾏﾂｵ ﾊﾞｼｮｳ ｱﾟ -> マツオ バショウ ア゚
///
/// # Examples
///
/// ```rust
/// use language_pack::TextProcessor;
/// use language_pack_jp::transform::processor::ConvertHalfWidth;
///
/// let processor = ConvertHalfWidth {};
///
/// assert_eq!("ヨミチャン", processor.process("ﾖﾐﾁｬﾝ".into()).text);
/// assert_eq!(
///     "マツオ バショウ ア゚",
///     processor.process("ﾏﾂｵ ﾊﾞｼｮｳ ｱﾟ".into()).text
/// );
/// ```
pub struct ConvertHalfWidth;

impl TextProcessor for ConvertHalfWidth {
    fn matches(&self, text: &str) -> bool {
        text.chars().any(is_halfwidth)
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
            .filter_map(|(a, b)| Some((a?, b)))
            .fold(false, |prev, (curr, next)| {
                // The previous window's `next` was a voiced half and was merged into
                // a single character.
                if prev {
                    return false;
                }

                let curr_len = curr.len_utf8();

                if let Some(next) = next {
                    let merged = match next {
                        DAKUTEN_HALF if let Some(v) = HALF_WIDTH_DAKUTEN_MAP.get(&curr) => Some(v),
                        HANDAKUTEN_HALF if let Some(v) = HALF_WIDTH_HANDAKUTEN_MAP.get(&curr) => {
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

                line.push(match HALF_WIDTH_MAP.get(&curr) {
                    Some(c) => *c,
                    None => curr,
                });

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
