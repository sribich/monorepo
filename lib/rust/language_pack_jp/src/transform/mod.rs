use std::ffi::CStr;
use std::ffi::CString;

use itertools::Itertools;
use language_pack::segment::TextSegmenter;
use language_pack::transform::LanguageTransformer;
pub use transforms::*;
use trie_rs::map::Trie;

use crate::segment::JapaneseTextSegmenter;
use crate::segment::Morpheme;

mod transforms;

fn attaches(pos: &str, pos2: &str) -> bool {
    ["助詞", "助動詞"].contains(&pos) && !["係助詞", "格助詞", "準体助詞"].contains(&pos2)
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
    ["形状詞", "動詞"].contains(&pos)
}

fn is_conjugating(s: &str) -> bool {
    s.ends_with("っ")
}

pub fn group_inflected(list: Vec<Morpheme>) -> Vec<(String, bool)> {
    // TODO: This can be a static
    let morpheme = Morpheme::Untagged("".to_owned());

    list.iter()
        .chain([&morpheme])
        .tuple_windows()
        .fold(vec![(String::new(), true)], |mut prev, (curr, next)| {
            let conjugating = prev.last().map(|it| is_conjugating(&it.0)).unwrap_or(false);

            if inflects(curr.pos()) || conjugating || attaches(curr.pos(), curr.pos2()) {
                if !is_new_root(curr.pos()) || conjugating {
                    let prev_mut = prev.last_mut().unwrap();
                    let next = format!("{}{}", prev_mut.0, curr);
                    *prev_mut = (next, true);
                } else {
                    prev.push((curr.to_string(), true));
                }

                return prev;
            }

            if attaches(next.pos(), next.pos2()) {
                prev.push((curr.to_string(), true));
            } else {
                prev.push((curr.to_string(), false));
                prev.push((String::new(), true));
            }

            prev
        })
        .into_iter()
        .filter(|it| !it.0.is_empty())
        .collect::<Vec<_>>()
}

fn either_dict_has_prefix(
    data: &str,
    adaptive: &Trie<u8, Option<usize>>,
    adaptive_readings: &Trie<u8, Option<usize>>,
) -> bool {
    adaptive.is_prefix(data)
        || adaptive_readings.is_prefix(data)
        || adaptive.exact_match(data).is_some()
        || adaptive_readings.exact_match(data).is_some()
}

fn either_dict_has_exact(
    data: &str,
    adaptive: &Trie<u8, Option<usize>>,
    adaptive_readings: &Trie<u8, Option<usize>>,
) -> bool {
    adaptive.exact_match(data).is_some() || adaptive_readings.exact_match(data).is_some()
}

