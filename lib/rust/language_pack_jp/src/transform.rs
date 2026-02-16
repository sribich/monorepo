#[cfg(test)]
pub mod test {
    use std::ffi::CString;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    use blart::TreeMap;

    use crate::segment::JapaneseTextSegmenter;
    use crate::segment::TextSegmenter;

    fn get_tree() -> TreeMap<CString, u32> {
        let mut adaptive = TreeMap::<CString, u32>::new();

        let lines = BufReader::new(File::open("/home/nulliel/main_Word.csv").unwrap()).lines();

        for line in lines.map_while(Result::ok) {
            if line == r#""""# {
                continue;
            }

            adaptive.insert(CString::new(line).unwrap(), 1);
        }

        adaptive

        /*
        for it in adaptive.prefix("もう".as_bytes()) {
            if it.0.to_string_lossy().starts_with("もう少") {
                println!("MATCH: {:#?}", it.0);
            }
        }
         */
    }

    #[test]
    fn mecab() {
        let tagger = JapaneseTextSegmenter::new();

        let result = tagger.segment("集めておかなきゃ");
        println!("{:#?}", result);

        let result = tagger.segment("彷徨った");
        println!("{:#?}", result);

        let result = tagger.segment("家ﾊﾛｰﾜｰﾙﾄﾞ家");
        println!("{:#?}", result);

        let result = tagger.segment("ﾊﾛｰﾜｰﾙﾄﾞ");
        println!("{:#?}", result);
    }
}

/*
textPreprocessors: {
            convertHalfWidthCharacters,
            alphabeticToHiragana,
            normalizeCombiningCharacters,
            normalizeCJKCompatibilityCharacters,
            normalizeRadicalCharacters,
            alphanumericWidthVariants,
            convertHiraganaToKatakana,
            collapseEmphaticSequences,
            standardizeKanji,
        },
        languageTransforms: japaneseTransforms,
 */
