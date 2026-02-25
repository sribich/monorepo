pub mod processor;

use language_pack::LanguageTransformer;
use language_pack::TextProcessors;
use language_pack::Transform;
use processor::CollapseEmphaticSequences;
use processor::ConvertHalfWidth;
use processor::NormalizeCombiningCharacters;
use processor::NormalizeKanji;
use processor::NormalizeUnicode;

pub struct JapaneseTransformer;

impl LanguageTransformer for JapaneseTransformer {
    fn text_processors(&'_ self) -> TextProcessors<'_> {
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

        for processor in self.text_processors().pre {
            if !processor.matches(&transform.text) {
                continue;
            }

            transform = processor.process(transform);
        }

        transform
    }
}
