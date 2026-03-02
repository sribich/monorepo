pub mod processor;

use language_pack::LanguageTransformer;
use language_pack::TextProcessors;
use language_pack::Transform;
use processor::CollapseEmphaticSequences;
use processor::ConvertHalfWidth;
use processor::NormalizeCombiningCharacters;
use processor::NormalizeKanji;
use processor::NormalizeUnicode;

use crate::segment::Morpheme;

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

fn attaches(pos: &str) -> bool {
    ["助詞"].contains(&pos)
}

fn inflects(pos: &str) -> bool {
    ![
        "名詞",
        "代名詞",
        "助詞",
        "副詞",
        "連体詞",
        "接続詞",
        "感動詞",
    ]
    .contains(&pos)
}

fn is_new_root(pos: &str) -> bool {
    ["形状詞"].contains(&pos)
}

fn is_conjugating(s: &str) -> bool {
    s.ends_with("っ")
}

pub fn group_inflected(list: Vec<Morpheme>) -> Vec<(String, bool)> {
    list.into_iter()
        .fold(vec![(String::new(), true)], |mut prev, curr| {
            let conjugating = prev.last().map(|it| is_conjugating(&it.0)).unwrap_or(false);

            if inflects(curr.pos()) || conjugating {
                if !is_new_root(curr.pos()) || conjugating {
                    let prev_mut = prev.last_mut().unwrap();
                    let next = format!("{}{}", prev_mut.0, curr);
                    *prev_mut = (next, true);
                } else {
                    prev.push((curr.to_string(), true));
                }

                return prev;
            }

            prev.push((curr.to_string(), false));
            prev.push((String::new(), true));

            prev
        })
        .into_iter()
        .filter(|it| !it.0.is_empty())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    use blart::TreeMap;
    use itertools::Itertools;
    use language_pack::IsSegment;
    use language_pack::segment::TextSegmenter;
    use language_pack::transform::TextTransform;

    use crate::segment::JapaneseTextSegmenter;
    use crate::segment::Morpheme;
    use crate::transform::group_inflected;

    #[test]
    fn thing() {
        let text = "冬の食料がなくなるのだから、わたしが嫌だと言っても許されるわけがない。行かなかったら冬の食料がないので、どんなに嫌でもわたしだって参加するしかない。";
        // let text = "良かった";
        // let text = "良く";
        // let text = "ようで";
        let segments = JapaneseTextSegmenter::new().segment(text);

        // println!("{:#?}", segments);
        // panic!();
        // println!("{:#?}", group_inflected(segments));

        let mut adaptive = TreeMap::<CString, Option<usize>>::new();
        let mut adaptive_readings = TreeMap::<CString, Option<usize>>::new();

        let lines = BufReader::new(File::open("/home/nulliel/Result_45.csv").unwrap()).lines();
        for line in lines.map_while(Result::ok) {
            if line == r#""""# {
                continue;
            }

            if let Some((left, right)) = line.split_once(",") {
                if let Some((middle, right)) = right.split_once(",") {
                    let freq = usize::from_str_radix(right, 10).ok();

                    if left != r#""""# {
                        adaptive.insert(CString::new(left).unwrap(), freq);
                    }

                    if middle != r#""""# {
                        adaptive_readings.insert(CString::new(middle).unwrap(), freq);
                    }
                }
            }
        }

        // let mut transform = TextTransform::new("言っても".to_owned());
        // let resolve = transform.resolve(&adaptive);
        // println!("{:#?}", resolve);
        // panic!();

        // println!("{:#?}", segments);
        let segments = group_inflected(segments);
        // println!("{:#?}", segments);
        // panic!();

        let mut output = vec![];

        for (segment, inflects) in &segments {
            if *inflects {
                let mut transform = TextTransform::new(segment.to_owned());
                let resolve = transform.resolve(&adaptive);

                if resolve.is_empty() {
                    output.push((segment.clone(), segment.clone()));
                } else {
                    let mut out = resolve
                        .into_iter()
                        .map(|it| {
                            let base = it.1;

                            let inflected = it.0.map_or_else(
                                || base.clone(),
                                |inflection| {
                                    if inflection.last_inflection == "" {
                                        return base.clone();
                                    }

                                    if inflection.last_inflection.len() == base.len() {
                                        return inflection.inflection;
                                    }

                                    return [
                                        &base[..(base.len() - inflection.last_inflection.len())],
                                        &inflection.inflection,
                                    ]
                                    .join("");
                                },
                            );

                            (inflected, base)
                        })
                        .collect::<Vec<_>>();

                    output.append(&mut out);
                }
            } else {
                output.push((segment.clone(), segment.clone()));
            }
        }

        let mut result = vec![];
        let mut i = 0;

        while i < output.len() {
            let segment = &output[i];

            if i == output.len() - 1 {
                result.push(segment.clone());
                i += 1;
                continue;
            }

            if !adaptive.contains_key(&CString::new(segment.0.as_bytes()).unwrap()) {
                result.push(segment.clone());
                i += 1;
                continue;
            }

            let mut key_with_boundary = i + 1;
            let mut curr_check = i + 2;

            while curr_check <= output.len()
                && let Some(prefix) = adaptive.get_prefix(
                    &CString::new(
                        output[i..curr_check]
                            .iter()
                            .map(|it| &it.0)
                            .join("")
                            .as_bytes(),
                    )
                    .unwrap(),
                )
            {
                if adaptive.contains_key(
                    &CString::new(
                        output[i..curr_check]
                            .iter()
                            .map(|it| &it.0)
                            .join("")
                            .as_bytes(),
                    )
                    .unwrap(),
                ) {
                    key_with_boundary = curr_check;
                }

                curr_check += 1;
            }

            let entry = &output[i..key_with_boundary];
            i = key_with_boundary;

            if entry.len() == 1 {
                result.push(segment.clone());
            } else {
                result.push(
                    entry
                        .iter()
                        .fold((String::new(), String::new()), |prev, curr| {
                            let a = format!("{}{}", prev.0, curr.0);

                            (a.clone(), a)
                        }),
                );
            }
        }

        let result = result
            .into_iter()
            .map(|it| {
                let freq = adaptive
                    .get(&CString::new(it.0.as_bytes()).unwrap())
                    .or_else(|| {
                        adaptive_readings
                            .get(&CString::new(it.0.as_bytes()).unwrap())
                            .or(None)
                    });

                (it.0, it.1, freq)
            })
            .collect::<Vec<_>>();

        assert_eq!(1, 1);
    }
}