pub fn transform_japanese_text(
    line: &str,
    adaptive: &Trie<u8, Option<usize>>,
    adaptive_readings: &Trie<u8, Option<usize>>,
) -> Vec<(String, String, Option<usize>)> {
    let segmenter = JapaneseTextSegmenter::new();
    let segments = group_inflected(segmenter.segment(line));

    let mut output = vec![];

    for (segment, inflects) in &segments {
        let dict_entry: Option<(String, String, Option<String>)> =
            if adaptive.exact_match(segment).is_some() {
                Some((segment.clone(), segment.clone(), None))
            } else if adaptive_readings.exact_match(segment).is_some() {
                Some((segment.clone(), segment.clone(), None))
            } else {
                None
            };

        if *inflects {
            let conditions = &*JAPANESE_CONDITIONS;

            let mut transformer = LanguageTransformer::new(
                JAPANESE_TRANSFORMS,
                conditions,
                adaptive,
                adaptive_readings,
            );

            let resolve = transformer.resolve(segment);

            for item in &resolve {
                if let Some(it) = &item.0 {
                    for link in it.chain {
                        // print!(
                        //     "{} ({}) -> ",
                        //     JAPANESE_TRANSFORMS[link.0].name,
                        //     JAPANESE_TRANSFORMS[link.0].rules[link.1].conditions_out[0]
                        // );
                    }
                    // println!("");
                }
            }

            if resolve.is_empty() {
                output.push((segment.clone(), segment.clone(), None));
            }

            // If resolution created too many segments, but a single segment
            // exists in the dictionary, it's almost certainly the correct
            // choice.
            if (resolve.len() > 1 || resolve[0].1.len() > segment.len())
                && let Some(entry) = dict_entry
            {
                output.push(entry);
                continue;
            }

            {
                let mut out = resolve
                    .into_iter()
                    .map(|it| {
                        let base = it.1;

                        let inflected = it.0.map_or_else(
                            || (base.clone(), None),
                            |inflection| {
                                if inflection.last_inflection.is_empty() {
                                    return (base.clone(), None);
                                }

                                if inflection.last_inflection.len() == base.len() {
                                    return (inflection.inflection, None);
                                }

                                (
                                    [
                                        &base[..(base
                                            .len()
                                            .saturating_sub(inflection.last_inflection.len()))],
                                        &inflection.inflection,
                                    ]
                                    .join(""),
                                    inflection.replacement,
                                )
                            },
                        );

                        (inflected.0, base, inflected.1)
                    })
                    .collect::<Vec<_>>();

                output.append(&mut out);
            }
        } else {
            output.push((segment.clone(), segment.clone(), None));
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

        if !adaptive.is_prefix(&segment.0) && !adaptive_readings.is_prefix(&segment.0) {
            result.push(segment.clone());
            i += 1;
            continue;
        }

        let mut key_with_boundary = i + 1;
        let mut curr_check = i + 1;

        while curr_check < output.len() {
            let data = output[i..=curr_check].iter().map(|it| &it.0).join("");

            if either_dict_has_prefix(&data, adaptive, adaptive_readings) {
                if either_dict_has_exact(&data, adaptive, adaptive_readings) {
                    key_with_boundary = curr_check + 1;
                }

                curr_check += 1;
                continue;
            }

            break;
        }

        let entry = &output[i..key_with_boundary];
        i = key_with_boundary;

        if entry.len() == 1 {
            result.push(segment.clone());
        } else {
            result.push(
                entry
                    .iter()
                    .fold((String::new(), String::new(), None), |prev, curr| {
                        let a = format!("{}{}", prev.0, curr.0);

                        (a.clone(), a, None)
                    }),
            );
        }
    }

    result
        .into_iter()
        .map(|it| {
            let text = if let Some(it) = it.2 {
                it
            } else {
                it.1.clone()
            };

            let freq_a = adaptive_readings.exact_match(&text).flatten_ref();
            let freq_b = adaptive.exact_match(&text).flatten_ref();

            let freq = match (freq_a, freq_b) {
                (None, None) => None,
                (None, Some(b)) => Some(*b),
                (Some(a), None) => Some(*a),
                (Some(a), Some(b)) => Some(*a.min(b)),
            };

            (it.0, it.1, freq)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    #[test]
    fn thing() {
        // let timing_data = read_to_string("/home/nulliel/Japanese/bookworm.json").unwrap();
        // let timing_data: Transcription = serde_json::from_str(&timing_data).unwrap();

        // let pipeline = japanese_language_pipeline();

        /*
        for i in 0..25 {
            println!("{i}");
            pipeline.run(&timing_data, &mut timing_data);
        }
        */

        /*
        でこぼこしていて、気を付けて歩かないと、足を引っ掛けてすぐに転びさそうだ。手を繋いでくれてるトゥーリに周りのことは任せて、わたしは自分の足元だけを見て歩くことにした。
        「あれ？　トゥーリじゃん！　何やってんだ？」
        やや遠いところから響いてた男の子の声にわたしは顔を上げた。
        背負子と弓を持った男の子が三人、駆け寄ってくる。赤、金、ピンクの頭がカラフルで、ついつい髪の色に注目してしまった。三人が着てる服は土や食べ物の染みで斑模様の薄い灰色のような色になっていて、お下がりを着回してるのか、継ぎ接ぎだらけだ。自分達が着てる物と大して違いがない様子から、生活レベルは同じくらいだと思う。

        トゥーリ != トゥー + リ == トゥーリ
        駆け寄ってくる != 駆け + 寄 + っ + てくる == 駆け寄って + くる
        響いてた != 響い + てた == 響いてた

        */

        /*

                let text = "駆け寄ってくる";
                println!("{:#?}", JapaneseTextSegmenter::new().segment(text));

                panic!();

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
        */
        assert_eq!(1, 1);
    }
}
