use language_pack::TextProcessor;
use language_pack::Transform;
use unicode_normalization::IsNormalized;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::is_nfkc_quick;

/// Normalizes UTF-8 text.
///
/// We specifically run NFKC normalization, the most aggressive form,
/// which reduces all possible text, down to its most basic form.
///
/// For example:
///
///   - ㌀ → アパート
///   - ⼀ → 一 (U+2F00 → U+4E00)
///
/// # Examples
///
/// ```rust
/// use language_pack::TextProcessor;
/// use language_pack_jp::transform::processor::NormalizeUnicode;
///
/// let processor = NormalizeUnicode {};
///
/// assert_eq!("アパート", processor.process("㌀".into()).text);
/// assert_eq!("一", processor.process("⼀".into()).text);
/// ```
pub struct NormalizeUnicode;

impl TextProcessor for NormalizeUnicode {
    fn matches(&self, text: &str) -> bool {
        is_nfkc_quick(text.chars()) != IsNormalized::Yes
    }

    fn process(&self, transform: Transform) -> Transform {
        let line = transform.text.nfkc().collect();

        Transform {
            text: line,
            mappings: vec![],
            previous: Some(Box::new(transform)),
        }
    }
}