// 1, 2, 3
// 1..2  3
//

/*

const ikuVerbs = ['いく', '行く', '逝く', '往く'];
const godanUSpecialVerbs = ['こう', 'とう', '請う', '乞う', '恋う', '問う', '訪う', '宣う', '曰う', '給う', '賜う', '揺蕩う'];
const fuVerbTeConjugations = [
    ['のたまう', 'のたもう'],
    ['たまう', 'たもう'],
    ['たゆたう', 'たゆとう'],
];

/** @typedef {keyof typeof conditions} Condition */
/**
 * @param {'て' | 'た' | 'たら' | 'たり'} suffix
 * @param {Condition[]} conditionsIn
 * @param {Condition[]} conditionsOut
 * @returns {import('language-transformer').SuffixRule<Condition>[]}
 */
function irregularVerbSuffixInflections(suffix, conditionsIn, conditionsOut) {
    const inflections = [];
    for (const verb of ikuVerbs) {
        inflections.push(suffixInflection(`${verb[0]}っ${suffix}`, verb, conditionsIn, conditionsOut));
    }
    for (const verb of godanUSpecialVerbs) {
        inflections.push(suffixInflection(`${verb}${suffix}`, verb, conditionsIn, conditionsOut));
    }
    for (const [verb, teRoot] of fuVerbTeConjugations) {
        inflections.push(suffixInflection(`${teRoot}${suffix}`, verb, conditionsIn, conditionsOut));
    }
    return inflections;
}

