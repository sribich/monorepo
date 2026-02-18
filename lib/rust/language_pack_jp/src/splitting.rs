use mecab::Tagger;
use serde::Deserialize;
use serde::Serialize;

use crate::transcription::Word;

#[derive(Debug)]
pub struct MorphSegment {
    word: String,
    base: String,
    freq: bool,
    last_morph: Word,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub word: String,
    pub base: String,
    pub freq: bool,
}

pub fn segment_text<S: AsRef<str>>(text: S) -> Vec<Segment> {
    let home = std::env::var("HOME").unwrap();
    let tagger = Tagger::new(format!(
        "-Ounidic --dicdir={}/Projects/sribich/_/unidic-cwj-202302",
        home
    ));

    let output = tagger.parse_str(text.as_ref().to_owned().as_bytes());

    let mut morphemes = vec![];

    for line in output.split('\n') {
        if line == "UNK" || line == "EOS" || line == "BOS" || line.is_empty() {
            continue;
        }

        let mut items = line.split(',');

        morphemes.push(Word {
            ts: None,
            unit: items.next().unwrap_or("").to_string(),
            pos1: items.next().unwrap_or("").to_string(),
            pos2: items.next().unwrap_or("").to_string(),
            pos3: items.next().unwrap_or("").to_string(),
            pos4: items.next().unwrap_or("").to_string(),
            c_type: items.next().unwrap_or("").to_string(),
            c_form: items.next().unwrap_or("").to_string(),
            l_form: items.next().unwrap_or("").to_string(),
            lemma: items.next().unwrap_or("").to_string(), // Dictionary form
            orth: items.next().unwrap_or("").to_string(),  // Same as surface
            orth_base: items.next().unwrap_or("").to_string(), // Dictionary written form
        });
    }

    let mut segments: Vec<MorphSegment> = vec![];

    /*
    let mut text = vec![];

    let line_chars = line.as_ref().chars().count();
    let mut char_idx = 0;

    'outer: for morpheme in morphemes {
        let morpheme_length = morpheme.surface.chars().count();

        trace!(
            "Searching for {} \t-- char_idx={}/{} | morphene_length={}",
            morpheme.surface, char_idx, line_chars, morpheme_length
        );
        // println!(
        //     "Searching for {:?} -- char_idx={}/{} | morphene_length={}",
        //     morpheme.unit,
        //     char_idx,
        //     segment.words.len(),
        //     morphene_length
        // );

        'inner: for i in char_idx..=(line_chars - morpheme_length) {
            let mut indices = line.as_ref().char_indices();
            let start_idx = indices.nth(i).map(|it| it.0).unwrap();
            let end_idx = indices.nth(morpheme_length - 1).map(|it| it.0);

            let word = &line.as_ref()[start_idx..end_idx.unwrap_or(line.as_ref().len())];

            if morpheme.surface == word {
                if i != char_idx {
                    let mut old_indices = line.as_ref().char_indices();
                    let old_start = old_indices.nth(char_idx).unwrap().0;

                    text.push(Segment::Raw(line.as_ref()[old_start..start_idx].to_owned()));
                }

                text.push(Segment::Tagged(morpheme));

                char_idx = i + morpheme_length;
                continue 'outer;
            }
        }

        panic!("No match found");
    }

    text

    */
    for morpheme in morphemes {
        if segments.is_empty() || morpheme.c_form.starts_with("\u{7d42}\u{6b62}\u{5f62}") {
            segments.push(MorphSegment {
                word: morpheme.unit.clone(),
                base: morpheme.orth_base.clone(),
                freq: has_freq(&morpheme),
                last_morph: morpheme,
            });
            continue;
        }

        let last_segment = segments.last_mut().unwrap();

        let previous_word_is_conjunctive = last_segment
            .last_morph
            .c_form
            .starts_with("\u{9023}\u{7528}\u{5f62}")
            || last_segment
                .last_morph
                .c_form
                .starts_with("\u{672a}\u{7136}\u{5f62}");

        if morpheme.pos1 == "\u{52a9}\u{52d5}\u{8a5e}" && previous_word_is_conjunctive {
            last_segment.word.push_str(&morpheme.unit);
            last_segment.last_morph = morpheme;
            continue;
        }

        if morpheme.pos2 == "\u{63a5}\u{7d9a}\u{52a9}\u{8a5e}"
            && last_segment
                .last_morph
                .c_form
                .starts_with("\u{9023}\u{7528}\u{5f62}")
        {
            last_segment.word.push_str(&morpheme.unit);
            last_segment.last_morph = morpheme;
            continue;
        }

        if morpheme.pos1 == "\u{63a5}\u{5c3e}\u{8f9e}" {
            last_segment.word.push_str(&morpheme.unit);
            last_segment.base.push_str(&morpheme.unit);
            last_segment.last_morph = morpheme;
            continue;
        }

        segments.push(MorphSegment {
            word: morpheme.unit.clone(),
            base: morpheme.orth_base.clone(),
            freq: has_freq(&morpheme),
            last_morph: morpheme,
        });
    }

    segments
        .into_iter()
        .map(|it| Segment {
            word: it.word,
            base: it.base,
            freq: it.freq,
        })
        .collect::<Vec<_>>()
}

