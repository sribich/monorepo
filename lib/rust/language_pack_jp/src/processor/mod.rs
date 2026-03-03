use collapse_emphatic_sequences::CollapseEmphaticSequences;
use convert_half_width::ConvertHalfWidth;
use language_pack::processor::Processor;
use language_pack::processor::TextProcessors;
use language_pack::processor::Transform;
use normalize_combining_characters::NormalizeCombiningCharacters;
use normalize_kanji::NormalizeKanji;
use normalize_unicode::NormalizeUnicode;

mod collapse_emphatic_sequences;
mod convert_half_width;
mod normalize_combining_characters;
mod normalize_kanji;
mod normalize_unicode;

pub struct JapaneseProcessor;

impl Processor for JapaneseProcessor {
    fn processors(&'_ self) -> TextProcessors<'_> {
        TextProcessors {
            pre: &[
                &ConvertHalfWidth {},
                &NormalizeCombiningCharacters {},
                &NormalizeUnicode {},
                &CollapseEmphaticSequences {},
                &NormalizeKanji {},
            ],
            post: &[],
        }
    }

    fn transform(&self, text: String) -> Transform {
        let mut transform: Transform = text.into();

        for processor in self.processors().pre {
            if !processor.matches(&transform.text) {
                continue;
            }

            transform = processor.process(transform);
        }

        transform
    }
}
