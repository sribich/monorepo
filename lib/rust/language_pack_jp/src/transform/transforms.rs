use std::collections::HashMap;
use std::ops::BitOr;
use std::sync::LazyLock;

use indoc::indoc;
use language_pack::transform::Condition;
use language_pack::transform::Transform;
use language_pack::transform::suffix_inflection;
use language_pack::transform::suffix_inflection_with_replacements;

const passive_description: &str = indoc! {"

"};

struct Conditions(u32);

impl Conditions {
    const adj_i: Self = Self(1 << 12);
    const ba: Self = Self(1 << 16);
    const ku: Self = Self(1 << 17);
    const masen: Self = Self(1 << 14);
    const masu: Self = Self(1 << 13);
    const n: Self = Self(1 << 19);
    const nasai: Self = Self(1 << 20);
    const ta: Self = Self(1 << 18);
    const te: Self = Self(1 << 15);
    const v: Self = Self(1);
    const v1: Self = Self(1 << 1);
    const v1d: Self = Self(1 << 2);
    const v1p: Self = Self(1 << 3);
    const v5: Self = Self(1 << 4);
    const v5d: Self = Self(1 << 5);
    const v5s: Self = Self(1 << 6);
    const v5sp: Self = Self(1 << 8);
    const v5ss: Self = Self(1 << 7);
    const vk: Self = Self(1 << 9);
    const vs: Self = Self(1 << 10);
    const vz: Self = Self(1 << 11);
    const ya: Self = Self(1 << 21);
}

impl Conditions {
    pub const fn with_subconditions(&self) -> u32 {
        const fn subcondition_fold(prev: u32, curr: u32) -> u32 {
            let mut i = 0;
            let num_conditions = JAPANESE_CONDITIONS.len();

            while i < num_conditions {
                let condition = &JAPANESE_CONDITIONS[i];

                if condition.0 == curr {
                    return prev | condition.1.subconditions;
                }

                i += 1;
            }

            prev
        }

        self.fold(self.0, &subcondition_fold)
    }

    pub const fn fold<F>(&self, initial: u32, f: &F) -> u32
    where
        F: const Fn(u32, u32) -> u32,
    {
        let mut curr = initial;

        let mut i = 0;
        let end = 21;

        while i <= end {
            if (1 << i) & initial == (i << i) {
                curr = f(curr, 1 << i)
            }

            i += 1;
        }

        curr
    }
}

impl const From<Conditions> for u32 {
    fn from(value: Conditions) -> Self {
        value.0
    }
}

impl const BitOr for Conditions {
    type Output = Conditions;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub const JAPANESE_CONDITIONS: &'static [(u32, Condition)] = &[
    (
        Conditions::v.into(),
        Condition {
            name: "Verb",
            short: "v",
            description: "動詞",
            is_dictionary_form: false,
            subconditions: (Conditions::v1
                | Conditions::v5
                | Conditions::vk
                | Conditions::vs
                | Conditions::vz)
                .into(),
        },
    ),
    (
        Conditions::v1.into(),
        Condition {
            name: "Ichidan verb",
            short: "v1",
            description: "一段動詞",
            is_dictionary_form: true,
            subconditions: (Conditions::v1d | Conditions::v1p).into(),
        },
    ),
    (
        Conditions::v1d.into(),
        Condition {
            name: "Ichidan verb, dictionary form",
            short: "v1d",
            description: "一段動詞、終止形",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::v1p.into(),
        Condition {
            name: "Ichidan verb, progressive or perfect form",
            short: "v1p",
            description: "一段動詞、～てる・でる",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::v5.into(),
        Condition {
            name: "Godan verb",
            short: "v5",
            description: "五段動詞",
            is_dictionary_form: true,
            subconditions: (Conditions::v5d | Conditions::v5s).into(),
        },
    ),
    (
        Conditions::v5d.into(),
        Condition {
            name: "Godan verb, dictionary form",
            short: "v5d",
            description: "五段動詞、終止形",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::v5s.into(),
        Condition {
            name: "Godan verb, short causative form",
            short: "v5s",
            description: "五段動詞、～す・さす",
            is_dictionary_form: false,
            subconditions: (Conditions::v5ss | Conditions::v5sp).into(),
        },
    ),
    (
        Conditions::v5ss.into(),
        Condition {
            name: "Godan verb, short causative form having さす ending (cannot conjugate with passive form)",
            short: "v5ss",
            description: "五段動詞、～さす",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::v5sp.into(),
        Condition {
            name: "Godan verb, short causative form not having さす ending (can conjugate with passive form)",
            short: "v5sp",
            description: "五段動詞、～す",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::vk.into(),
        Condition {
            name: "Kuru verb",
            short: "vk",
            description: "来る動詞",
            is_dictionary_form: true,
            subconditions: 0,
        },
    ),
    (
        Conditions::vs.into(),
        Condition {
            name: "Suru verb",
            short: "vs",
            description: "する動詞",
            is_dictionary_form: true,
            subconditions: 0,
        },
    ),
    (
        Conditions::vz.into(),
        Condition {
            name: "Zuru verb",
            short: "vz",
            description: "ずる動詞",
            is_dictionary_form: true,
            subconditions: 0,
        },
    ),
    (
        Conditions::adj_i.into(),
        Condition {
            name: "Adjective with i ending",
            short: "adj-i",
            description: "形容詞",
            is_dictionary_form: true,
            subconditions: 0,
        },
    ),
    (
        Conditions::masu.into(),
        Condition {
            name: "Polite -ます ending",
            short: "-ます",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::masen.into(),
        Condition {
            name: "Polite negative -ません ending",
            short: "-ません",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::te.into(),
        Condition {
            name: "Intermediate -て endings for progressive or perfect tense",
            short: "-て",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::ba.into(),
        Condition {
            name: "Intermediate -ば endings for conditional contraction",
            short: "-ば",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::ku.into(),
        Condition {
            name: "Intermediate -く endings for adverbs",
            short: "-く",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::ta.into(),
        Condition {
            name: "-た form ending",
            short: "-た",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::n.into(),
        Condition {
            name: "-ん negative ending",
            short: "-ん",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::nasai.into(),
        Condition {
            name: "Intermediate -なさい ending (polite imperative)",
            short: "-なさい",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
    (
        Conditions::ya.into(),
        Condition {
            name: "Intermediate -や ending (conditional contraction)",
            short: "-ゃ",
            description: "",
            is_dictionary_form: false,
            subconditions: 0,
        },
    ),
];

/// // Irregular (iku verbs)
/// suffix_inflection("いっ", "いく", &[], &[]),
/// suffix_inflection("行っ", "行く", &[], &[]),
/// suffix_inflection("逝っ", "逝く", &[], &[]),
/// suffix_inflection("往っ", "往く", &[], &[]),
/// // Irregular (godan u special)
/// suffix_inflection("こう", "こう", &[], &[]),
/// suffix_inflection("とう", "とう", &[], &[]),
/// suffix_inflection("請う", "請う", &[], &[]),
/// suffix_inflection("乞う", "乞う", &[], &[]),
/// suffix_inflection("恋う", "恋う", &[], &[]),
/// suffix_inflection("問う", "問う", &[], &[]),
/// suffix_inflection("訪う", "訪う", &[], &[]),
/// suffix_inflection("宣う", "宣う", &[], &[]),
/// suffix_inflection("曰う", "曰う", &[], &[]),
/// suffix_inflection("給う", "給う", &[], &[]),
/// suffix_inflection("賜う", "賜う", &[], &[]),
/// suffix_inflection("揺蕩う", "揺蕩う", &[], &[]),
/// // Irregular (fu verb te conjugations)
/// suffix_inflection("のたもう", "のたまう", &[], &[]),
/// suffix_inflection("たもう", "たまう", &[], &[]),
/// suffix_inflection("たゆとう", "たゆたう", &[], &[]),
pub const JAPANESE_TRANSFORMS: &[Transform] = &[
    Transform {
        name: "-ば",
        description: indoc! {"
            1. Conditional form; shows that the previous stated condition's establishment is the condition for the latter stated condition to occur.
            2. Shows a trigger for a latter stated perception or judgment.

            Usage: Attach ば to the hypothetical form (仮定形) of verbs and i-adjectives.
        "},
        rules: &[
            suffix_inflection(
                "ければ",
                "い",
                Conditions::ba.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection("えば", "う", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("けば", "く", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("げば", "ぐ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("せば", "す", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("てば", "つ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ねば", "ぬ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("べば", "ぶ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("めば", "む", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection(
                "れば",
                "る",
                Conditions::ba.with_subconditions(),
                (Conditions::v1
                    | Conditions::v5
                    | Conditions::vk
                    | Conditions::vs
                    | Conditions::vz)
                    .with_subconditions(),
            ),
            suffix_inflection("れば", "", Conditions::ba.with_subconditions(), Conditions::masu.with_subconditions()),
        ],
    },
    Transform {
        name: "-ゃ",
        description: indoc! {"
            Contraction of -ば.
        "},
        rules: &[
            suffix_inflection(
                "けりゃ",
                "ければ",
                Conditions::ya.with_subconditions(),
                Conditions::ba.with_subconditions(),
            ),
            suffix_inflection(
                "きゃ",
                "ければ",
                Conditions::ya.with_subconditions(),
                Conditions::ba.with_subconditions(),
            ),
            suffix_inflection("や", "えば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("きゃ", "けば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("ぎゃ", "げば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("しゃ", "せば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("ちゃ", "てば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("にゃ", "ねば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("びゃ", "べば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("みゃ", "めば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
            suffix_inflection("りゃ", "れば", Conditions::ya.with_subconditions(), Conditions::ba.with_subconditions()),
        ],
    },
    Transform {
        name: "-ちゃ",
        description: indoc! {"
            Contraction of ～ては.

            1. Explains how something always happens under the condition that it marks.
            2. Expresses the repetition (of a series of) actions.
            3. Indicates a hypothetical situation in which the speaker gives a (negative) evaluation about the other party's intentions.
            4. Used in \"Must Not\" patterns like ～てはいけない.

            Usage: Attach は after the て-form of verbs, contract ては into ちゃ.
        "},
        rules: &[
            suffix_inflection("ちゃ", "る", Conditions::v5.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection("いじゃ", "ぐ", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("いちゃ", "く", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("しちゃ", "す", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("っちゃ", "う", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("っちゃ", "く", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("っちゃ", "つ", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("っちゃ", "る", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("んじゃ", "ぬ", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("んじゃ", "ぶ", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("んじゃ", "む", Conditions::v5.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection(
                "じちゃ",
                "ずる",
                Conditions::v5.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "しちゃ",
                "する",
                Conditions::v5.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為ちゃ",
                "為る",
                Conditions::v5.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "きちゃ",
                "くる",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来ちゃ",
                "来る",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來ちゃ",
                "來る",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-ちゃう",
        description: indoc! {"
            Contraction of -しまう.

            1. Shows a sense of regret/surprise when you did have volition in doing something, but it turned out to be bad to do.
            2. Shows perfective/punctual achievement. This shows that an action has been completed.
            3. Shows unintentional action–\"accidentally\".

            Usage: Attach しまう after the て-form of verbs, contract てしまう into ちゃう.
        "},
        rules: &[
            suffix_inflection("ちゃう", "る", Conditions::v5.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection(
                "いじゃう",
                "ぐ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "いちゃう",
                "く",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "しちゃう",
                "す",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちゃう",
                "う",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちゃう",
                "く",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちゃう",
                "つ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちゃう",
                "る",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "んじゃう",
                "ぬ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "んじゃう",
                "ぶ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "んじゃう",
                "む",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "じちゃう",
                "ずる",
                Conditions::v5.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "しちゃう",
                "する",
                Conditions::v5.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為ちゃう",
                "為る",
                Conditions::v5.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "きちゃう",
                "くる",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来ちゃう",
                "来る",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來ちゃう",
                "來る",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-ちまう",
        description: indoc! {"
            Contraction of -しまう.

            1. Shows a sense of regret/surprise when you did have volition in doing something, but it turned out to be bad to do.
            2. Shows perfective/punctual achievement. This shows that an action has been completed.
            3. Shows unintentional action–\"accidentally\".

            Usage: Attach しまう after the て-form of verbs, contract てしまう into ちまう.
        "},
        rules: &[
            suffix_inflection("ちまう", "る", Conditions::v5.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection(
                "いじまう",
                "ぐ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "いちまう",
                "く",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "しちまう",
                "す",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちまう",
                "う",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちまう",
                "く",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちまう",
                "つ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "っちまう",
                "る",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "んじまう",
                "ぬ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "んじまう",
                "ぶ",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "んじまう",
                "む",
                Conditions::v5.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "じちまう",
                "ずる",
                Conditions::v5.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "しちまう",
                "する",
                Conditions::v5.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為ちまう",
                "為る",
                Conditions::v5.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "きちまう",
                "くる",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来ちまう",
                "来る",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來ちまう",
                "來る",
                Conditions::v5.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-しまう",
        description: indoc! {"
            1. Shows a sense of regret/surprise when you did have volition in doing something, but it turned out to be bad to do.
            2. Shows perfective/punctual achievement. This shows that an action has been completed.
            3. Shows unintentional action–\"accidentally\".

            Usage: Attach しまう after the て-form of verbs.
        "},
        rules: &[
            suffix_inflection(
                "てしまう",
                "て",
                Conditions::v5.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "でしまう",
                "で",
                Conditions::v5.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-なさい",
        description: indoc! {"
            Polite imperative suffix.

            Usage: Attach なさい after the continuative form (連用形) of verbs.
        "},
        rules: &[
            suffix_inflection(
                "なさい",
                "る",
                Conditions::nasai.with_subconditions(),
                Conditions::v1.with_subconditions(),
            ),
            suffix_inflection(
                "いなさい",
                "う",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "きなさい",
                "く",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ぎなさい",
                "ぐ",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "しなさい",
                "す",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ちなさい",
                "つ",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "になさい",
                "ぬ",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "びなさい",
                "ぶ",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "みなさい",
                "む",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "りなさい",
                "る",
                Conditions::nasai.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "じなさい",
                "ずる",
                Conditions::nasai.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "しなさい",
                "する",
                Conditions::nasai.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為なさい",
                "為る",
                Conditions::nasai.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "きなさい",
                "くる",
                Conditions::nasai.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来なさい",
                "来る",
                Conditions::nasai.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來なさい",
                "來る",
                Conditions::nasai.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-そう",
        description: indoc! {"
            Appearing that; looking like.

            Usage: Attach そう to the continuative form (連用形) of verbs, or to the stem of adjectives.
        "},
        rules: &[
            suffix_inflection("そう", "い", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("そう", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("いそう", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("きそう", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぎそう", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("しそう", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ちそう", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("にそう", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("びそう", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("みそう", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("りそう", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("じそう", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("しそう", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為そう", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("きそう", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来そう", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來そう", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "-すぎる",
        description: indoc! {"
            Shows something \"is too...\" or someone is doing something \"too much\".

            Usage: Attach すぎる to the continuative form (連用形) of verbs, or to the stem of adjectives.'
        "},
        rules: &[
            suffix_inflection(
                "すぎる",
                "い",
                Conditions::v1.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection("すぎる", "る", Conditions::v1.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection(
                "いすぎる",
                "う",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "きすぎる",
                "く",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ぎすぎる",
                "ぐ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "しすぎる",
                "す",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ちすぎる",
                "つ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "にすぎる",
                "ぬ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "びすぎる",
                "ぶ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "みすぎる",
                "む",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "りすぎる",
                "る",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "じすぎる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "しすぎる",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為すぎる",
                "為る",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "きすぎる",
                "くる",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来すぎる",
                "来る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來すぎる",
                "來る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-過ぎる",
        description: indoc! {"
            Shows something \"is too...\" or someone is doing something \"too much\".

            Usage: Attach 過ぎる to the continuative form (連用形) of verbs, or to the stem of adjectives.
        "},
        rules: &[
            suffix_inflection(
                "過ぎる",
                "い",
                Conditions::v1.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection("過ぎる", "る", Conditions::v1.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection(
                "い過ぎる",
                "う",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "き過ぎる",
                "く",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ぎ過ぎる",
                "ぐ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "し過ぎる",
                "す",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ち過ぎる",
                "つ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "に過ぎる",
                "ぬ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "び過ぎる",
                "ぶ",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "み過ぎる",
                "む",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "り過ぎる",
                "る",
                Conditions::v1.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "じ過ぎる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "し過ぎる",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為過ぎる",
                "為る",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "き過ぎる",
                "くる",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来過ぎる",
                "来る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來過ぎる",
                "來る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-たい",
        description: indoc! {"
            1. Expresses the feeling of desire or hope.
            2. Used in ...たいと思います, an indirect way of saying what the speaker intends to do.

            Usage: Attach たい to the continuative form (連用形) of verbs. たい itself conjugates as i-adjective.
        "},
        rules: &[
            suffix_inflection(
                "たい",
                "る",
                Conditions::adj_i.with_subconditions(),
                Conditions::v1.with_subconditions(),
            ),
            suffix_inflection(
                "いたい",
                "う",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "きたい",
                "く",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ぎたい",
                "ぐ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "したい",
                "す",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ちたい",
                "つ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "にたい",
                "ぬ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "びたい",
                "ぶ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "みたい",
                "む",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "りたい",
                "る",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "じたい",
                "ずる",
                Conditions::adj_i.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "したい",
                "する",
                Conditions::adj_i.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為たい",
                "為る",
                Conditions::adj_i.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "きたい",
                "くる",
                Conditions::adj_i.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来たい",
                "来る",
                Conditions::adj_i.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來たい",
                "來る",
                Conditions::adj_i.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-たら",
        description: indoc! {"
            1. Denotes the latter stated event is a continuation of the previous stated event.
            2. Assumes that a matter has been completed or concluded.

            Usage: Attach たら to the continuative form (連用形) of verbs after euphonic change form, かったら to the stem of i-adjectives.
        "},
        rules: &[
            suffix_inflection("かったら", "い", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("たら", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("いたら", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("いだら", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("したら", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ったら", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ったら", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ったら", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("んだら", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("んだら", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("んだら", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("じたら", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("したら", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為たら", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("きたら", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来たら", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來たら", "來る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("ましたら", "ます", 0, Conditions::masu.with_subconditions()),
            // Irregular (iku verbs)
            suffix_inflection("いったら", "いく", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("行ったら", "行く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("逝ったら", "逝く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("往ったら", "往く", 0, Conditions::v5.with_subconditions()),
            // Irregular (godan u special)
            suffix_inflection("こうたら", "こう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("とうたら", "とう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("請うたら", "請う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("乞うたら", "乞う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("恋うたら", "恋う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("問うたら", "問う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("訪うたら", "訪う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("宣うたら", "宣う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("曰うたら", "曰う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("給うたら", "給う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("賜うたら", "賜う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("揺蕩うたら", "揺蕩う", 0, Conditions::v5.with_subconditions()),
            // Irregular (fu verb te conjugations)
            suffix_inflection("のたもうたら", "のたまう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たもうたら", "たまう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たゆとうたら", "たゆたう", 0, Conditions::v5.with_subconditions()),
        ],
    },
    Transform {
        name: "-たり",
        description: indoc! {"
            1. Shows two actions occurring back and forth (when used with two verbs).
            2. Shows examples of actions and states (when used with multiple verbs and adjectives).

            Usage: Attach たり to the continuative form (連用形) of verbs after euphonic change form, かったり to the stem of i-adjectives
        "},
        rules: &[
            suffix_inflection("かったり", "い", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("たり", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("いたり", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("いだり", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("したり", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ったり", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ったり", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ったり", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("んだり", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("んだり", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("んだり", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("じたり", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("したり", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為たり", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("きたり", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来たり", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來たり", "來る", 0, Conditions::vk.with_subconditions()),
            // Irregular (iku verbs)
            suffix_inflection("いったり", "いく", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("行ったり", "行く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("逝ったり", "逝く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("往ったり", "往く", 0, Conditions::v5.with_subconditions()),
            // Irregular (godan u special)
            suffix_inflection("こうたり", "こう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("とうたり", "とう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("請うたり", "請う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("乞うたり", "乞う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("恋うたり", "恋う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("問うたり", "問う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("訪うたり", "訪う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("宣うたり", "宣う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("曰うたり", "曰う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("給うたり", "給う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("賜うたり", "賜う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("揺蕩うたり", "揺蕩う", 0, Conditions::v5.with_subconditions()),
            // Irregular (fu verb te conjugations)
            suffix_inflection("のたもうたり", "のたまう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たもうたり", "たまう", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たゆとうたり", "たゆたう", 0, Conditions::v5.with_subconditions()),
        ],
    },
    Transform {
        name: "-て",
        description: indoc! {"
            て-form.

            It has a myriad of meanings. Primarily, it is a conjunctive particle that connects two clauses together.

            Usage: Attach て to the continuative form (連用形) of verbs after euphonic change form, くて to the stem of i-adjectives.
        "},
        rules: &[
            suffix_inflection(
                "くて",
                "い",
                Conditions::te.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection("て", "る", Conditions::te.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection("いて", "く", Conditions::te.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("いで", "ぐ", Conditions::te.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("して", "す", Conditions::te.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("って", "う", Conditions::te.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("って", "つ", Conditions::te.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("って", "る", Conditions::te.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection_with_replacements(
                "んで",
                "ぬ",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
                &[("こんで", "込んで")],
            ),
            suffix_inflection_with_replacements(
                "んで",
                "ぶ",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
                &[("こんで", "込んで")],
            ),
            suffix_inflection_with_replacements(
                "んで",
                "む",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
                &[("こんで", "込んで")],
            ),
            suffix_inflection("じて", "ずる", Conditions::te.with_subconditions(), Conditions::vz.with_subconditions()),
            suffix_inflection("して", "する", Conditions::te.with_subconditions(), Conditions::vs.with_subconditions()),
            suffix_inflection("為て", "為る", Conditions::te.with_subconditions(), Conditions::vs.with_subconditions()),
            suffix_inflection("きて", "くる", Conditions::te.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection("来て", "来る", Conditions::te.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection("來て", "來る", Conditions::te.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection("まして", "ます", 0, Conditions::masu.with_subconditions()),
            // Irregular (iku verbs)
            suffix_inflection(
                "いって",
                "いく",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "行って",
                "行く",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "逝って",
                "逝く",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "往って",
                "往く",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            // Irregular (godan u special)
            suffix_inflection(
                "こうて",
                "こう",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "とうて",
                "とう",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "請うて",
                "請う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "乞うて",
                "乞う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "恋うて",
                "恋う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "問うて",
                "問う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "訪うて",
                "訪う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "宣うて",
                "宣う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "曰うて",
                "曰う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "給うて",
                "給う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "賜うて",
                "賜う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "揺蕩うて",
                "揺蕩う",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            // Irregular (fu verb te conjugations)
            suffix_inflection(
                "のたもうて",
                "のたまう",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "たもうて",
                "たまう",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "たゆとうて",
                "たゆたう",
                Conditions::te.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-ず",
        description: indoc! {"
            1. Negative form of verbs.
            2. Continuative form (連用形) of the particle ぬ (nu).

            Usage: Attach ず to the irrealis form (未然形) of verbs.
        "},
        rules: &[
            suffix_inflection("ず", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("かず", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("がず", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("さず", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たず", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("なず", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ばず", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("まず", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("らず", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("わず", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぜず", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("せず", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為ず", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こず", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来ず", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來ず", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "-ぬ",
        description: indoc! {"
            Negative form of verbs.

            Usage: Attach ぬ to the irrealis form (未然形) of verbs.

            Irregularities: する becomes せぬ.
        "},
        rules: &[
            suffix_inflection("ぬ", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("かぬ", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("がぬ", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("さぬ", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たぬ", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("なぬ", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ばぬ", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("まぬ", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("らぬ", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("わぬ", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぜぬ", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("せぬ", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為ぬ", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こぬ", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来ぬ", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來ぬ", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "-ん",
        description: indoc! {"
            Negative form of verbs; a sound change of ぬ.

            Usage: Attach ん to the irrealis form (未然形) of verbs.

            Irregularities: する becomes せん.
        "},
        rules: &[
            suffix_inflection("ん", "る", Conditions::n.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection("かん", "く", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("がん", "ぐ", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("さん", "す", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("たん", "つ", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("なん", "ぬ", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ばん", "ぶ", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("まん", "む", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("らん", "る", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("わん", "う", Conditions::n.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ぜん", "ずる", Conditions::n.with_subconditions(), Conditions::vz.with_subconditions()),
            suffix_inflection("せん", "する", Conditions::n.with_subconditions(), Conditions::vs.with_subconditions()),
            suffix_inflection("為ん", "為る", Conditions::n.with_subconditions(), Conditions::vs.with_subconditions()),
            suffix_inflection("こん", "くる", Conditions::n.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection("来ん", "来る", Conditions::n.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection("來ん", "來る", Conditions::n.with_subconditions(), Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "-んばかり",
        description: indoc! {"
            Shows an action or condition is on the verge of occurring, or an excessive/extreme degree.

            Usage: Attach んばかり to the irrealis form (未然形) of verbs.

            Irregularities: する becomes せんばかり
        "},
        rules: &[
            suffix_inflection("んばかり", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("かんばかり", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("がんばかり", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("さんばかり", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たんばかり", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("なんばかり", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ばんばかり", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("まんばかり", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("らんばかり", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("わんばかり", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぜんばかり", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("せんばかり", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為んばかり", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こんばかり", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来んばかり", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來んばかり", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "-んとする",
        description: indoc! {"
            1. Shows the speaker\'s will or intention.
            2. Shows an action or condition is on the verge of occurring.

            Usage: Attach んとする to the irrealis form (未然形) of verbs.

            Irregularities: する becomes せんとする
        "},
        rules: &[
            suffix_inflection(
                "んとする",
                "る",
                Conditions::vs.with_subconditions(),
                Conditions::v1.with_subconditions(),
            ),
            suffix_inflection(
                "かんとする",
                "く",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "がんとする",
                "ぐ",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "さんとする",
                "す",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "たんとする",
                "つ",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "なんとする",
                "ぬ",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ばんとする",
                "ぶ",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "まんとする",
                "む",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "らんとする",
                "る",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "わんとする",
                "う",
                Conditions::vs.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ぜんとする",
                "ずる",
                Conditions::vs.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "せんとする",
                "する",
                Conditions::vs.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為んとする",
                "為る",
                Conditions::vs.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "こんとする",
                "くる",
                Conditions::vs.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来んとする",
                "来る",
                Conditions::vs.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來んとする",
                "來る",
                Conditions::vs.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-む",
        description: indoc! {"
            Archaic.
            1. Shows an inference of a certain matter.
            2. Shows speaker's intention.

            Usage: Attach む to the irrealis form (未然形) of verbs.

            Irregularities: する becomes せむ
        "},
        rules: &[
            suffix_inflection("む", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("かむ", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("がむ", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("さむ", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たむ", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("なむ", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ばむ", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("まむ", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("らむ", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("わむ", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぜむ", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("せむ", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為む", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こむ", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来む", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來む", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "-ざる",
        description: indoc! {"
            Negative form of verbs.

            Usage: Attach ざる to the irrealis form (未然形) of verbs.

            Irregularities: する becomes せざる
        "},
        rules: &[
            suffix_inflection("ざる", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("かざる", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("がざる", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("さざる", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("たざる", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("なざる", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ばざる", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("まざる", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("らざる", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("わざる", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぜざる", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("せざる", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為ざる", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こざる", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来ざる", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來ざる", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "-ねば",
        description: indoc! {"
            1. Shows a hypothetical negation; if not...
            2. Shows a must. Used with or without ならぬ.

            Usage: Attach ねば to the irrealis form (未然形) of verbs.

            Irregularities: する becomes せねば
        "},
        rules: &[
            suffix_inflection("ねば", "る", Conditions::ba.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection("かねば", "く", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("がねば", "ぐ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("さねば", "す", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("たねば", "つ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("なねば", "ぬ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ばねば", "ぶ", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("まねば", "む", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("らねば", "る", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("わねば", "う", Conditions::ba.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection(
                "ぜねば",
                "ずる",
                Conditions::ba.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "せねば",
                "する",
                Conditions::ba.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為ねば",
                "為る",
                Conditions::ba.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "こねば",
                "くる",
                Conditions::ba.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来ねば",
                "来る",
                Conditions::ba.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來ねば",
                "來る",
                Conditions::ba.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-く",
        description: indoc! {"
            Adverbial form of i-adjectives.\n

        "},
        rules: &[suffix_inflection(
            "く",
            "い",
            Conditions::ku.with_subconditions(),
            Conditions::adj_i.with_subconditions(),
        )],
    },
    Transform {
        name: "causative",
        description: indoc! {"
            Describes the intention to make someone do something.

            Usage: Attach させる to the irrealis form (未然形) of ichidan verbs and くる.
                   Attach せる to the irrealis form (未然形) of godan verbs and する.

            It itself conjugates as an ichidan verb.
        "},
        rules: &[
            suffix_inflection("させる", "る", Conditions::v1.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection("かせる", "く", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("がせる", "ぐ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("させる", "す", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("たせる", "つ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("なせる", "ぬ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ばせる", "ぶ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ませる", "む", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("らせる", "る", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("わせる", "う", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection(
                "じさせる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "ぜさせる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "させる",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為せる",
                "為る",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "せさせる",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為させる",
                "為る",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "こさせる",
                "くる",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来させる",
                "来る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來させる",
                "來る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "short causative",
        description: indoc! {"
            Contraction of the causative form.
            Describes the intention to make someone do something.

            Usage: Attach す to the irrealis form (未然形) of godan verbs.
                   Attach さす to the dictionary form (終止形) of ichidan verbs.

            Irregularities: する becomes さす, くる becomes こさす.
            It itself conjugates as an godan verb.
        "},
        rules: &[
            suffix_inflection("さす", "る", Conditions::v5ss.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection("かす", "く", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("がす", "ぐ", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("さす", "す", Conditions::v5ss.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("たす", "つ", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("なす", "ぬ", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ばす", "ぶ", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ます", "む", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("らす", "る", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("わす", "う", Conditions::v5sp.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection(
                "じさす",
                "ずる",
                Conditions::v5ss.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "ぜさす",
                "ずる",
                Conditions::v5ss.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "さす",
                "する",
                Conditions::v5ss.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為す",
                "為る",
                Conditions::v5ss.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "こさす",
                "くる",
                Conditions::v5ss.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来さす",
                "来る",
                Conditions::v5ss.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來さす",
                "來る",
                Conditions::v5ss.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "imperative",
        description: indoc! {"
            1. To give orders.
            2. (As あれ) Represents the fact that it will never change no matter the circumstances.
            3. Express a feeling of hope.
        "},
        rules: &[
            suffix_inflection("ろ", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("よ", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("え", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("け", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("げ", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("せ", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("て", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ね", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("べ", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("め", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("れ", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("じろ", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("ぜよ", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("しろ", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("せよ", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為ろ", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為よ", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こい", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来い", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來い", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "continuative",
        description: indoc! {"
            Used to indicate actions that are (being) carried out.
            Refers to 連用形, the part of the verb after conjugating with -ます and dropping ます.
        "},
        rules: &[
            suffix_inflection("い", "いる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("え", "える", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("き", "きる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("ぎ", "ぎる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("け", "ける", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("げ", "げる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("じ", "じる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("せ", "せる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("ぜ", "ぜる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("ち", "ちる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("て", "てる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("で", "でる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("に", "にる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("ね", "ねる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("ひ", "ひる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("び", "びる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("へ", "へる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("べ", "べる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("み", "みる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("め", "める", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("り", "りる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("れ", "れる", 0, Conditions::v1d.with_subconditions()),
            suffix_inflection("い", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("き", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぎ", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("し", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ち", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("に", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("び", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("み", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("り", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("き", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("し", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("来", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來", "來る", 0, Conditions::vk.with_subconditions()),
        ],
    },
    Transform {
        name: "negative",
        description: indoc! {"
            1. Negative form of verbs.
            2. Expresses a feeling of solicitation to the other party.

            Usage: Attach ない to the irrealis form (未然形) of verbs, くない to the stem of i-adjectives. ない itself conjugates as i-adjective. ます becomes ません.
        "},
        rules: &[
            suffix_inflection(
                "くない",
                "い",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "ない",
                "る",
                Conditions::adj_i.with_subconditions(),
                Conditions::v1.with_subconditions(),
            ),
            suffix_inflection(
                "かない",
                "く",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "がない",
                "ぐ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "さない",
                "す",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "たない",
                "つ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "なない",
                "ぬ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "ばない",
                "ぶ",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "まない",
                "む",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "らない",
                "る",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "わない",
                "う",
                Conditions::adj_i.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "じない",
                "ずる",
                Conditions::adj_i.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "しない",
                "する",
                Conditions::adj_i.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為ない",
                "為る",
                Conditions::adj_i.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "こない",
                "くる",
                Conditions::adj_i.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来ない",
                "来る",
                Conditions::adj_i.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來ない",
                "來る",
                Conditions::adj_i.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "ません",
                "ます",
                Conditions::masen.with_subconditions(),
                Conditions::masu.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-さ",
        description: indoc! {"
            Nominalizing suffix of i-adjectives indicating nature, state, mind or degree.

            Usage: Attach さ to the stem of i-adjectives.
        "},
        rules: &[suffix_inflection("さ", "い", 0, Conditions::adj_i.with_subconditions())],
    },
    Transform {
        name: "passive",
        description: indoc! {"
            1. Indicates an action received from an action performer.
            2. Expresses respect for the subject of action performer.
        "},
        rules: &[
            suffix_inflection("かれる", "く", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("がれる", "ぐ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection(
                "される",
                "す",
                Conditions::v1.with_subconditions(),
                (Conditions::v5d | Conditions::v5sp).with_subconditions(),
            ),
            suffix_inflection("たれる", "つ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("なれる", "ぬ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("ばれる", "ぶ", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("まれる", "む", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("われる", "う", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("られる", "る", Conditions::v1.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection(
                "じされる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "ぜされる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "される",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為れる",
                "為る",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "こられる",
                "くる",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来られる",
                "来る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來られる",
                "來る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-た",
        description: indoc! {"
            1. Indicates a reality that has happened in the past.
            2. Indicates the completion of an action.
            3. Indicates the confirmation of a matter.
            4. Indicates the speaker\'s confidence that the action will definitely be fulfilled.
            5. Indicates the events that occur before the main clause are represented as relative past.
            6. Indicates a mild imperative/command.

            Usage: Attach た to the continuative form (連用形) of verbs after euphonic change form, かった to the stem of i-adjectives.
        "},
        rules: &[
            suffix_inflection(
                "かった",
                "い",
                Conditions::ta.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection("た", "る", Conditions::ta.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection("いた", "く", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("いだ", "ぐ", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("した", "す", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("った", "う", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("った", "つ", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("った", "る", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("んだ", "ぬ", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("んだ", "ぶ", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("んだ", "む", Conditions::ta.with_subconditions(), Conditions::v5.with_subconditions()),
            suffix_inflection("じた", "ずる", Conditions::ta.with_subconditions(), Conditions::vz.with_subconditions()),
            suffix_inflection("した", "する", Conditions::ta.with_subconditions(), Conditions::vs.with_subconditions()),
            suffix_inflection("為た", "為る", Conditions::ta.with_subconditions(), Conditions::vs.with_subconditions()),
            suffix_inflection("きた", "くる", Conditions::ta.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection("来た", "来る", Conditions::ta.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection("來た", "來る", Conditions::ta.with_subconditions(), Conditions::vk.with_subconditions()),
            suffix_inflection(
                "ました",
                "ます",
                Conditions::ta.with_subconditions(),
                Conditions::masu.with_subconditions(),
            ),
            suffix_inflection(
                "でした",
                "",
                Conditions::ta.with_subconditions(),
                Conditions::masen.with_subconditions(),
            ),
            suffix_inflection(
                "かった",
                "",
                Conditions::ta.with_subconditions(),
                (Conditions::masen | Conditions::n).with_subconditions(),
            ),
            // Irregular (iku verbs)
            suffix_inflection(
                "いった",
                "いく",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "行った",
                "行く",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "逝った",
                "逝く",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "往った",
                "往く",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            // Irregular (godan u special)
            suffix_inflection(
                "こうた",
                "こう",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "とうた",
                "とう",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "請うた",
                "請う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "乞うた",
                "乞う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "恋うた",
                "恋う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "問うた",
                "問う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "訪うた",
                "訪う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "宣うた",
                "宣う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "曰うた",
                "曰う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "給うた",
                "給う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "賜うた",
                "賜う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "揺蕩うた",
                "揺蕩う",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            // Irregular (fu verb te conjugations)
            suffix_inflection(
                "のたもうた",
                "のたまう",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "たもうた",
                "たまう",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
            suffix_inflection(
                "たゆとうた",
                "たゆたう",
                Conditions::ta.with_subconditions(),
                Conditions::v5.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-ます",
        description: indoc! {"
            Polite conjugation of verbs and adjectives.

            Usage: Attach ます to the continuative form (連用形) of verbs.
        "},
        rules: &[
            suffix_inflection("ます", "る", Conditions::masu.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection(
                "います",
                "う",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "きます",
                "く",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "ぎます",
                "ぐ",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "します",
                "す",
                Conditions::masu.with_subconditions(),
                (Conditions::v5d | Conditions::v5s).with_subconditions(),
            ),
            suffix_inflection(
                "ちます",
                "つ",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "にます",
                "ぬ",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "びます",
                "ぶ",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "みます",
                "む",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "ります",
                "る",
                Conditions::masu.with_subconditions(),
                Conditions::v5d.with_subconditions(),
            ),
            suffix_inflection(
                "じます",
                "ずる",
                Conditions::masu.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "します",
                "する",
                Conditions::masu.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為ます",
                "為る",
                Conditions::masu.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "きます",
                "くる",
                Conditions::masu.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来ます",
                "来る",
                Conditions::masu.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來ます",
                "來る",
                Conditions::masu.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "くあります",
                "い",
                Conditions::masu.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "potential",
        description: indoc! {"
            Indicates a state of being (naturally) capable of doing an action.

            Usage: Attach (ら)れる to the irrealis form (未然形) of ichidan verbs.
                   Attach る to the imperative form (命令形) of godan verbs.

            Irregularities: する becomes できる, くる becomes こ(ら)れる
        "},
        rules: &[
            suffix_inflection(
                "れる",
                "る",
                Conditions::v1.with_subconditions(),
                (Conditions::v1 | Conditions::v5d).with_subconditions(),
            ),
            suffix_inflection("える", "う", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection("ける", "く", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection("げる", "ぐ", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection("せる", "す", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection("てる", "つ", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection("ねる", "ぬ", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection("べる", "ぶ", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection("める", "む", Conditions::v1.with_subconditions(), Conditions::v5d.with_subconditions()),
            suffix_inflection(
                "できる",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "出来る",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "これる",
                "くる",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来れる",
                "来る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來れる",
                "來る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "potential or passive",
        description: indoc! {"
            1. Indicates an action received from an action performer.
            2. Expresses respect for the subject of action performer.
            3. Indicates a state of being (naturally) capable of doing an action.

            Usage: Attach られる to the irrealis form (未然形) of ichidan verbs.

            Irregularities: する becomes せられる, くる becomes こられる
        "},
        rules: &[
            suffix_inflection("られる", "る", Conditions::v1.with_subconditions(), Conditions::v1.with_subconditions()),
            suffix_inflection(
                "ざれる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "ぜられる",
                "ずる",
                Conditions::v1.with_subconditions(),
                Conditions::vz.with_subconditions(),
            ),
            suffix_inflection(
                "せられる",
                "する",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "為られる",
                "為る",
                Conditions::v1.with_subconditions(),
                Conditions::vs.with_subconditions(),
            ),
            suffix_inflection(
                "こられる",
                "くる",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "来られる",
                "来る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
            suffix_inflection(
                "來られる",
                "來る",
                Conditions::v1.with_subconditions(),
                Conditions::vk.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "volitional",
        description: indoc! {"
            1. Expresses speaker's will or intention.
            2. Expresses an invitation to the other party.
            3. (Used in …ようとする) Indicates being on the verge of initiating an action or transforming a state.
            4. Indicates an inference of a matter.

            Usage: Attach よう to the irrealis form (未然形) of ichidan verbs.
                   Attach う to the irrealis form (未然形) of godan verbs after -o euphonic change form.
                   Attach かろう to the stem of i-adjectives (4th meaning only).

        "},
        rules: &[
            suffix_inflection("よう", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("おう", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("こう", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ごう", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("そう", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("とう", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("のう", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぼう", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("もう", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ろう", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("じよう", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("しよう", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為よう", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こよう", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来よう", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來よう", "來る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("ましょう", "ます", 0, Conditions::masu.with_subconditions()),
            suffix_inflection("かろう", "い", 0, Conditions::adj_i.with_subconditions()),
        ],
    },
    Transform {
        name: "volitional slang",
        description: indoc! {"
            Contraction of volitional form + か

            1. Expresses speaker's will or intention.
            2. Expresses an invitation to the other party.

            Usage: Replace final う with っ of volitional form then add か.

            For example: 行こうか -> 行こっか.
        "},
        rules: &[
            suffix_inflection("よっか", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("おっか", "う", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("こっか", "く", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ごっか", "ぐ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("そっか", "す", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("とっか", "つ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("のっか", "ぬ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ぼっか", "ぶ", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("もっか", "む", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("ろっか", "る", 0, Conditions::v5.with_subconditions()),
            suffix_inflection("じよっか", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("しよっか", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為よっか", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こよっか", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来よっか", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來よっか", "來る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("ましょっか", "ます", 0, Conditions::masu.with_subconditions()),
        ],
    },
    Transform {
        name: "-まい",
        description: indoc! {"
            Negative volitional form of verbs.
            1. Expresses speaker's assumption that something is likely not true.
            2. Expresses speaker's will or intention not to do something.

            Usage: Attach まい to the dictionary form (終止形) of verbs.
                   Attach まい to the irrealis form (未然形) of ichidan verbs.

            Irregularities: する becomes しまい, くる becomes こまい
        "},
        rules: &[
            suffix_inflection("まい", "", 0, Conditions::v.with_subconditions()),
            suffix_inflection("まい", "る", 0, Conditions::v1.with_subconditions()),
            suffix_inflection("じまい", "ずる", 0, Conditions::vz.with_subconditions()),
            suffix_inflection("しまい", "する", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("為まい", "為る", 0, Conditions::vs.with_subconditions()),
            suffix_inflection("こまい", "くる", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("来まい", "来る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("來まい", "來る", 0, Conditions::vk.with_subconditions()),
            suffix_inflection("まい", "", 0, Conditions::masu.with_subconditions()),
        ],
    },
    Transform {
        name: "-おく",
        description: indoc! {"
            To do certain things in advance in preparation (or in anticipation) of latter needs.

            Usage: Attach おく to the て-form of verbs.
                   Attach でおく after ない negative form of verbs.

            Contracts to とく・どく in speech.
        "},
        rules: &[
            suffix_inflection("ておく", "て", Conditions::v5.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("でおく", "で", Conditions::v5.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("とく", "て", Conditions::v5.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("どく", "で", Conditions::v5.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection(
                "ないでおく",
                "ない",
                Conditions::v5.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "ないどく",
                "ない",
                Conditions::v5.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-いる",
        description: indoc! {r#"
            1. Indicates an action continues or progresses to a point in time.
            2. Indicates an action is completed and remains as is.
            3. Indicates a state or condition that can be taken to be the result of undergoing some change.

            Usage: Attach いる to the て-form of verbs. い can be dropped in speech.
                   Attach でいる after ない negative form of verbs.
                   (Slang) Attach おる to the て-form of verbs. Contracts to とる・でる in speech.
        "#},
        rules: &[
            suffix_inflection("ている", "て", Conditions::v1.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("ておる", "て", Conditions::v5.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("てる", "て", Conditions::v1p.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("でいる", "で", Conditions::v1.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("でおる", "で", Conditions::v5.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("でる", "で", Conditions::v1p.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection("とる", "て", Conditions::v5.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection(
                "ないでいる",
                "ない",
                Conditions::v1.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "-き",
        description: indoc! {"
            Attributive form (連体形) of i-adjectives. An archaic form that remains in modern Japanese.
        "},
        rules: &[suffix_inflection("き", "い", 0, Conditions::adj_i.with_subconditions())],
    },
    Transform {
        name: "-げ",
        description: indoc! {"
            Describes a person's appearance. Shows feelings of the person.

            Usage: Attach げ or 気 to the stem of i-adjectives
        "},
        rules: &[
            suffix_inflection("げ", "い", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("気", "い", 0, Conditions::adj_i.with_subconditions()),
        ],
    },
    Transform {
        name: "-がる",
        description: indoc! {"
            1. Shows subject’s feelings contrast with what is thought/known about them.
            2. Indicates subject's behavior (stands out).

            Usage: Attach がる to the stem of i-adjectives. It itself conjugates as a godan verb.
        "},
        rules: &[suffix_inflection(
            "がる",
            "い",
            Conditions::v5.with_subconditions(),
            Conditions::adj_i.with_subconditions(),
        )],
    },
    Transform {
        name: "-え",
        description: indoc! {"
            Slang. A sound change of i-adjectives.

            ai: やばい → やべぇ
            ui: さむい → さみぃ/さめぇ
            oi: すごい → すげぇ
        "},
        rules: &[
            suffix_inflection("ねえ", "ない", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("めえ", "むい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("みい", "むい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ちぇえ", "つい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ちい", "つい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("せえ", "すい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ええ", "いい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ええ", "わい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ええ", "よい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("いぇえ", "よい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("うぇえ", "わい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("けえ", "かい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("げえ", "がい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("げえ", "ごい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("せえ", "さい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("めえ", "まい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ぜえ", "ずい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("っぜえ", "ずい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("れえ", "らい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("れえ", "らい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ちぇえ", "ちゃい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("でえ", "どい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("れえ", "れい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("べえ", "ばい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("てえ", "たい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ねぇ", "ない", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("めぇ", "むい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("みぃ", "むい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ちぃ", "つい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("せぇ", "すい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("けぇ", "かい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("げぇ", "がい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("げぇ", "ごい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("せぇ", "さい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("めぇ", "まい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ぜぇ", "ずい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("っぜぇ", "ずい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("れぇ", "らい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("でぇ", "どい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("れぇ", "れい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("べぇ", "ばい", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("てぇ", "たい", 0, Conditions::adj_i.with_subconditions()),
        ],
    },
    Transform {
        name: "n-slang",
        description: indoc! {"
            Slang sound change of r-column syllables to n (when before an n-sound, usually の or な)
        "},
        rules: &[
            suffix_inflection("んなさい", "りなさい", 0, Conditions::nasai.with_subconditions()),
            suffix_inflection(
                "らんない",
                "られない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "んない",
                "らない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection("んなきゃ", "らなきゃ", 0, Conditions::ya.with_subconditions()),
            suffix_inflection("んなきゃ", "れなきゃ", 0, Conditions::ya.with_subconditions()),
        ],
    },
    Transform {
        name: "imperative negative slang",
        description: "",
        rules: &[suffix_inflection("んな", "る", 0, Conditions::v.with_subconditions())],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            Negative form of kansai-ben verbs
        "},
        rules: &[
            suffix_inflection("へん", "ない", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("ひん", "ない", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection("せえへん", "しない", 0, Conditions::adj_i.with_subconditions()),
            suffix_inflection(
                "へんかった",
                "なかった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ひんかった",
                "なかった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection("うてへん", "ってない", 0, Conditions::adj_i.with_subconditions()),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            -て form of kansai-ben verbs
        "},
        rules: &[
            suffix_inflection("うて", "って", Conditions::te.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection(
                "おうて",
                "あって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "こうて",
                "かって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ごうて",
                "がって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "そうて",
                "さって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ぞうて",
                "ざって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "とうて",
                "たって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "どうて",
                "だって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "のうて",
                "なって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ほうて",
                "はって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ぼうて",
                "ばって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "もうて",
                "まって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ろうて",
                "らって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ようて",
                "やって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ゆうて",
                "いって",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            -た form of kansai-ben terms
        "},
        rules: &[
            suffix_inflection("うた", "った", Conditions::ta.with_subconditions(), Conditions::ta.with_subconditions()),
            suffix_inflection(
                "おうた",
                "あった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "こうた",
                "かった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ごうた",
                "がった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "そうた",
                "さった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ぞうた",
                "ざった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "とうた",
                "たった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "どうた",
                "だった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "のうた",
                "なった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ほうた",
                "はった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ぼうた",
                "ばった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "もうた",
                "まった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ろうた",
                "らった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ようた",
                "やった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
            suffix_inflection(
                "ゆうた",
                "いった",
                Conditions::ta.with_subconditions(),
                Conditions::ta.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            -たら form of kansai-ben terms
        "},
        rules: &[
            suffix_inflection("うたら", "ったら", 0, 0),
            suffix_inflection("おうたら", "あったら", 0, 0),
            suffix_inflection("こうたら", "かったら", 0, 0),
            suffix_inflection("ごうたら", "がったら", 0, 0),
            suffix_inflection("そうたら", "さったら", 0, 0),
            suffix_inflection("ぞうたら", "ざったら", 0, 0),
            suffix_inflection("とうたら", "たったら", 0, 0),
            suffix_inflection("どうたら", "だったら", 0, 0),
            suffix_inflection("のうたら", "なったら", 0, 0),
            suffix_inflection("ほうたら", "はったら", 0, 0),
            suffix_inflection("ぼうたら", "ばったら", 0, 0),
            suffix_inflection("もうたら", "まったら", 0, 0),
            suffix_inflection("ろうたら", "らったら", 0, 0),
            suffix_inflection("ようたら", "やったら", 0, 0),
            suffix_inflection("ゆうたら", "いったら", 0, 0),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            -たり form of kansai-ben terms
        "},
        rules: &[
            suffix_inflection("うたり", "ったり", 0, 0),
            suffix_inflection("おうたり", "あったり", 0, 0),
            suffix_inflection("こうたり", "かったり", 0, 0),
            suffix_inflection("ごうたり", "がったり", 0, 0),
            suffix_inflection("そうたり", "さったり", 0, 0),
            suffix_inflection("ぞうたり", "ざったり", 0, 0),
            suffix_inflection("とうたり", "たったり", 0, 0),
            suffix_inflection("どうたり", "だったり", 0, 0),
            suffix_inflection("のうたり", "なったり", 0, 0),
            suffix_inflection("ほうたり", "はったり", 0, 0),
            suffix_inflection("ぼうたり", "ばったり", 0, 0),
            suffix_inflection("もうたり", "まったり", 0, 0),
            suffix_inflection("ろうたり", "らったり", 0, 0),
            suffix_inflection("ようたり", "やったり", 0, 0),
            suffix_inflection("ゆうたり", "いったり", 0, 0),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            -く stem of kansai-ben adjectives
        "},
        rules: &[
            suffix_inflection("う", "く", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("こう", "かく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("ごう", "がく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("そう", "さく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("とう", "たく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("のう", "なく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("ぼう", "ばく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("もう", "まく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("ろう", "らく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("よう", "よく", 0, Conditions::ku.with_subconditions()),
            suffix_inflection("しゅう", "しく", 0, Conditions::ku.with_subconditions()),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            -て form of kansai-ben adjectives
        "},
        rules: &[
            suffix_inflection("うて", "くて", Conditions::te.with_subconditions(), Conditions::te.with_subconditions()),
            suffix_inflection(
                "こうて",
                "かくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ごうて",
                "がくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "そうて",
                "さくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "とうて",
                "たくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "のうて",
                "なくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ぼうて",
                "ばくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "もうて",
                "まくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ろうて",
                "らくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "ようて",
                "よくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
            suffix_inflection(
                "しゅうて",
                "しくて",
                Conditions::te.with_subconditions(),
                Conditions::te.with_subconditions(),
            ),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {"
            Negative form of kansai-ben adjectives
        "},
        rules: &[
            suffix_inflection(
                "うない",
                "くない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "こうない",
                "かくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "ごうない",
                "がくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "そうない",
                "さくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "とうない",
                "たくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "のうない",
                "なくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "ぼうない",
                "ばくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "もうない",
                "まくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "ろうない",
                "らくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "ようない",
                "よくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
            suffix_inflection(
                "しゅうない",
                "しくない",
                Conditions::adj_i.with_subconditions(),
                Conditions::adj_i.with_subconditions(),
            ),
        ],
    },
];