fn has_freq(morpheme: &Word) -> bool {
    match &*morpheme.pos1 {
        "補助記号" => false,
        "助動詞" => false,
        "助詞" => false,
        "接頭辞" => false,
        "空白" => false,
        "感動詞" => false,
        "接続詞" => false,
        "記号" => false,
        "接尾辞" => false,
        "動詞" => true,
        "名詞" => true,
        "形容詞" => true,
        "代名詞" => true,
        "形状詞" => true,
        "副詞" => true,
        "連体詞" => true,
        _ => {
            println!("{:#?} - {:#?}", morpheme.unit, morpheme.pos1);
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::segment_text;

    #[derive(Debug)]
    pub struct Segment(String);

    pub fn test_segment_text<S: AsRef<str>>(text: S) -> Vec<Segment> {
        segment_text(text)
            .into_iter()
            .map(|it| Segment(it.word))
            .collect::<Vec<_>>()
    }

    // #[test]
    fn test() {
        let text = "\u{69d8}\u{3005}\u{306a}\u{77e5}\u{8b58}\u{304c}\u{4e00}\u{518a}\u{306b}\u{307e}\u{3068}\u{3081}\u{3089}\u{308c}\u{3066}\u{3044}\u{308b}\u{672c}\u{3092}\u{8aad}\u{3080}\u{3068}\u{3001}\u{3068}\u{3066}\u{3082}\u{5f97}\u{3092}\u{3057}\u{305f}\u{6c17}\u{5206}\u{306b}\u{306a}\u{308c}\u{308b}\u{3057}\u{3001}\u{81ea}\u{5206}\u{304c}\u{3053}\u{306e}\u{76ee}\u{3067}\u{898b}\u{305f}\u{3053}\u{3068}\u{304c}\u{306a}\u{3044}\u{4e16}\u{754c}\u{3092}\u{3001}\u{672c}\u{5c4b}\u{3084}\u{56f3}\u{66f8}\u{9928}\u{306b}\u{4e26}\u{3076}\u{5199}\u{771f}\u{96c6}\u{3092}\u{901a}\u{3057}\u{3066}\u{898b}\u{308b}\u{306e}\u{3082}\u{3001}\u{4e16}\u{754c}\u{304c}\u{5e83}\u{304c}\u{3063}\u{3066}\u{3044}\u{304f}\u{3088}\u{3046}\u{3067}\u{9676}\u{9154}\u{3067}\u{304d}\u{308b}\u{3002}";

        expect_test::expect![[r#"
            [
                Segment(
                    "\u{69d8}\u{3005}",
                ),
                Segment(
                    "\u{306a}",
                ),
                Segment(
                    "\u{77e5}\u{8b58}",
                ),
                Segment(
                    "\u{304c}",
                ),
                Segment(
                    "\u{4e00}\u{518a}",
                ),
                Segment(
                    "\u{306b}",
                ),
                Segment(
                    "\u{307e}\u{3068}\u{3081}\u{3089}\u{308c}\u{3066}",
                ),
                Segment(
                    "\u{3044}\u{308b}",
                ),
                Segment(
                    "\u{672c}",
                ),
                Segment(
                    "\u{3092}",
                ),
                Segment(
                    "\u{8aad}\u{3080}",
                ),
                Segment(
                    "\u{3068}",
                ),
                Segment(
                    "\u{3001}",
                ),
                Segment(
                    "\u{3068}\u{3066}\u{3082}",
                ),
                Segment(
                    "\u{5f97}",
                ),
                Segment(
                    "\u{3092}",
                ),
                Segment(
                    "\u{3057}\u{305f}",
                ),
                Segment(
                    "\u{6c17}\u{5206}",
                ),
                Segment(
                    "\u{306b}",
                ),
                Segment(
                    "\u{306a}\u{308c}\u{308b}",
                ),
                Segment(
                    "\u{3057}",
                ),
                Segment(
                    "\u{3001}",
                ),
                Segment(
                    "\u{81ea}\u{5206}",
                ),
                Segment(
                    "\u{304c}",
                ),
                Segment(
                    "\u{3053}\u{306e}",
                ),
                Segment(
                    "\u{76ee}",
                ),
                Segment(
                    "\u{3067}",
                ),
                Segment(
                    "\u{898b}\u{305f}",
                ),
                Segment(
                    "\u{3053}\u{3068}",
                ),
                Segment(
                    "\u{304c}",
                ),
                Segment(
                    "\u{306a}\u{3044}",
                ),
                Segment(
                    "\u{4e16}\u{754c}",
                ),
                Segment(
                    "\u{3092}",
                ),
                Segment(
                    "\u{3001}",
                ),
                Segment(
                    "\u{672c}\u{5c4b}",
                ),
                Segment(
                    "\u{3084}",
                ),
                Segment(
                    "\u{56f3}\u{66f8}\u{9928}",
                ),
                Segment(
                    "\u{306b}",
                ),
                Segment(
                    "\u{4e26}\u{3076}",
                ),
                Segment(
                    "\u{5199}\u{771f}\u{96c6}",
                ),
                Segment(
                    "\u{3092}",
                ),
                Segment(
                    "\u{901a}\u{3057}\u{3066}",
                ),
                Segment(
                    "\u{898b}\u{308b}",
                ),
                Segment(
                    "\u{306e}",
                ),
                Segment(
                    "\u{3082}",
                ),
                Segment(
                    "\u{3001}",
                ),
                Segment(
                    "\u{4e16}\u{754c}",
                ),
                Segment(
                    "\u{304c}",
                ),
                Segment(
                    "\u{5e83}\u{304c}\u{3063}\u{3066}",
                ),
                Segment(
                    "\u{3044}\u{304f}",
                ),
                Segment(
                    "\u{3088}\u{3046}",
                ),
                Segment(
                    "\u{3067}",
                ),
                Segment(
                    "\u{9676}\u{9154}",
                ),
                Segment(
                    "\u{3067}\u{304d}\u{308b}",
                ),
                Segment(
                    "\u{3002}",
                ),
            ]
        "#]]
        .assert_debug_eq(&test_segment_text(text));
    }
}
