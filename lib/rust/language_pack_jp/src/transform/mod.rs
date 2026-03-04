pub use transforms::*;

use crate::segment::Morpheme;

mod transforms;

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
    use std::fs::read_to_string;
    use std::io::BufRead;
    use std::io::BufReader;

    use blart::TreeMap;
    use itertools::Itertools;
    use language_pack::Transcription;
    use language_pack::segment::TextSegmenter;
    use language_pack::transform::TextTransform;

    use crate::japanese_language_pipeline;
    use crate::segment::JapaneseTextSegmenter;
    use crate::segment::Morpheme;
    use crate::transform::group_inflected;

    #[test]
    fn thing() {
        let timing_data = read_to_string("/home/nulliel/Japanese/bookworm.json").unwrap();
        let timing_data: Transcription = serde_json::from_str(&timing_data).unwrap();

        let pipeline = japanese_language_pipeline();

        for i in 0..25 {
            println!("{i}");
            pipeline.run(&timing_data, &timing_data);
        }

        return;

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
