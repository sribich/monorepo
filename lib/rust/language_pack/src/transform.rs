use std::ffi::CStr;
use std::ffi::CString;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use blart::TreeMap;
use tinyvec::ArrayVec;

pub struct LanguageTransformer<'a> {
    transforms: &'static [Transform],
    dictionary: &'a TreeMap<&'a CStr, Option<usize>>,
}

impl<'a> LanguageTransformer<'a> {
    pub fn new(
        transforms: &'static [Transform],
        dictionary: &'a TreeMap<&'a CStr, Option<usize>>,
    ) -> Self {
        Self {
            transforms,
            dictionary,
        }
    }

    pub fn resolve(&mut self, text: &str) -> Vec<(Option<TextTransform>, String)> {
        let mut work = text;
        let mut matches = vec![];

        while !work.is_empty() {
            let transform = TextTransform {
                text: work.to_owned(),
                inflection: String::new(),
                last_inflection: String::new(),
                conditions: &[],
                i: 0,
            };

            let (result, _) = self.resolve_from(transform);

            panic!();
            // println!("1");
            // println!("{result:#?}");

            let mut result = result
                .into_iter()
                .filter_map(|it| {
                    if it.i == 0 {
                        return None;
                    }

                    let mut longest_match = None;

                    for i in 0..24 {
                        if let Some((index, _)) = it.text.char_indices().nth_back(i)
                            && self.dictionary.contains_key(
                                CString::new(it.text[index..].as_bytes())
                                    .unwrap()
                                    .as_c_str(),
                            )
                        {
                            longest_match = Some(index);
                        }
                    }

                    longest_match.map(|len| {
                        let text = it.text[len..].to_owned();
                        (it, text)
                    })
                })
                .collect::<Vec<_>>();

            if result.is_empty() {
                matches.push((None, work.chars().last().unwrap().to_string()));

                if let Some((index, _)) = work.char_indices().nth_back(0) {
                    work = &text[0..index];
                }

                continue;
            }

            // println!("2");
            // println!("{result:#?}");

            result.sort_by(|a, b| b.0.i.cmp(&a.0.i).then(b.1.len().cmp(&a.1.len())));

            // We got the same result out, that means the output was already set once
            if work == &text[0..(result[0].0.text.len() - result[0].1.len())] {
                break;
            }

            work = &text[0..(result[0].0.text.len() - result[0].1.len())];

            matches.push((Some(result[0].0.clone()), result[0].1.clone()));
        }

        matches.reverse();

        matches
    }

    fn resolve_from(
        &self,
        resolve: TextTransform,
    ) -> (Vec<TextTransform>, ArrayVec<[(usize, usize); 12]>) {
        let mut seen = ArrayVec::<[(usize, usize); 12]>::new();

        let mut result = vec![];

        println!("{:#?}", resolve);

        for (transform_pos, transform) in self.transforms.iter().enumerate() {
            for (rule_pos, rule) in transform.rules.iter().enumerate() {
                if seen.contains(&(transform_pos, rule_pos)) {
                    println!(".");
                    continue;
                }

                if (resolve.conditions.is_empty()
                    || rule.conditions_in.is_empty()
                    || resolve.conditions.contains(&rule.conditions_in[0]))
                    && let Some(InflectionResult {
                        text,
                        inflection,
                        deinflection,
                        conditions,
                    }) = rule.deinflect(&resolve)
                {
                    seen.push((transform_pos, rule_pos));

                    let (mut data, mut sub_seen) = self.resolve_from(TextTransform {
                        text,
                        inflection: format!("{}{}", inflection, resolve.inflection),
                        last_inflection: deinflection,
                        conditions,
                        i: resolve.i + 1,
                    });

                    result.append(&mut data);
                    // seen.append(&mut sub_seen);
                }
            }
        }

        result.push(resolve);

        (result, seen)
    }
}

#[derive(Debug)]
pub struct Inflection {
    kind: InflectionKind,
    test: &'static str,
    deinflection: &'static str,
    conditions_in: &'static [&'static str],
    conditions_out: &'static [&'static str],
}

#[derive(Debug)]
pub enum InflectionKind {
    Suffix,
}

pub const fn suffix_inflection(
    test: &'static str,
    deinflection: &'static str,
    conditions_in: &'static [&'static str],
    conditions_out: &'static [&'static str],
) -> Inflection {
    Inflection {
        kind: InflectionKind::Suffix,
        test,
        deinflection,
        conditions_in,
        conditions_out,
    }
}

pub struct InflectionResult {
    text: String,
    inflection: String,
    deinflection: String,
    conditions: &'static [&'static str],
}

impl Inflection {
    pub fn deinflect(&self, text: &TextTransform) -> Option<InflectionResult> {
        let len = text.text.len();

        match self.kind {
            InflectionKind::Suffix => {
                if text.text.ends_with(self.test) {
                    return Some(InflectionResult {
                        text: [&text.text[0..(len - self.test.len())], self.deinflection].join(""),
                        inflection: self.test
                            [0..(self.test.len().saturating_sub(text.last_inflection.len()))]
                            .to_string(),
                        deinflection: self.deinflection.to_owned(),
                        conditions: self.conditions_out,
                    });
                }
            }
        }

        None
    }
}

pub struct Transform {
    pub name: &'static str,
    pub description: &'static str,
    pub rules: &'static [Inflection],
}

#[derive(Clone, Debug)]
pub struct TextTransform {
    pub text: String,
    pub inflection: String,
    pub last_inflection: String,
    pub conditions: &'static [&'static str],
    pub i: usize,
}
