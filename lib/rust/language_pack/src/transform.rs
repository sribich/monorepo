use std::collections::HashMap;

use tinyvec::ArrayVec;
use trie_rs::map::Trie;

pub struct LanguageTransformer<'a> {
    transforms: &'static [Transform],
    conditions: &'static [(u32, Condition)],
    dictionary: &'a Trie<u8, Option<usize>>,
    dictionary_readings: &'a Trie<u8, Option<usize>>,
}

impl<'a> LanguageTransformer<'a> {
    pub fn new(
        transforms: &'static [Transform],
        conditions: &'static [(u32, Condition)],
        dictionary: &'a Trie<u8, Option<usize>>,
        dictionary_readings: &'a Trie<u8, Option<usize>>,
    ) -> Self {
        Self {
            transforms,
            conditions,
            dictionary,
            dictionary_readings,
        }
    }

    // TODO: Filter our results which collapse down to a single mora.
    pub fn resolve(&mut self, text: &str) -> Vec<(Option<TextTransform>, String)> {
        let mut work = text;
        let mut matches = vec![];

        while !work.is_empty() {
            let transform = TextTransform::new(work.to_owned());

            let result = self.resolve_from(transform, ArrayVec::<[(usize, usize); 14]>::new());

            let mut result = result
                .into_iter()
                .filter_map(|it| {
                    if it.i == 0 {
                        return None;
                    }

                    let inflected_len =
                        &it.text[..(it.text.len().saturating_sub(it.last_inflection.len()))].len()
                            + it.inflection.len();

                    // Do not accept inflected results longer than the original.
                    if inflected_len > work.len() {
                        return None;
                    }

                    let text = it.replacement.as_ref().map_or(&it.text, |it| it);

                    if let Some(freq) = self.dictionary.exact_match(text) {
                        let text = it.text.clone();
                        return Some((it, text, *freq));
                    }

                    if let Some(freq) = self.dictionary_readings.exact_match(text) {
                        let text = it.text.clone();
                        return Some((it, text, *freq));
                    }

                    None
                    /*
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
                     */
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

            result.sort_by(|a, b| {
                // a.2.cmp(&b.2)

                // /*
                b.0.i
                    .cmp(&a.0.i)
                    .then(b.2.cmp(&a.2))
                    .then(b.1.len().cmp(&a.1.len()))
                // */
            });

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
        seen: ArrayVec<[(usize, usize); 14]>,
    ) -> Vec<TextTransform> {
        let mut result = vec![];

        for (transform_pos, transform) in self.transforms.iter().enumerate() {
            for (rule_pos, rule) in transform.rules.iter().enumerate() {
                let condition_matches =
                    resolve.conditions == 0 || ((resolve.conditions & rule.conditions_in) != 0);

                if condition_matches
                    && let Some(InflectionResult {
                        text,
                        inflection,
                        deinflection,
                        conditions,
                        replacement,
                    }) = rule.deinflect(&resolve)
                {
                    if seen.contains(&(transform_pos, rule_pos)) {
                        continue;
                    }

                    let mut child_seen = seen;
                    child_seen.push((transform_pos, rule_pos));

                    let mut data = self.resolve_from(
                        TextTransform {
                            text,
                            replacement,
                            inflection: format!("{}{}", inflection, resolve.inflection),
                            last_inflection: deinflection,
                            conditions,
                            chain: child_seen,
                            i: resolve.i + 1,
                        },
                        child_seen,
                    );

                    result.append(&mut data);
                }
            }
        }

        result.push(resolve);

        result
    }
}

#[derive(Debug)]
pub struct Inflection {
    pub kind: InflectionKind,
    pub test: &'static str,
    pub deinflection: &'static str,
    pub conditions_in: u32,  // &'static [&'static str],
    pub conditions_out: u32, // &'static [&'static str],
    pub replacements: &'static [(&'static str, &'static str)],
}

#[derive(Debug)]
pub enum InflectionKind {
    Suffix,
}

pub const fn suffix_inflection(
    test: &'static str,
    deinflection: &'static str,
    conditions_in: u32,
    conditions_out: u32,
) -> Inflection {
    Inflection {
        kind: InflectionKind::Suffix,
        test,
        deinflection,
        conditions_in,
        conditions_out,
        replacements: &[],
    }
}

pub const fn suffix_inflection_with_replacements(
    test: &'static str,
    deinflection: &'static str,
    conditions_in: u32,
    conditions_out: u32,
    replacements: &'static [(&'static str, &'static str)],
) -> Inflection {
    Inflection {
        kind: InflectionKind::Suffix,
        test,
        deinflection,
        conditions_in,
        conditions_out,
        replacements,
    }
}

#[derive(Debug)]
pub struct InflectionResult {
    text: String,
    replacement: Option<String>,
    inflection: String,
    deinflection: String,
    conditions: u32,
}

impl Inflection {
    pub fn deinflect(&self, text: &TextTransform) -> Option<InflectionResult> {
        let len = text.text.len();

        match self.kind {
            InflectionKind::Suffix => {
                if text.text.ends_with(self.test) {
                    let replacement = self
                        .replacements
                        .iter()
                        .find(|(a, _)| text.text.ends_with(a))
                        .map(|it| {
                            let mut text = text.text.clone();
                            text.replace_last(it.0, it.1);

                            [&text[0..(len - self.test.len())], self.deinflection].join("")
                        });

                    return Some(InflectionResult {
                        text: [&text.text[0..(len - self.test.len())], self.deinflection].join(""),
                        replacement,
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

pub struct Condition {
    pub name: &'static str,
    pub short: &'static str,
    pub description: &'static str,
    pub is_dictionary_form: bool,
    pub subconditions: u32,
}

pub struct Transform {
    pub name: &'static str,
    pub description: &'static str,
    pub rules: &'static [Inflection],
}

// TODO: Since we know exactly what the inflection/last_inflection values
//       are, we should be able to make them &'static str
#[derive(Clone, Debug)]
pub struct TextTransform {
    pub text: String,
    pub replacement: Option<String>,
    pub inflection: String,
    pub last_inflection: String,
    pub conditions: u32,
    pub chain: ArrayVec<[(usize, usize); 14]>,
    pub i: usize,
}

impl TextTransform {
    pub fn new(text: String) -> Self {
        Self {
            text,
            replacement: None,
            inflection: String::new(),
            last_inflection: String::new(),
            conditions: 0,
            chain: ArrayVec::<[(usize, usize); 14]>::new(),
            i: 0,
        }
    }
}
