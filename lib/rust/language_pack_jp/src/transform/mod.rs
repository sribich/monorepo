pub mod processor;

use language_pack::LanguageTransformer;
use language_pack::TextProcessors;
use language_pack::Transform;
use processor::ConvertHalfWidth;
use processor::NormalizeCombiningCharacters;

struct JapaneseTransformer;

impl LanguageTransformer for JapaneseTransformer {
    fn text_processors(&self) -> TextProcessors {
        // 	// convertHalfWidthCharacters,
        // 	alphabeticToHiragana,
        // 	// normalizeCombiningCharacters,
        // 	normalizeCJKCompatibilityCharacters,
        // 	normalizeRadicalCharacters,
        // 	alphanumericWidthVariants,
        // 	convertHiraganaToKatakana,
        // 	collapseEmphaticSequences,
        // 	standardizeKanji,
        TextProcessors {
            pre: &[&ConvertHalfWidth {}, &NormalizeCombiningCharacters {}],
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
