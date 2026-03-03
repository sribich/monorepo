use itertools::Itertools;
use language_pack::processor::TextProcessor;
use language_pack::processor::Transform;
use unicode_normalization::IsNormalized;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::is_nfkc_quick;

fn is_emphatic(c: char) -> bool {
    c == '\u{3063}'       // っ
        || c == '\u{30C3}' // ッ
        || c == '\u{30FC}' // ー
}

/// Collapses emphatic sequences.
///
/// Sequences are only collapsed when they are 2 characters or
/// longer, and they are collapsed down to zero. Sequences of
/// one character are not touched.
///
/// For example:
///
///   - すっっごーーい -> すごい
///
/// # Examples
///
/// ```rust
/// use language_pack::TextProcessor;
/// use language_pack_jp::transform::processor::CollapseEmphaticSequences;
///
/// let processor = CollapseEmphaticSequences {};
///
/// assert_eq!("アパート", processor.process("アパート".into()).text);
/// assert_eq!("すごい", processor.process("すっっごーーい".into()).text);
/// ```
pub struct CollapseEmphaticSequences;

impl TextProcessor for CollapseEmphaticSequences {
    fn matches(&self, text: &str) -> bool {
        text.chars().any(is_emphatic)
    }

    // TODO: We need to figure out how to add options here to allow us to
    //       do dictionary lookups when collapsing to know if we can collapse
    //       final emphatic characters.
    fn process(&self, transform: Transform) -> Transform {
        let mut line = String::with_capacity(transform.text.len());

        transform
            .text
            .chars()
            .map(Some)
            .chain([None])
            .tuple_windows()
            .filter_map(|(curr, next)| Some((curr?, next)))
            .fold(false, |prev, (curr, next)| {
                if prev && is_emphatic(curr) {
                    return true;
                }

                if let Some(next) = next && is_emphatic(curr) && is_emphatic(next) {
                    return true;
                }

                line.push(curr);

                false
            });

        Transform {
            text: line,
            mappings: vec![],
            previous: Some(Box::new(transform)),
        }
    }
}
