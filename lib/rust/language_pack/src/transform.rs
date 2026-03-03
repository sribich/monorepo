use std::ffi::CStr;
use std::ffi::CString;

use blart::TreeMap;

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

    pub fn resolve(&mut self, text: &str) -> Vec<(Option<TextIdk>, String)> {
        let mut work = &text[..];
        let mut matches = vec![];

        while !work.is_empty() {
            let l = TextIdk {
                text: work.to_owned(),
                inflection: String::new(),
                last_inflection: String::new(),
                conditions: &[],
                i: 0,
            };

            let result = self.resolve_from(l);

            let mut result = result
                .into_iter()
                .filter_map(|it| {
                    if it.i == 0 {
                        return None;
                    }

                    let mut longest_match = None;

                    for i in 0..24 {
                        if let Some((index, _)) = it.text.char_indices().nth_back(i) {
                            if self.dictionary.contains_key(
                                CString::new(it.text[index..].as_bytes())
                                    .unwrap()
                                    .as_c_str(),
                            ) {
                                longest_match = Some(index);
                            }
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

    fn resolve_from(&self, resolve: TextIdk) -> Vec<TextIdk> {
        let mut result = self
            .transforms
            .iter()
            .flat_map(|transform| {
                transform
                    .rules
                    .iter()
                    .filter_map(|rule| {
                        if (resolve.conditions.is_empty()/*&& rule.conditions_in.is_empty()*/)
                            || (!rule.conditions_in.is_empty()
                                && resolve.conditions.contains(&rule.conditions_in[0]))
                        {
                            if let Some(InflectionResult {
                                text,
                                inflection,
                                deinflection,
                                conditions,
                            }) = rule.deinflect(&resolve)
                            {
                                // Cycle
                                if text == resolve.text {
                                    return None;
                                }

                                return Some(self.resolve_from(TextIdk {
                                    text,
                                    inflection: format!("{}{}", inflection, resolve.inflection),
                                    last_inflection: deinflection,
                                    conditions,
                                    i: resolve.i + 1,
                                }));
                            }
                        }

                        None
                    })
                    .flatten()
            })
            .collect::<Vec<_>>();

        result.push(resolve);
        result
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
    pub fn deinflect(&self, text: &TextIdk) -> Option<InflectionResult> {
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

pub struct TextTransform {
    text: String,
    resolved_segments: Vec<String>,
}

enum TransformResult {
    Resolved {},
    Unresolved { text: String },
}

#[derive(Clone, Debug)]
pub struct TextIdk {
    pub text: String,
    pub inflection: String,
    pub last_inflection: String,
    pub conditions: &'static [&'static str],
    pub i: usize,
}

impl TextTransform {
    pub fn new(text: String) -> Self {
        Self {
            text,
            resolved_segments: vec![],
        }
    }
}