const conditions = {
    'v': {
        name: 'Verb',
        i18n: [
            {
                language: 'ja',
                name: '動詞',
            },
        ],
        isDictionaryForm: false,
        subConditions: ['v1', 'v5', 'vk', 'vs', 'vz'],
    },
    'v1': {
        name: 'Ichidan verb',
        i18n: [
            {
                language: 'ja',
                name: '一段動詞',
            },
        ],
        isDictionaryForm: true,
        subConditions: ['v1d', 'v1p'],
    },
    'v1d': {
        name: 'Ichidan verb, dictionary form',
        i18n: [
            {
                language: 'ja',
                name: '一段動詞、終止形',
            },
        ],
        isDictionaryForm: false,
    },
    'v1p': {
        name: 'Ichidan verb, progressive or perfect form',
        i18n: [
            {
                language: 'ja',
                name: '一段動詞、～てる・でる',
            },
        ],
        isDictionaryForm: false,
    },
    'v5': {
        name: 'Godan verb',
        i18n: [
            {
                language: 'ja',
                name: '五段動詞',
            },
        ],
        isDictionaryForm: true,
        subConditions: ['v5d', 'v5s'],
    },
    'v5d': {
        name: 'Godan verb, dictionary form',
        i18n: [
            {
                language: 'ja',
                name: '五段動詞、終止形',
            },
        ],
        isDictionaryForm: false,
    },
    'v5s': {
        name: 'Godan verb, short causative form',
        i18n: [
            {
                language: 'ja',
                name: '五段動詞、～す・さす',
            },
        ],
        isDictionaryForm: false,
        subConditions: ['v5ss', 'v5sp'],
    },
    'v5ss': {
        name: 'Godan verb, short causative form having さす ending (cannot conjugate with passive form)',
        i18n: [
            {
                language: 'ja',
                name: '五段動詞、～さす',
            },
        ],
        isDictionaryForm: false,
    },
    'v5sp': {
        name: 'Godan verb, short causative form not having さす ending (can conjugate with passive form)',
        i18n: [
            {
                language: 'ja',
                name: '五段動詞、～す',
            },
        ],
        isDictionaryForm: false,
    },
    'vk': {
        name: 'Kuru verb',
        i18n: [
            {
                language: 'ja',
                name: '来る動詞',
            },
        ],
        isDictionaryForm: true,
    },
    'vs': {
        name: 'Suru verb',
        i18n: [
            {
                language: 'ja',
                name: 'する動詞',
            },
        ],
        isDictionaryForm: true,
    },
    'vz': {
        name: 'Zuru verb',
        i18n: [
            {
                language: 'ja',
                name: 'ずる動詞',
            },
        ],
        isDictionaryForm: true,
    },
    'adj-i': {
        name: 'Adjective with i ending',
        i18n: [
            {
                language: 'ja',
                name: '形容詞',
            },
        ],
        isDictionaryForm: true,
    },
    '-ます': {
        name: 'Polite -ます ending',
        isDictionaryForm: false,
    },
    '-ません': {
        name: 'Polite negative -ません ending',
        isDictionaryForm: false,
    },
    '-て': {
        name: 'Intermediate -て endings for progressive or perfect tense',
        isDictionaryForm: false,
    },
    '-ば': {
        name: 'Intermediate -ば endings for conditional contraction',
        isDictionaryForm: false,
    },
    '-く': {
        name: 'Intermediate -く endings for adverbs',
        isDictionaryForm: false,
    },
    '-た': {
        name: '-た form ending',
        isDictionaryForm: false,
    },
    '-ん': {
        name: '-ん negative ending',
        isDictionaryForm: false,
    },
    '-なさい': {
        name: 'Intermediate -なさい ending (polite imperative)',
        isDictionaryForm: false,
    },
    '-ゃ': {
        name: 'Intermediate -や ending (conditional contraction)',
        isDictionaryForm: false,
    },
};
*/
