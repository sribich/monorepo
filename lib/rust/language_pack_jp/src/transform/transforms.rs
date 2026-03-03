use indoc::indoc;
use language_pack::transform::Transform;
use language_pack::transform::suffix_inflection;

const shimau_description: &str = indoc! {r#"
    1. Shows a sense of regret/surprise when you did have volition in doing something, but it turned out to be bad to do.
    2. Shows perfective/punctual achievement. This shows that an action has been completed.
    3. Shows unintentional action–"accidentally".
"#};

const passive_description: &str = indoc! {r#"
    1. Indicates an action received from an action performer.
    2. Expresses respect for the subject of action performer.
"#};

pub const JAPANESE_TRANSFORMS: &[Transform] = &[
    Transform {
        name: "-ば",
        description: indoc! {"
            1. Conditional form; shows that the previous stated condition's establishment is the condition for the latter stated condition to occur.
            2. Shows a trigger for a latter stated perception or judgment.

            Usage: Attach ば to the hypothetical form (仮定形) of verbs and i-adjectives.
        "},
        rules: &[
            suffix_inflection("ければ", "い", &["-ば"], &["adj-i"]),
            suffix_inflection("えば", "う", &["-ば"], &["v5"]),
            suffix_inflection("けば", "く", &["-ば"], &["v5"]),
            suffix_inflection("げば", "ぐ", &["-ば"], &["v5"]),
            suffix_inflection("せば", "す", &["-ば"], &["v5"]),
            suffix_inflection("てば", "つ", &["-ば"], &["v5"]),
            suffix_inflection("ねば", "ぬ", &["-ば"], &["v5"]),
            suffix_inflection("べば", "ぶ", &["-ば"], &["v5"]),
            suffix_inflection("めば", "む", &["-ば"], &["v5"]),
            suffix_inflection("れば", "る", &["-ば"], &["v1", "v5", "vk", "vs", "vz"]),
            suffix_inflection("れば", "", &["-ば"], &["-ます"]),
        ],
    },
    Transform {
        name: "-ゃ",
        description: indoc! {r#"
            Contraction of -ば.
        "#},
        rules: &[
            suffix_inflection("けりゃ", "ければ", &["-ゃ"], &["-ば"]),
            suffix_inflection("きゃ", "ければ", &["-ゃ"], &["-ば"]),
            suffix_inflection("や", "えば", &["-ゃ"], &["-ば"]),
            suffix_inflection("きゃ", "けば", &["-ゃ"], &["-ば"]),
            suffix_inflection("ぎゃ", "げば", &["-ゃ"], &["-ば"]),
            suffix_inflection("しゃ", "せば", &["-ゃ"], &["-ば"]),
            suffix_inflection("ちゃ", "てば", &["-ゃ"], &["-ば"]),
            suffix_inflection("にゃ", "ねば", &["-ゃ"], &["-ば"]),
            suffix_inflection("びゃ", "べば", &["-ゃ"], &["-ば"]),
            suffix_inflection("みゃ", "めば", &["-ゃ"], &["-ば"]),
            suffix_inflection("りゃ", "れば", &["-ゃ"], &["-ば"]),
        ],
    },
    Transform {
        name: "-ちゃ",
        description: indoc! {r#"
            Contraction of ～ては.

            1. Explains how something always happens under the condition that it marks.
            2. Expresses the repetition (of a series of) actions.
            3. Indicates a hypothetical situation in which the speaker gives a (negative) evaluation about the other party\'s intentions.
            4. Used in "Must Not" patterns like ～てはいけない.

            Usage: Attach は after the て-form of verbs, contract ては into ちゃ.
        "#},
        rules: &[
            suffix_inflection("ちゃ", "る", &["v5"], &["v1"]),
            suffix_inflection("いじゃ", "ぐ", &["v5"], &["v5"]),
            suffix_inflection("いちゃ", "く", &["v5"], &["v5"]),
            suffix_inflection("しちゃ", "す", &["v5"], &["v5"]),
            suffix_inflection("っちゃ", "う", &["v5"], &["v5"]),
            suffix_inflection("っちゃ", "く", &["v5"], &["v5"]),
            suffix_inflection("っちゃ", "つ", &["v5"], &["v5"]),
            suffix_inflection("っちゃ", "る", &["v5"], &["v5"]),
            suffix_inflection("んじゃ", "ぬ", &["v5"], &["v5"]),
            suffix_inflection("んじゃ", "ぶ", &["v5"], &["v5"]),
            suffix_inflection("んじゃ", "む", &["v5"], &["v5"]),
            suffix_inflection("じちゃ", "ずる", &["v5"], &["vz"]),
            suffix_inflection("しちゃ", "する", &["v5"], &["vs"]),
            suffix_inflection("為ちゃ", "為る", &["v5"], &["vs"]),
            suffix_inflection("きちゃ", "くる", &["v5"], &["vk"]),
            suffix_inflection("来ちゃ", "来る", &["v5"], &["vk"]),
            suffix_inflection("來ちゃ", "來る", &["v5"], &["vk"]),
        ],
    },
    Transform {
        name: "-ちゃう",
        description: indoc! {r#"
            Contraction of -しまう.

            {shimau_description}

            Usage: Attach しまう after the て-form of verbs, contract てしまう into ちゃう.
        "#},
        rules: &[
            suffix_inflection("ちゃう", "る", &["v5"], &["v1"]),
            suffix_inflection("いじゃう", "ぐ", &["v5"], &["v5"]),
            suffix_inflection("いちゃう", "く", &["v5"], &["v5"]),
            suffix_inflection("しちゃう", "す", &["v5"], &["v5"]),
            suffix_inflection("っちゃう", "う", &["v5"], &["v5"]),
            suffix_inflection("っちゃう", "く", &["v5"], &["v5"]),
            suffix_inflection("っちゃう", "つ", &["v5"], &["v5"]),
            suffix_inflection("っちゃう", "る", &["v5"], &["v5"]),
            suffix_inflection("んじゃう", "ぬ", &["v5"], &["v5"]),
            suffix_inflection("んじゃう", "ぶ", &["v5"], &["v5"]),
            suffix_inflection("んじゃう", "む", &["v5"], &["v5"]),
            suffix_inflection("じちゃう", "ずる", &["v5"], &["vz"]),
            suffix_inflection("しちゃう", "する", &["v5"], &["vs"]),
            suffix_inflection("為ちゃう", "為る", &["v5"], &["vs"]),
            suffix_inflection("きちゃう", "くる", &["v5"], &["vk"]),
            suffix_inflection("来ちゃう", "来る", &["v5"], &["vk"]),
            suffix_inflection("來ちゃう", "來る", &["v5"], &["vk"]),
        ],
    },
    Transform {
        name: "-ちまう",
        description: indoc! {r#"
            Contraction of -しまう.

            {shimau_description}

            Usage: Attach しまう after the て-form of verbs, contract てしまう into ちまう.
        "#},
        rules: &[
            suffix_inflection("ちまう", "る", &["v5"], &["v1"]),
            suffix_inflection("いじまう", "ぐ", &["v5"], &["v5"]),
            suffix_inflection("いちまう", "く", &["v5"], &["v5"]),
            suffix_inflection("しちまう", "す", &["v5"], &["v5"]),
            suffix_inflection("っちまう", "う", &["v5"], &["v5"]),
            suffix_inflection("っちまう", "く", &["v5"], &["v5"]),
            suffix_inflection("っちまう", "つ", &["v5"], &["v5"]),
            suffix_inflection("っちまう", "る", &["v5"], &["v5"]),
            suffix_inflection("んじまう", "ぬ", &["v5"], &["v5"]),
            suffix_inflection("んじまう", "ぶ", &["v5"], &["v5"]),
            suffix_inflection("んじまう", "む", &["v5"], &["v5"]),
            suffix_inflection("じちまう", "ずる", &["v5"], &["vz"]),
            suffix_inflection("しちまう", "する", &["v5"], &["vs"]),
            suffix_inflection("為ちまう", "為る", &["v5"], &["vs"]),
            suffix_inflection("きちまう", "くる", &["v5"], &["vk"]),
            suffix_inflection("来ちまう", "来る", &["v5"], &["vk"]),
            suffix_inflection("來ちまう", "來る", &["v5"], &["vk"]),
        ],
    },
    Transform {
        name: "-しまう",
        description: indoc! {r#"
            {shimau_description}

            Usage: Attach しまう after the て-form of verbs.
        "#},
        rules: &[
            suffix_inflection("てしまう", "て", &["v5"], &["-て"]),
            suffix_inflection("でしまう", "で", &["v5"], &["-て"]),
        ],
    },
    Transform {
        name: "-なさい",
        description: indoc! {r#"
            Polite imperative suffix.

            Usage: Attach なさい after the continuative form (連用形) of verbs.
        "#},
        rules: &[
            suffix_inflection("なさい", "る", &["-なさい"], &["v1"]),
            suffix_inflection("いなさい", "う", &["-なさい"], &["v5"]),
            suffix_inflection("きなさい", "く", &["-なさい"], &["v5"]),
            suffix_inflection("ぎなさい", "ぐ", &["-なさい"], &["v5"]),
            suffix_inflection("しなさい", "す", &["-なさい"], &["v5"]),
            suffix_inflection("ちなさい", "つ", &["-なさい"], &["v5"]),
            suffix_inflection("になさい", "ぬ", &["-なさい"], &["v5"]),
            suffix_inflection("びなさい", "ぶ", &["-なさい"], &["v5"]),
            suffix_inflection("みなさい", "む", &["-なさい"], &["v5"]),
            suffix_inflection("りなさい", "る", &["-なさい"], &["v5"]),
            suffix_inflection("じなさい", "ずる", &["-なさい"], &["vz"]),
            suffix_inflection("しなさい", "する", &["-なさい"], &["vs"]),
            suffix_inflection("為なさい", "為る", &["-なさい"], &["vs"]),
            suffix_inflection("きなさい", "くる", &["-なさい"], &["vk"]),
            suffix_inflection("来なさい", "来る", &["-なさい"], &["vk"]),
            suffix_inflection("來なさい", "來る", &["-なさい"], &["vk"]),
        ],
    },
    Transform {
        name: "-そう",
        description: indoc! {r#"
            Appearing that; looking like.

            Usage: Attach そう to the continuative form (連用形) of verbs, or to the stem of adjectives.
        "#},
        rules: &[
            suffix_inflection("そう", "い", &[], &["adj-i"]),
            suffix_inflection("そう", "る", &[], &["v1"]),
            suffix_inflection("いそう", "う", &[], &["v5"]),
            suffix_inflection("きそう", "く", &[], &["v5"]),
            suffix_inflection("ぎそう", "ぐ", &[], &["v5"]),
            suffix_inflection("しそう", "す", &[], &["v5"]),
            suffix_inflection("ちそう", "つ", &[], &["v5"]),
            suffix_inflection("にそう", "ぬ", &[], &["v5"]),
            suffix_inflection("びそう", "ぶ", &[], &["v5"]),
            suffix_inflection("みそう", "む", &[], &["v5"]),
            suffix_inflection("りそう", "る", &[], &["v5"]),
            suffix_inflection("じそう", "ずる", &[], &["vz"]),
            suffix_inflection("しそう", "する", &[], &["vs"]),
            suffix_inflection("為そう", "為る", &[], &["vs"]),
            suffix_inflection("きそう", "くる", &[], &["vk"]),
            suffix_inflection("来そう", "来る", &[], &["vk"]),
            suffix_inflection("來そう", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "-すぎる",
        description: indoc! {r#"
            Shows something "is too..." or someone is doing something "too much".

            Usage: Attach すぎる to the continuative form (連用形) of verbs, or to the stem of adjectives.'
        "#},
        rules: &[
            suffix_inflection("すぎる", "い", &["v1"], &["adj-i"]),
            suffix_inflection("すぎる", "る", &["v1"], &["v1"]),
            suffix_inflection("いすぎる", "う", &["v1"], &["v5"]),
            suffix_inflection("きすぎる", "く", &["v1"], &["v5"]),
            suffix_inflection("ぎすぎる", "ぐ", &["v1"], &["v5"]),
            suffix_inflection("しすぎる", "す", &["v1"], &["v5"]),
            suffix_inflection("ちすぎる", "つ", &["v1"], &["v5"]),
            suffix_inflection("にすぎる", "ぬ", &["v1"], &["v5"]),
            suffix_inflection("びすぎる", "ぶ", &["v1"], &["v5"]),
            suffix_inflection("みすぎる", "む", &["v1"], &["v5"]),
            suffix_inflection("りすぎる", "る", &["v1"], &["v5"]),
            suffix_inflection("じすぎる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("しすぎる", "する", &["v1"], &["vs"]),
            suffix_inflection("為すぎる", "為る", &["v1"], &["vs"]),
            suffix_inflection("きすぎる", "くる", &["v1"], &["vk"]),
            suffix_inflection("来すぎる", "来る", &["v1"], &["vk"]),
            suffix_inflection("來すぎる", "來る", &["v1"], &["vk"]),
        ],
    },
    Transform {
        name: "-過ぎる",
        description: indoc! {r#"
            Shows something "is too..." or someone is doing something "too much".

            Usage: Attach 過ぎる to the continuative form (連用形) of verbs, or to the stem of adjectives.
        "#},
        rules: &[
            suffix_inflection("過ぎる", "い", &["v1"], &["adj-i"]),
            suffix_inflection("過ぎる", "る", &["v1"], &["v1"]),
            suffix_inflection("い過ぎる", "う", &["v1"], &["v5"]),
            suffix_inflection("き過ぎる", "く", &["v1"], &["v5"]),
            suffix_inflection("ぎ過ぎる", "ぐ", &["v1"], &["v5"]),
            suffix_inflection("し過ぎる", "す", &["v1"], &["v5"]),
            suffix_inflection("ち過ぎる", "つ", &["v1"], &["v5"]),
            suffix_inflection("に過ぎる", "ぬ", &["v1"], &["v5"]),
            suffix_inflection("び過ぎる", "ぶ", &["v1"], &["v5"]),
            suffix_inflection("み過ぎる", "む", &["v1"], &["v5"]),
            suffix_inflection("り過ぎる", "る", &["v1"], &["v5"]),
            suffix_inflection("じ過ぎる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("し過ぎる", "する", &["v1"], &["vs"]),
            suffix_inflection("為過ぎる", "為る", &["v1"], &["vs"]),
            suffix_inflection("き過ぎる", "くる", &["v1"], &["vk"]),
            suffix_inflection("来過ぎる", "来る", &["v1"], &["vk"]),
            suffix_inflection("來過ぎる", "來る", &["v1"], &["vk"]),
        ],
    },
    Transform {
        name: "-たい",
        description: indoc! {r#"
            1. Expresses the feeling of desire or hope.
            2. Used in ...たいと思います, an indirect way of saying what the speaker intends to do.

            Usage: Attach たい to the continuative form (連用形) of verbs. たい itself conjugates as i-adjective.
        "#},
        rules: &[
            suffix_inflection("たい", "る", &["adj-i"], &["v1"]),
            suffix_inflection("いたい", "う", &["adj-i"], &["v5"]),
            suffix_inflection("きたい", "く", &["adj-i"], &["v5"]),
            suffix_inflection("ぎたい", "ぐ", &["adj-i"], &["v5"]),
            suffix_inflection("したい", "す", &["adj-i"], &["v5"]),
            suffix_inflection("ちたい", "つ", &["adj-i"], &["v5"]),
            suffix_inflection("にたい", "ぬ", &["adj-i"], &["v5"]),
            suffix_inflection("びたい", "ぶ", &["adj-i"], &["v5"]),
            suffix_inflection("みたい", "む", &["adj-i"], &["v5"]),
            suffix_inflection("りたい", "る", &["adj-i"], &["v5"]),
            suffix_inflection("じたい", "ずる", &["adj-i"], &["vz"]),
            suffix_inflection("したい", "する", &["adj-i"], &["vs"]),
            suffix_inflection("為たい", "為る", &["adj-i"], &["vs"]),
            suffix_inflection("きたい", "くる", &["adj-i"], &["vk"]),
            suffix_inflection("来たい", "来る", &["adj-i"], &["vk"]),
            suffix_inflection("來たい", "來る", &["adj-i"], &["vk"]),
        ],
    },
    Transform {
        name: "-たら",
        description: indoc! {r#"
            1. Denotes the latter stated event is a continuation of the previous stated event.
            2. Assumes that a matter has been completed or concluded.

            Usage: Attach たら to the continuative form (連用形) of verbs after euphonic change form, かったら to the stem of i-adjectives.
        "#},
        rules: &[
            suffix_inflection("かったら", "い", &[], &["adj-i"]),
            suffix_inflection("たら", "る", &[], &["v1"]),
            suffix_inflection("いたら", "く", &[], &["v5"]),
            suffix_inflection("いだら", "ぐ", &[], &["v5"]),
            suffix_inflection("したら", "す", &[], &["v5"]),
            suffix_inflection("ったら", "う", &[], &["v5"]),
            suffix_inflection("ったら", "つ", &[], &["v5"]),
            suffix_inflection("ったら", "る", &[], &["v5"]),
            suffix_inflection("んだら", "ぬ", &[], &["v5"]),
            suffix_inflection("んだら", "ぶ", &[], &["v5"]),
            suffix_inflection("んだら", "む", &[], &["v5"]),
            suffix_inflection("じたら", "ずる", &[], &["vz"]),
            suffix_inflection("したら", "する", &[], &["vs"]),
            suffix_inflection("為たら", "為る", &[], &["vs"]),
            suffix_inflection("きたら", "くる", &[], &["vk"]),
            suffix_inflection("来たら", "来る", &[], &["vk"]),
            suffix_inflection("來たら", "來る", &[], &["vk"]),
            // ...irregularVerbSuffixInflections("たら", &[], &["v5"]),
            suffix_inflection("ましたら", "ます", &[], &["-ます"]),
        ],
    },
    Transform {
        name: "-たり",
        description: indoc! {r#"
            1. Shows two actions occurring back and forth (when used with two verbs).
            2. Shows examples of actions and states (when used with multiple verbs and adjectives).

            Usage: Attach たり to the continuative form (連用形) of verbs after euphonic change form, かったり to the stem of i-adjectives
        "#},
        rules: &[
            suffix_inflection("かったり", "い", &[], &["adj-i"]),
            suffix_inflection("たり", "る", &[], &["v1"]),
            suffix_inflection("いたり", "く", &[], &["v5"]),
            suffix_inflection("いだり", "ぐ", &[], &["v5"]),
            suffix_inflection("したり", "す", &[], &["v5"]),
            suffix_inflection("ったり", "う", &[], &["v5"]),
            suffix_inflection("ったり", "つ", &[], &["v5"]),
            suffix_inflection("ったり", "る", &[], &["v5"]),
            suffix_inflection("んだり", "ぬ", &[], &["v5"]),
            suffix_inflection("んだり", "ぶ", &[], &["v5"]),
            suffix_inflection("んだり", "む", &[], &["v5"]),
            suffix_inflection("じたり", "ずる", &[], &["vz"]),
            suffix_inflection("したり", "する", &[], &["vs"]),
            suffix_inflection("為たり", "為る", &[], &["vs"]),
            suffix_inflection("きたり", "くる", &[], &["vk"]),
            suffix_inflection("来たり", "来る", &[], &["vk"]),
            suffix_inflection("來たり", "來る", &[], &["vk"]),
            // ...irregularVerbSuffixInflections("たり", &[], &["v5"]),
        ],
    },
    Transform {
        name: "-て",
        description: indoc! {r#"
            て-form.

            It has a myriad of meanings. Primarily, it is a conjunctive particle that connects two clauses together.

            Usage: Attach て to the continuative form (連用形) of verbs after euphonic change form, くて to the stem of i-adjectives.
        "#},
        rules: &[
            suffix_inflection("くて", "い", &["-て"], &["adj-i"]),
            suffix_inflection("て", "る", &["-て"], &["v1"]),
            suffix_inflection("いて", "く", &["-て"], &["v5"]),
            suffix_inflection("いで", "ぐ", &["-て"], &["v5"]),
            suffix_inflection("して", "す", &["-て"], &["v5"]),
            suffix_inflection("って", "う", &["-て"], &["v5"]),
            suffix_inflection("って", "つ", &["-て"], &["v5"]),
            suffix_inflection("って", "る", &["-て"], &["v5"]),
            suffix_inflection("んで", "ぬ", &["-て"], &["v5"]),
            suffix_inflection("んで", "ぶ", &["-て"], &["v5"]),
            suffix_inflection("んで", "む", &["-て"], &["v5"]),
            suffix_inflection("じて", "ずる", &["-て"], &["vz"]),
            suffix_inflection("して", "する", &["-て"], &["vs"]),
            suffix_inflection("為て", "為る", &["-て"], &["vs"]),
            suffix_inflection("きて", "くる", &["-て"], &["vk"]),
            suffix_inflection("来て", "来る", &["-て"], &["vk"]),
            suffix_inflection("來て", "來る", &["-て"], &["vk"]),
            // ...irregularVerbSuffixInflections("て", &["-て"], &["v5"]),
            suffix_inflection("まして", "ます", &[], &["-ます"]),
        ],
    },
    Transform {
        name: "-ず",
        description: indoc! {r#"
            1. Negative form of verbs.
            2. Continuative form (連用形) of the particle ぬ (nu).

            Usage: Attach ず to the irrealis form (未然形) of verbs.
        "#},
        rules: &[
            suffix_inflection("ず", "る", &[], &["v1"]),
            suffix_inflection("かず", "く", &[], &["v5"]),
            suffix_inflection("がず", "ぐ", &[], &["v5"]),
            suffix_inflection("さず", "す", &[], &["v5"]),
            suffix_inflection("たず", "つ", &[], &["v5"]),
            suffix_inflection("なず", "ぬ", &[], &["v5"]),
            suffix_inflection("ばず", "ぶ", &[], &["v5"]),
            suffix_inflection("まず", "む", &[], &["v5"]),
            suffix_inflection("らず", "る", &[], &["v5"]),
            suffix_inflection("わず", "う", &[], &["v5"]),
            suffix_inflection("ぜず", "ずる", &[], &["vz"]),
            suffix_inflection("せず", "する", &[], &["vs"]),
            suffix_inflection("為ず", "為る", &[], &["vs"]),
            suffix_inflection("こず", "くる", &[], &["vk"]),
            suffix_inflection("来ず", "来る", &[], &["vk"]),
            suffix_inflection("來ず", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "-ぬ",
        description: indoc! {r#"
            Negative form of verbs.

            する becomes せぬ.

            Usage: Attach ぬ to the irrealis form (未然形) of verbs.
        "#},
        rules: &[
            suffix_inflection("ぬ", "る", &[], &["v1"]),
            suffix_inflection("かぬ", "く", &[], &["v5"]),
            suffix_inflection("がぬ", "ぐ", &[], &["v5"]),
            suffix_inflection("さぬ", "す", &[], &["v5"]),
            suffix_inflection("たぬ", "つ", &[], &["v5"]),
            suffix_inflection("なぬ", "ぬ", &[], &["v5"]),
            suffix_inflection("ばぬ", "ぶ", &[], &["v5"]),
            suffix_inflection("まぬ", "む", &[], &["v5"]),
            suffix_inflection("らぬ", "る", &[], &["v5"]),
            suffix_inflection("わぬ", "う", &[], &["v5"]),
            suffix_inflection("ぜぬ", "ずる", &[], &["vz"]),
            suffix_inflection("せぬ", "する", &[], &["vs"]),
            suffix_inflection("為ぬ", "為る", &[], &["vs"]),
            suffix_inflection("こぬ", "くる", &[], &["vk"]),
            suffix_inflection("来ぬ", "来る", &[], &["vk"]),
            suffix_inflection("來ぬ", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "-ん",
        description: indoc! {r#"
            Negative form of verbs; a sound change of ぬ.

            する becomes せん.

            Usage: Attach ん to the irrealis form (未然形) of verbs.
        "#},
        rules: &[
            suffix_inflection("ん", "る", &["-ん"], &["v1"]),
            suffix_inflection("かん", "く", &["-ん"], &["v5"]),
            suffix_inflection("がん", "ぐ", &["-ん"], &["v5"]),
            suffix_inflection("さん", "す", &["-ん"], &["v5"]),
            suffix_inflection("たん", "つ", &["-ん"], &["v5"]),
            suffix_inflection("なん", "ぬ", &["-ん"], &["v5"]),
            suffix_inflection("ばん", "ぶ", &["-ん"], &["v5"]),
            suffix_inflection("まん", "む", &["-ん"], &["v5"]),
            suffix_inflection("らん", "る", &["-ん"], &["v5"]),
            suffix_inflection("わん", "う", &["-ん"], &["v5"]),
            suffix_inflection("ぜん", "ずる", &["-ん"], &["vz"]),
            suffix_inflection("せん", "する", &["-ん"], &["vs"]),
            suffix_inflection("為ん", "為る", &["-ん"], &["vs"]),
            suffix_inflection("こん", "くる", &["-ん"], &["vk"]),
            suffix_inflection("来ん", "来る", &["-ん"], &["vk"]),
            suffix_inflection("來ん", "來る", &["-ん"], &["vk"]),
        ],
    },
    Transform {
        name: "-んばかり",
        description: indoc! {r#"
            Shows an action or condition is on the verge of occurring, or an excessive/extreme degree.

            する becomes せんばかり

            Usage: Attach んばかり to the irrealis form (未然形) of verbs.
        "#},
        rules: &[
            suffix_inflection("んばかり", "る", &[], &["v1"]),
            suffix_inflection("かんばかり", "く", &[], &["v5"]),
            suffix_inflection("がんばかり", "ぐ", &[], &["v5"]),
            suffix_inflection("さんばかり", "す", &[], &["v5"]),
            suffix_inflection("たんばかり", "つ", &[], &["v5"]),
            suffix_inflection("なんばかり", "ぬ", &[], &["v5"]),
            suffix_inflection("ばんばかり", "ぶ", &[], &["v5"]),
            suffix_inflection("まんばかり", "む", &[], &["v5"]),
            suffix_inflection("らんばかり", "る", &[], &["v5"]),
            suffix_inflection("わんばかり", "う", &[], &["v5"]),
            suffix_inflection("ぜんばかり", "ずる", &[], &["vz"]),
            suffix_inflection("せんばかり", "する", &[], &["vs"]),
            suffix_inflection("為んばかり", "為る", &[], &["vs"]),
            suffix_inflection("こんばかり", "くる", &[], &["vk"]),
            suffix_inflection("来んばかり", "来る", &[], &["vk"]),
            suffix_inflection("來んばかり", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "-んとする",
        description: indoc! {r#"
            '1. Shows the speaker\'s will or intention.\n' +
            '2. Shows an action or condition is on the verge of occurring.\n' +
            'Usage: Attach んとする to the irrealis form (未然形) of verbs.\n' +
            'する becomes せんとする',

        "#},
        rules: &[
            suffix_inflection("んとする", "る", &["vs"], &["v1"]),
            suffix_inflection("かんとする", "く", &["vs"], &["v5"]),
            suffix_inflection("がんとする", "ぐ", &["vs"], &["v5"]),
            suffix_inflection("さんとする", "す", &["vs"], &["v5"]),
            suffix_inflection("たんとする", "つ", &["vs"], &["v5"]),
            suffix_inflection("なんとする", "ぬ", &["vs"], &["v5"]),
            suffix_inflection("ばんとする", "ぶ", &["vs"], &["v5"]),
            suffix_inflection("まんとする", "む", &["vs"], &["v5"]),
            suffix_inflection("らんとする", "る", &["vs"], &["v5"]),
            suffix_inflection("わんとする", "う", &["vs"], &["v5"]),
            suffix_inflection("ぜんとする", "ずる", &["vs"], &["vz"]),
            suffix_inflection("せんとする", "する", &["vs"], &["vs"]),
            suffix_inflection("為んとする", "為る", &["vs"], &["vs"]),
            suffix_inflection("こんとする", "くる", &["vs"], &["vk"]),
            suffix_inflection("来んとする", "来る", &["vs"], &["vk"]),
            suffix_inflection("來んとする", "來る", &["vs"], &["vk"]),
        ],
    },
    Transform {
        name: "-む",
        description: indoc! {r#"
            'Archaic.\n' +
            '1. Shows an inference of a certain matter.\n' +
            '2. Shows speaker\'s intention.\n' +
            'Usage: Attach む to the irrealis form (未然形) of verbs.\n' +
            'する becomes せむ',

        "#},
        rules: &[
            suffix_inflection("む", "る", &[], &["v1"]),
            suffix_inflection("かむ", "く", &[], &["v5"]),
            suffix_inflection("がむ", "ぐ", &[], &["v5"]),
            suffix_inflection("さむ", "す", &[], &["v5"]),
            suffix_inflection("たむ", "つ", &[], &["v5"]),
            suffix_inflection("なむ", "ぬ", &[], &["v5"]),
            suffix_inflection("ばむ", "ぶ", &[], &["v5"]),
            suffix_inflection("まむ", "む", &[], &["v5"]),
            suffix_inflection("らむ", "る", &[], &["v5"]),
            suffix_inflection("わむ", "う", &[], &["v5"]),
            suffix_inflection("ぜむ", "ずる", &[], &["vz"]),
            suffix_inflection("せむ", "する", &[], &["vs"]),
            suffix_inflection("為む", "為る", &[], &["vs"]),
            suffix_inflection("こむ", "くる", &[], &["vk"]),
            suffix_inflection("来む", "来る", &[], &["vk"]),
            suffix_inflection("來む", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "-ざる",
        description: indoc! {r#"
            'Negative form of verbs.\n' +
            'Usage: Attach ざる to the irrealis form (未然形) of verbs.\n' +
            'する becomes せざる',

        "#},
        rules: &[
            suffix_inflection("ざる", "る", &[], &["v1"]),
            suffix_inflection("かざる", "く", &[], &["v5"]),
            suffix_inflection("がざる", "ぐ", &[], &["v5"]),
            suffix_inflection("さざる", "す", &[], &["v5"]),
            suffix_inflection("たざる", "つ", &[], &["v5"]),
            suffix_inflection("なざる", "ぬ", &[], &["v5"]),
            suffix_inflection("ばざる", "ぶ", &[], &["v5"]),
            suffix_inflection("まざる", "む", &[], &["v5"]),
            suffix_inflection("らざる", "る", &[], &["v5"]),
            suffix_inflection("わざる", "う", &[], &["v5"]),
            suffix_inflection("ぜざる", "ずる", &[], &["vz"]),
            suffix_inflection("せざる", "する", &[], &["vs"]),
            suffix_inflection("為ざる", "為る", &[], &["vs"]),
            suffix_inflection("こざる", "くる", &[], &["vk"]),
            suffix_inflection("来ざる", "来る", &[], &["vk"]),
            suffix_inflection("來ざる", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "-ねば",
        description: indoc! {r#"
            '1. Shows a hypothetical negation; if not ...\n' +
            '2. Shows a must. Used with or without ならぬ.\n' +
            'Usage: Attach ねば to the irrealis form (未然形) of verbs.\n' +
            'する becomes せねば',

        "#},
        rules: &[
            suffix_inflection("ねば", "る", &["-ば"], &["v1"]),
            suffix_inflection("かねば", "く", &["-ば"], &["v5"]),
            suffix_inflection("がねば", "ぐ", &["-ば"], &["v5"]),
            suffix_inflection("さねば", "す", &["-ば"], &["v5"]),
            suffix_inflection("たねば", "つ", &["-ば"], &["v5"]),
            suffix_inflection("なねば", "ぬ", &["-ば"], &["v5"]),
            suffix_inflection("ばねば", "ぶ", &["-ば"], &["v5"]),
            suffix_inflection("まねば", "む", &["-ば"], &["v5"]),
            suffix_inflection("らねば", "る", &["-ば"], &["v5"]),
            suffix_inflection("わねば", "う", &["-ば"], &["v5"]),
            suffix_inflection("ぜねば", "ずる", &["-ば"], &["vz"]),
            suffix_inflection("せねば", "する", &["-ば"], &["vs"]),
            suffix_inflection("為ねば", "為る", &["-ば"], &["vs"]),
            suffix_inflection("こねば", "くる", &["-ば"], &["vk"]),
            suffix_inflection("来ねば", "来る", &["-ば"], &["vk"]),
            suffix_inflection("來ねば", "來る", &["-ば"], &["vk"]),
        ],
    },
    Transform {
        name: "-く",
        description: indoc! {r#"
            'Adverbial form of i-adjectives.\n',

        "#},
        rules: &[suffix_inflection("く", "い", &["-く"], &["adj-i"])],
    },
    Transform {
        name: "causative",
        description: indoc! {r#"
            'Describes the intention to make someone do something.\n' +
            'Usage: Attach させる to the irrealis form (未然形) of ichidan verbs and くる.\n' +
            'Attach せる to the irrealis form (未然形) of godan verbs and する.\n' +
            'It itself conjugates as an ichidan verb.',

        "#},
        rules: &[
            suffix_inflection("させる", "る", &["v1"], &["v1"]),
            suffix_inflection("かせる", "く", &["v1"], &["v5"]),
            suffix_inflection("がせる", "ぐ", &["v1"], &["v5"]),
            suffix_inflection("させる", "す", &["v1"], &["v5"]),
            suffix_inflection("たせる", "つ", &["v1"], &["v5"]),
            suffix_inflection("なせる", "ぬ", &["v1"], &["v5"]),
            suffix_inflection("ばせる", "ぶ", &["v1"], &["v5"]),
            suffix_inflection("ませる", "む", &["v1"], &["v5"]),
            suffix_inflection("らせる", "る", &["v1"], &["v5"]),
            suffix_inflection("わせる", "う", &["v1"], &["v5"]),
            suffix_inflection("じさせる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("ぜさせる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("させる", "する", &["v1"], &["vs"]),
            suffix_inflection("為せる", "為る", &["v1"], &["vs"]),
            suffix_inflection("せさせる", "する", &["v1"], &["vs"]),
            suffix_inflection("為させる", "為る", &["v1"], &["vs"]),
            suffix_inflection("こさせる", "くる", &["v1"], &["vk"]),
            suffix_inflection("来させる", "来る", &["v1"], &["vk"]),
            suffix_inflection("來させる", "來る", &["v1"], &["vk"]),
        ],
    },
    Transform {
        name: "short causative",
        description: indoc! {r#"
            'Contraction of the causative form.\n' +
            'Describes the intention to make someone do something.\n' +
            'Usage: Attach す to the irrealis form (未然形) of godan verbs.\n' +
            'Attach さす to the dictionary form (終止形) of ichidan verbs.\n' +
            'する becomes さす, くる becomes こさす.\n' +
            'It itself conjugates as an godan verb.',

        "#},
        rules: &[
            suffix_inflection("さす", "る", &["v5ss"], &["v1"]),
            suffix_inflection("かす", "く", &["v5sp"], &["v5"]),
            suffix_inflection("がす", "ぐ", &["v5sp"], &["v5"]),
            suffix_inflection("さす", "す", &["v5ss"], &["v5"]),
            suffix_inflection("たす", "つ", &["v5sp"], &["v5"]),
            suffix_inflection("なす", "ぬ", &["v5sp"], &["v5"]),
            suffix_inflection("ばす", "ぶ", &["v5sp"], &["v5"]),
            suffix_inflection("ます", "む", &["v5sp"], &["v5"]),
            suffix_inflection("らす", "る", &["v5sp"], &["v5"]),
            suffix_inflection("わす", "う", &["v5sp"], &["v5"]),
            suffix_inflection("じさす", "ずる", &["v5ss"], &["vz"]),
            suffix_inflection("ぜさす", "ずる", &["v5ss"], &["vz"]),
            suffix_inflection("さす", "する", &["v5ss"], &["vs"]),
            suffix_inflection("為す", "為る", &["v5ss"], &["vs"]),
            suffix_inflection("こさす", "くる", &["v5ss"], &["vk"]),
            suffix_inflection("来さす", "来る", &["v5ss"], &["vk"]),
            suffix_inflection("來さす", "來る", &["v5ss"], &["vk"]),
        ],
    },
    Transform {
        name: "imperative",
        description: indoc! {r#"
            '1. To give orders.\n' +
            '2. (As あれ) Represents the fact that it will never change no matter the circumstances.\n' +
            '3. Express a feeling of hope.',

        "#},
        rules: &[
            suffix_inflection("ろ", "る", &[], &["v1"]),
            suffix_inflection("よ", "る", &[], &["v1"]),
            suffix_inflection("え", "う", &[], &["v5"]),
            suffix_inflection("け", "く", &[], &["v5"]),
            suffix_inflection("げ", "ぐ", &[], &["v5"]),
            suffix_inflection("せ", "す", &[], &["v5"]),
            suffix_inflection("て", "つ", &[], &["v5"]),
            suffix_inflection("ね", "ぬ", &[], &["v5"]),
            suffix_inflection("べ", "ぶ", &[], &["v5"]),
            suffix_inflection("め", "む", &[], &["v5"]),
            suffix_inflection("れ", "る", &[], &["v5"]),
            suffix_inflection("じろ", "ずる", &[], &["vz"]),
            suffix_inflection("ぜよ", "ずる", &[], &["vz"]),
            suffix_inflection("しろ", "する", &[], &["vs"]),
            suffix_inflection("せよ", "する", &[], &["vs"]),
            suffix_inflection("為ろ", "為る", &[], &["vs"]),
            suffix_inflection("為よ", "為る", &[], &["vs"]),
            suffix_inflection("こい", "くる", &[], &["vk"]),
            suffix_inflection("来い", "来る", &[], &["vk"]),
            suffix_inflection("來い", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "continuative",
        description: indoc! {r#"
            'Used to indicate actions that are (being) carried out.\n' +
            'Refers to 連用形, the part of the verb after conjugating with -ます and dropping ます.',

        "#},
        rules: &[
            suffix_inflection("い", "いる", &[], &["v1d"]),
            suffix_inflection("え", "える", &[], &["v1d"]),
            suffix_inflection("き", "きる", &[], &["v1d"]),
            suffix_inflection("ぎ", "ぎる", &[], &["v1d"]),
            suffix_inflection("け", "ける", &[], &["v1d"]),
            suffix_inflection("げ", "げる", &[], &["v1d"]),
            suffix_inflection("じ", "じる", &[], &["v1d"]),
            suffix_inflection("せ", "せる", &[], &["v1d"]),
            suffix_inflection("ぜ", "ぜる", &[], &["v1d"]),
            suffix_inflection("ち", "ちる", &[], &["v1d"]),
            suffix_inflection("て", "てる", &[], &["v1d"]),
            suffix_inflection("で", "でる", &[], &["v1d"]),
            suffix_inflection("に", "にる", &[], &["v1d"]),
            suffix_inflection("ね", "ねる", &[], &["v1d"]),
            suffix_inflection("ひ", "ひる", &[], &["v1d"]),
            suffix_inflection("び", "びる", &[], &["v1d"]),
            suffix_inflection("へ", "へる", &[], &["v1d"]),
            suffix_inflection("べ", "べる", &[], &["v1d"]),
            suffix_inflection("み", "みる", &[], &["v1d"]),
            suffix_inflection("め", "める", &[], &["v1d"]),
            suffix_inflection("り", "りる", &[], &["v1d"]),
            suffix_inflection("れ", "れる", &[], &["v1d"]),
            suffix_inflection("い", "う", &[], &["v5"]),
            suffix_inflection("き", "く", &[], &["v5"]),
            suffix_inflection("ぎ", "ぐ", &[], &["v5"]),
            suffix_inflection("し", "す", &[], &["v5"]),
            suffix_inflection("ち", "つ", &[], &["v5"]),
            suffix_inflection("に", "ぬ", &[], &["v5"]),
            suffix_inflection("び", "ぶ", &[], &["v5"]),
            suffix_inflection("み", "む", &[], &["v5"]),
            suffix_inflection("り", "る", &[], &["v5"]),
            suffix_inflection("き", "くる", &[], &["vk"]),
            suffix_inflection("し", "する", &[], &["vs"]),
            suffix_inflection("来", "来る", &[], &["vk"]),
            suffix_inflection("來", "來る", &[], &["vk"]),
        ],
    },
    Transform {
        name: "negative",
        description: indoc! {r#"
            '1. Negative form of verbs.\n' +
            '2. Expresses a feeling of solicitation to the other party.\n' +
            'Usage: Attach ない to the irrealis form (未然形) of verbs, くない to the stem of i-adjectives. ない itself conjugates as i-adjective. ます becomes ません.',

        "#},
        rules: &[
            suffix_inflection("くない", "い", &["adj-i"], &["adj-i"]),
            suffix_inflection("ない", "る", &["adj-i"], &["v1"]),
            suffix_inflection("かない", "く", &["adj-i"], &["v5"]),
            suffix_inflection("がない", "ぐ", &["adj-i"], &["v5"]),
            suffix_inflection("さない", "す", &["adj-i"], &["v5"]),
            suffix_inflection("たない", "つ", &["adj-i"], &["v5"]),
            suffix_inflection("なない", "ぬ", &["adj-i"], &["v5"]),
            suffix_inflection("ばない", "ぶ", &["adj-i"], &["v5"]),
            suffix_inflection("まない", "む", &["adj-i"], &["v5"]),
            suffix_inflection("らない", "る", &["adj-i"], &["v5"]),
            suffix_inflection("わない", "う", &["adj-i"], &["v5"]),
            suffix_inflection("じない", "ずる", &["adj-i"], &["vz"]),
            suffix_inflection("しない", "する", &["adj-i"], &["vs"]),
            suffix_inflection("為ない", "為る", &["adj-i"], &["vs"]),
            suffix_inflection("こない", "くる", &["adj-i"], &["vk"]),
            suffix_inflection("来ない", "来る", &["adj-i"], &["vk"]),
            suffix_inflection("來ない", "來る", &["adj-i"], &["vk"]),
            suffix_inflection("ません", "ます", &["-ません"], &["-ます"]),
        ],
    },
    Transform {
        name: "-さ",
        description: indoc! {r#"
            'Nominalizing suffix of i-adjectives indicating nature, state, mind or degree.\n' +
            'Usage: Attach さ to the stem of i-adjectives.',

        "#},
        rules: &[suffix_inflection("さ", "い", &[], &["adj-i"])],
    },
    Transform {
        name: "passive",
        description: indoc! {r#"
            passiveEnglishDescription +
            'Usage: Attach れる to the irrealis form (未然形) of godan verbs.',
        "#},
        rules: &[
            suffix_inflection("かれる", "く", &["v1"], &["v5"]),
            suffix_inflection("がれる", "ぐ", &["v1"], &["v5"]),
            suffix_inflection("される", "す", &["v1"], &["v5d", "v5sp"]),
            suffix_inflection("たれる", "つ", &["v1"], &["v5"]),
            suffix_inflection("なれる", "ぬ", &["v1"], &["v5"]),
            suffix_inflection("ばれる", "ぶ", &["v1"], &["v5"]),
            suffix_inflection("まれる", "む", &["v1"], &["v5"]),
            suffix_inflection("われる", "う", &["v1"], &["v5"]),
            suffix_inflection("られる", "る", &["v1"], &["v5"]),
            suffix_inflection("じされる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("ぜされる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("される", "する", &["v1"], &["vs"]),
            suffix_inflection("為れる", "為る", &["v1"], &["vs"]),
            suffix_inflection("こられる", "くる", &["v1"], &["vk"]),
            suffix_inflection("来られる", "来る", &["v1"], &["vk"]),
            suffix_inflection("來られる", "來る", &["v1"], &["vk"]),
        ],
    },
    Transform {
        name: "-た",
        description: indoc! {r#"
            '1. Indicates a reality that has happened in the past.\n' +
            '2. Indicates the completion of an action.\n' +
            '3. Indicates the confirmation of a matter.\n' +
            '4. Indicates the speaker\'s confidence that the action will definitely be fulfilled.\n' +
            '5. Indicates the events that occur before the main clause are represented as relative past.\n' +
            '6. Indicates a mild imperative/command.\n' +
            'Usage: Attach た to the continuative form (連用形) of verbs after euphonic change form, かった to the stem of i-adjectives.',
        "#},
        rules: &[
            suffix_inflection("かった", "い", &["-た"], &["adj-i"]),
            suffix_inflection("た", "る", &["-た"], &["v1"]),
            suffix_inflection("いた", "く", &["-た"], &["v5"]),
            suffix_inflection("いだ", "ぐ", &["-た"], &["v5"]),
            suffix_inflection("した", "す", &["-た"], &["v5"]),
            suffix_inflection("った", "う", &["-た"], &["v5"]),
            suffix_inflection("った", "つ", &["-た"], &["v5"]),
            suffix_inflection("った", "る", &["-た"], &["v5"]),
            suffix_inflection("んだ", "ぬ", &["-た"], &["v5"]),
            suffix_inflection("んだ", "ぶ", &["-た"], &["v5"]),
            suffix_inflection("んだ", "む", &["-た"], &["v5"]),
            suffix_inflection("じた", "ずる", &["-た"], &["vz"]),
            suffix_inflection("した", "する", &["-た"], &["vs"]),
            suffix_inflection("為た", "為る", &["-た"], &["vs"]),
            suffix_inflection("きた", "くる", &["-た"], &["vk"]),
            suffix_inflection("来た", "来る", &["-た"], &["vk"]),
            suffix_inflection("來た", "來る", &["-た"], &["vk"]),
            // ...irregularVerbSuffixInflections("た", &["-た"], &["v5"]),
            suffix_inflection("ました", "ます", &["-た"], &["-ます"]),
            suffix_inflection("でした", "", &["-た"], &["-ません"]),
            suffix_inflection("かった", "", &["-た"], &["-ません", "-ん"]),
        ],
    },
    Transform {
        name: "-ます",
        description: indoc! {r#"
            'Polite conjugation of verbs and adjectives.\n' +
            'Usage: Attach ます to the continuative form (連用形) of verbs.',
        "#},
        rules: &[
            suffix_inflection("ます", "る", &["-ます"], &["v1"]),
            suffix_inflection("います", "う", &["-ます"], &["v5d"]),
            suffix_inflection("きます", "く", &["-ます"], &["v5d"]),
            suffix_inflection("ぎます", "ぐ", &["-ます"], &["v5d"]),
            suffix_inflection("します", "す", &["-ます"], &["v5d", "v5s"]),
            suffix_inflection("ちます", "つ", &["-ます"], &["v5d"]),
            suffix_inflection("にます", "ぬ", &["-ます"], &["v5d"]),
            suffix_inflection("びます", "ぶ", &["-ます"], &["v5d"]),
            suffix_inflection("みます", "む", &["-ます"], &["v5d"]),
            suffix_inflection("ります", "る", &["-ます"], &["v5d"]),
            suffix_inflection("じます", "ずる", &["-ます"], &["vz"]),
            suffix_inflection("します", "する", &["-ます"], &["vs"]),
            suffix_inflection("為ます", "為る", &["-ます"], &["vs"]),
            suffix_inflection("きます", "くる", &["-ます"], &["vk"]),
            suffix_inflection("来ます", "来る", &["-ます"], &["vk"]),
            suffix_inflection("來ます", "來る", &["-ます"], &["vk"]),
            suffix_inflection("くあります", "い", &["-ます"], &["adj-i"]),
        ],
    },
    Transform {
        name: "potential",
        description: indoc! {r#"
            'Indicates a state of being (naturally) capable of doing an action.\n' +
            'Usage: Attach (ら)れる to the irrealis form (未然形) of ichidan verbs.\n' +
            'Attach る to the imperative form (命令形) of godan verbs.\n' +
            'する becomes できる, くる becomes こ(ら)れる',
        "#},
        rules: &[
            suffix_inflection("れる", "る", &["v1"], &["v1", "v5d"]),
            suffix_inflection("える", "う", &["v1"], &["v5d"]),
            suffix_inflection("ける", "く", &["v1"], &["v5d"]),
            suffix_inflection("げる", "ぐ", &["v1"], &["v5d"]),
            suffix_inflection("せる", "す", &["v1"], &["v5d"]),
            suffix_inflection("てる", "つ", &["v1"], &["v5d"]),
            suffix_inflection("ねる", "ぬ", &["v1"], &["v5d"]),
            suffix_inflection("べる", "ぶ", &["v1"], &["v5d"]),
            suffix_inflection("める", "む", &["v1"], &["v5d"]),
            suffix_inflection("できる", "する", &["v1"], &["vs"]),
            suffix_inflection("出来る", "する", &["v1"], &["vs"]),
            suffix_inflection("これる", "くる", &["v1"], &["vk"]),
            suffix_inflection("来れる", "来る", &["v1"], &["vk"]),
            suffix_inflection("來れる", "來る", &["v1"], &["vk"]),
        ],
    },
    Transform {
        name: "potential or passive",
        description: indoc! {r#"
            passiveEnglishDescription +
            '3. Indicates a state of being (naturally) capable of doing an action.\n' +
            'Usage: Attach られる to the irrealis form (未然形) of ichidan verbs.\n' +
            'する becomes せられる, くる becomes こられる',
        "#},
        rules: &[
            suffix_inflection("られる", "る", &["v1"], &["v1"]),
            suffix_inflection("ざれる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("ぜられる", "ずる", &["v1"], &["vz"]),
            suffix_inflection("せられる", "する", &["v1"], &["vs"]),
            suffix_inflection("為られる", "為る", &["v1"], &["vs"]),
            suffix_inflection("こられる", "くる", &["v1"], &["vk"]),
            suffix_inflection("来られる", "来る", &["v1"], &["vk"]),
            suffix_inflection("來られる", "來る", &["v1"], &["vk"]),
        ],
    },
    Transform {
        name: "volitional",
        description: indoc! {r#"
            '1. Expresses speaker\'s will or intention.\n' +
            '2. Expresses an invitation to the other party.\n' +
            '3. (Used in …ようとする) Indicates being on the verge of initiating an action or transforming a state.\n' +
            '4. Indicates an inference of a matter.\n' +
            'Usage: Attach よう to the irrealis form (未然形) of ichidan verbs.\n' +
            'Attach う to the irrealis form (未然形) of godan verbs after -o euphonic change form.\n' +
            'Attach かろう to the stem of i-adjectives (4th meaning only).',

        "#},
        rules: &[
            suffix_inflection("よう", "る", &[], &["v1"]),
            suffix_inflection("おう", "う", &[], &["v5"]),
            suffix_inflection("こう", "く", &[], &["v5"]),
            suffix_inflection("ごう", "ぐ", &[], &["v5"]),
            suffix_inflection("そう", "す", &[], &["v5"]),
            suffix_inflection("とう", "つ", &[], &["v5"]),
            suffix_inflection("のう", "ぬ", &[], &["v5"]),
            suffix_inflection("ぼう", "ぶ", &[], &["v5"]),
            suffix_inflection("もう", "む", &[], &["v5"]),
            suffix_inflection("ろう", "る", &[], &["v5"]),
            suffix_inflection("じよう", "ずる", &[], &["vz"]),
            suffix_inflection("しよう", "する", &[], &["vs"]),
            suffix_inflection("為よう", "為る", &[], &["vs"]),
            suffix_inflection("こよう", "くる", &[], &["vk"]),
            suffix_inflection("来よう", "来る", &[], &["vk"]),
            suffix_inflection("來よう", "來る", &[], &["vk"]),
            suffix_inflection("ましょう", "ます", &[], &["-ます"]),
            suffix_inflection("かろう", "い", &[], &["adj-i"]),
        ],
    },
    Transform {
        name: "volitional slang",
        description: indoc! {r#"
            'Contraction of volitional form + か\n' +
            '1. Expresses speaker\'s will or intention.\n' +
            '2. Expresses an invitation to the other party.\n' +
            'Usage: Replace final う with っ of volitional form then add か.\n' +
            'For example: 行こうか -> 行こっか.',

        "#},
        rules: &[
            suffix_inflection("よっか", "る", &[], &["v1"]),
            suffix_inflection("おっか", "う", &[], &["v5"]),
            suffix_inflection("こっか", "く", &[], &["v5"]),
            suffix_inflection("ごっか", "ぐ", &[], &["v5"]),
            suffix_inflection("そっか", "す", &[], &["v5"]),
            suffix_inflection("とっか", "つ", &[], &["v5"]),
            suffix_inflection("のっか", "ぬ", &[], &["v5"]),
            suffix_inflection("ぼっか", "ぶ", &[], &["v5"]),
            suffix_inflection("もっか", "む", &[], &["v5"]),
            suffix_inflection("ろっか", "る", &[], &["v5"]),
            suffix_inflection("じよっか", "ずる", &[], &["vz"]),
            suffix_inflection("しよっか", "する", &[], &["vs"]),
            suffix_inflection("為よっか", "為る", &[], &["vs"]),
            suffix_inflection("こよっか", "くる", &[], &["vk"]),
            suffix_inflection("来よっか", "来る", &[], &["vk"]),
            suffix_inflection("來よっか", "來る", &[], &["vk"]),
            suffix_inflection("ましょっか", "ます", &[], &["-ます"]),
        ],
    },
    Transform {
        name: "-まい",
        description: indoc! {r#"
            'Negative volitional form of verbs.\n' +
            '1. Expresses speaker\'s assumption that something is likely not true.\n' +
            '2. Expresses speaker\'s will or intention not to do something.\n' +
            'Usage: Attach まい to the dictionary form (終止形) of verbs.\n' +
            'Attach まい to the irrealis form (未然形) of ichidan verbs.\n' +
            'する becomes しまい, くる becomes こまい',
        "#},
        rules: &[
            suffix_inflection("まい", "", &[], &["v"]),
            suffix_inflection("まい", "る", &[], &["v1"]),
            suffix_inflection("じまい", "ずる", &[], &["vz"]),
            suffix_inflection("しまい", "する", &[], &["vs"]),
            suffix_inflection("為まい", "為る", &[], &["vs"]),
            suffix_inflection("こまい", "くる", &[], &["vk"]),
            suffix_inflection("来まい", "来る", &[], &["vk"]),
            suffix_inflection("來まい", "來る", &[], &["vk"]),
            suffix_inflection("まい", "", &[], &["-ます"]),
        ],
    },
    Transform {
        name: "-おく",
        description: indoc! {r#"
            'To do certain things in advance in preparation (or in anticipation) of latter needs.\n' +
            'Usage: Attach おく to the て-form of verbs.\n' +
            'Attach でおく after ない negative form of verbs.\n' +
            'Contracts to とく・どく in speech.',
        "#},
        rules: &[
            suffix_inflection("ておく", "て", &["v5"], &["-て"]),
            suffix_inflection("でおく", "で", &["v5"], &["-て"]),
            suffix_inflection("とく", "て", &["v5"], &["-て"]),
            suffix_inflection("どく", "で", &["v5"], &["-て"]),
            suffix_inflection("ないでおく", "ない", &["v5"], &["adj-i"]),
            suffix_inflection("ないどく", "ない", &["v5"], &["adj-i"]),
        ],
    },
    Transform {
        name: "-いる",
        description: indoc! {r#"
            '1. Indicates an action continues or progresses to a point in time.\n' +
            '2. Indicates an action is completed and remains as is.\n' +
            '3. Indicates a state or condition that can be taken to be the result of undergoing some change.\n' +
            'Usage: Attach いる to the て-form of verbs. い can be dropped in speech.\n' +
            'Attach でいる after ない negative form of verbs.\n' +
            '(Slang) Attach おる to the て-form of verbs. Contracts to とる・でる in speech.',
        "#},
        rules: &[
            suffix_inflection("ている", "て", &["v1"], &["-て"]),
            suffix_inflection("ておる", "て", &["v5"], &["-て"]),
            suffix_inflection("てる", "て", &["v1p"], &["-て"]),
            suffix_inflection("でいる", "で", &["v1"], &["-て"]),
            suffix_inflection("でおる", "で", &["v5"], &["-て"]),
            suffix_inflection("でる", "で", &["v1p"], &["-て"]),
            suffix_inflection("とる", "て", &["v5"], &["-て"]),
            suffix_inflection("ないでいる", "ない", &["v1"], &["adj-i"]),
        ],
    },
    Transform {
        name: "-き",
        description: indoc! {r#"
            'Attributive form (連体形) of i-adjectives. An archaic form that remains in modern Japanese.',

        "#},
        rules: &[suffix_inflection("き", "い", &[], &["adj-i"])],
    },
    Transform {
        name: "-げ",
        description: indoc! {r#"
            'Describes a person\'s appearance. Shows feelings of the person.\n' +
            'Usage: Attach げ or 気 to the stem of i-adjectives',

        "#},
        rules: &[
            suffix_inflection("げ", "い", &[], &["adj-i"]),
            suffix_inflection("気", "い", &[], &["adj-i"]),
        ],
    },
    Transform {
        name: "-がる",
        description: indoc! {r#"
            '1. Shows subject’s feelings contrast with what is thought/known about them.\n' +
            '2. Indicates subject\'s behavior (stands out).\n' +
            'Usage: Attach がる to the stem of i-adjectives. It itself conjugates as a godan verb.',

        "#},
        rules: &[suffix_inflection("がる", "い", &["v5"], &["adj-i"])],
    },
    Transform {
        name: "-え",
        description: indoc! {r#"
            'Slang. A sound change of i-adjectives.\n' +
            'ai：やばい → やべぇ\n' +
            'ui：さむい → さみぃ/さめぇ\n' +
            'oi：すごい → すげぇ',
        "#},
        rules: &[
            suffix_inflection("ねえ", "ない", &[], &["adj-i"]),
            suffix_inflection("めえ", "むい", &[], &["adj-i"]),
            suffix_inflection("みい", "むい", &[], &["adj-i"]),
            suffix_inflection("ちぇえ", "つい", &[], &["adj-i"]),
            suffix_inflection("ちい", "つい", &[], &["adj-i"]),
            suffix_inflection("せえ", "すい", &[], &["adj-i"]),
            suffix_inflection("ええ", "いい", &[], &["adj-i"]),
            suffix_inflection("ええ", "わい", &[], &["adj-i"]),
            suffix_inflection("ええ", "よい", &[], &["adj-i"]),
            suffix_inflection("いぇえ", "よい", &[], &["adj-i"]),
            suffix_inflection("うぇえ", "わい", &[], &["adj-i"]),
            suffix_inflection("けえ", "かい", &[], &["adj-i"]),
            suffix_inflection("げえ", "がい", &[], &["adj-i"]),
            suffix_inflection("げえ", "ごい", &[], &["adj-i"]),
            suffix_inflection("せえ", "さい", &[], &["adj-i"]),
            suffix_inflection("めえ", "まい", &[], &["adj-i"]),
            suffix_inflection("ぜえ", "ずい", &[], &["adj-i"]),
            suffix_inflection("っぜえ", "ずい", &[], &["adj-i"]),
            suffix_inflection("れえ", "らい", &[], &["adj-i"]),
            suffix_inflection("れえ", "らい", &[], &["adj-i"]),
            suffix_inflection("ちぇえ", "ちゃい", &[], &["adj-i"]),
            suffix_inflection("でえ", "どい", &[], &["adj-i"]),
            suffix_inflection("れえ", "れい", &[], &["adj-i"]),
            suffix_inflection("べえ", "ばい", &[], &["adj-i"]),
            suffix_inflection("てえ", "たい", &[], &["adj-i"]),
            suffix_inflection("ねぇ", "ない", &[], &["adj-i"]),
            suffix_inflection("めぇ", "むい", &[], &["adj-i"]),
            suffix_inflection("みぃ", "むい", &[], &["adj-i"]),
            suffix_inflection("ちぃ", "つい", &[], &["adj-i"]),
            suffix_inflection("せぇ", "すい", &[], &["adj-i"]),
            suffix_inflection("けぇ", "かい", &[], &["adj-i"]),
            suffix_inflection("げぇ", "がい", &[], &["adj-i"]),
            suffix_inflection("げぇ", "ごい", &[], &["adj-i"]),
            suffix_inflection("せぇ", "さい", &[], &["adj-i"]),
            suffix_inflection("めぇ", "まい", &[], &["adj-i"]),
            suffix_inflection("ぜぇ", "ずい", &[], &["adj-i"]),
            suffix_inflection("っぜぇ", "ずい", &[], &["adj-i"]),
            suffix_inflection("れぇ", "らい", &[], &["adj-i"]),
            suffix_inflection("でぇ", "どい", &[], &["adj-i"]),
            suffix_inflection("れぇ", "れい", &[], &["adj-i"]),
            suffix_inflection("べぇ", "ばい", &[], &["adj-i"]),
            suffix_inflection("てぇ", "たい", &[], &["adj-i"]),
        ],
    },
    Transform {
        name: "n-slang",
        description: indoc! {r#"
            'Slang sound change of r-column syllables to n (when before an n-sound, usually の or な)',
        "#},
        rules: &[
            suffix_inflection("んなさい", "りなさい", &[], &["-なさい"]),
            suffix_inflection("らんない", "られない", &["adj-i"], &["adj-i"]),
            suffix_inflection("んない", "らない", &["adj-i"], &["adj-i"]),
            suffix_inflection("んなきゃ", "らなきゃ", &[], &["-ゃ"]),
            suffix_inflection("んなきゃ", "れなきゃ", &[], &["-ゃ"]),
        ],
    },
    Transform {
        name: "imperative negative slang",
        description: "",
        rules: &[suffix_inflection("んな", "る", &[], &["v"])],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            'Negative form of kansai-ben verbs',
        "#},
        rules: &[
            suffix_inflection("へん", "ない", &[], &["adj-i"]),
            suffix_inflection("ひん", "ない", &[], &["adj-i"]),
            suffix_inflection("せえへん", "しない", &[], &["adj-i"]),
            suffix_inflection("へんかった", "なかった", &["-た"], &["-た"]),
            suffix_inflection("ひんかった", "なかった", &["-た"], &["-た"]),
            suffix_inflection("うてへん", "ってない", &[], &["adj-i"]),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            '-て form of kansai-ben verbs',

        "#},
        rules: &[
            suffix_inflection("うて", "って", &["-て"], &["-て"]),
            suffix_inflection("おうて", "あって", &["-て"], &["-て"]),
            suffix_inflection("こうて", "かって", &["-て"], &["-て"]),
            suffix_inflection("ごうて", "がって", &["-て"], &["-て"]),
            suffix_inflection("そうて", "さって", &["-て"], &["-て"]),
            suffix_inflection("ぞうて", "ざって", &["-て"], &["-て"]),
            suffix_inflection("とうて", "たって", &["-て"], &["-て"]),
            suffix_inflection("どうて", "だって", &["-て"], &["-て"]),
            suffix_inflection("のうて", "なって", &["-て"], &["-て"]),
            suffix_inflection("ほうて", "はって", &["-て"], &["-て"]),
            suffix_inflection("ぼうて", "ばって", &["-て"], &["-て"]),
            suffix_inflection("もうて", "まって", &["-て"], &["-て"]),
            suffix_inflection("ろうて", "らって", &["-て"], &["-て"]),
            suffix_inflection("ようて", "やって", &["-て"], &["-て"]),
            suffix_inflection("ゆうて", "いって", &["-て"], &["-て"]),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            '-た form of kansai-ben terms',

        "#},
        rules: &[
            suffix_inflection("うた", "った", &["-た"], &["-た"]),
            suffix_inflection("おうた", "あった", &["-た"], &["-た"]),
            suffix_inflection("こうた", "かった", &["-た"], &["-た"]),
            suffix_inflection("ごうた", "がった", &["-た"], &["-た"]),
            suffix_inflection("そうた", "さった", &["-た"], &["-た"]),
            suffix_inflection("ぞうた", "ざった", &["-た"], &["-た"]),
            suffix_inflection("とうた", "たった", &["-た"], &["-た"]),
            suffix_inflection("どうた", "だった", &["-た"], &["-た"]),
            suffix_inflection("のうた", "なった", &["-た"], &["-た"]),
            suffix_inflection("ほうた", "はった", &["-た"], &["-た"]),
            suffix_inflection("ぼうた", "ばった", &["-た"], &["-た"]),
            suffix_inflection("もうた", "まった", &["-た"], &["-た"]),
            suffix_inflection("ろうた", "らった", &["-た"], &["-た"]),
            suffix_inflection("ようた", "やった", &["-た"], &["-た"]),
            suffix_inflection("ゆうた", "いった", &["-た"], &["-た"]),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            '-たら form of kansai-ben terms',

        "#},
        rules: &[
            suffix_inflection("うたら", "ったら", &[], &[]),
            suffix_inflection("おうたら", "あったら", &[], &[]),
            suffix_inflection("こうたら", "かったら", &[], &[]),
            suffix_inflection("ごうたら", "がったら", &[], &[]),
            suffix_inflection("そうたら", "さったら", &[], &[]),
            suffix_inflection("ぞうたら", "ざったら", &[], &[]),
            suffix_inflection("とうたら", "たったら", &[], &[]),
            suffix_inflection("どうたら", "だったら", &[], &[]),
            suffix_inflection("のうたら", "なったら", &[], &[]),
            suffix_inflection("ほうたら", "はったら", &[], &[]),
            suffix_inflection("ぼうたら", "ばったら", &[], &[]),
            suffix_inflection("もうたら", "まったら", &[], &[]),
            suffix_inflection("ろうたら", "らったら", &[], &[]),
            suffix_inflection("ようたら", "やったら", &[], &[]),
            suffix_inflection("ゆうたら", "いったら", &[], &[]),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            '-たり form of kansai-ben terms',

        "#},
        rules: &[
            suffix_inflection("うたり", "ったり", &[], &[]),
            suffix_inflection("おうたり", "あったり", &[], &[]),
            suffix_inflection("こうたり", "かったり", &[], &[]),
            suffix_inflection("ごうたり", "がったり", &[], &[]),
            suffix_inflection("そうたり", "さったり", &[], &[]),
            suffix_inflection("ぞうたり", "ざったり", &[], &[]),
            suffix_inflection("とうたり", "たったり", &[], &[]),
            suffix_inflection("どうたり", "だったり", &[], &[]),
            suffix_inflection("のうたり", "なったり", &[], &[]),
            suffix_inflection("ほうたり", "はったり", &[], &[]),
            suffix_inflection("ぼうたり", "ばったり", &[], &[]),
            suffix_inflection("もうたり", "まったり", &[], &[]),
            suffix_inflection("ろうたり", "らったり", &[], &[]),
            suffix_inflection("ようたり", "やったり", &[], &[]),
            suffix_inflection("ゆうたり", "いったり", &[], &[]),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            '-く stem of kansai-ben adjectives',

        "#},
        rules: &[
            suffix_inflection("う", "く", &[], &["-く"]),
            suffix_inflection("こう", "かく", &[], &["-く"]),
            suffix_inflection("ごう", "がく", &[], &["-く"]),
            suffix_inflection("そう", "さく", &[], &["-く"]),
            suffix_inflection("とう", "たく", &[], &["-く"]),
            suffix_inflection("のう", "なく", &[], &["-く"]),
            suffix_inflection("ぼう", "ばく", &[], &["-く"]),
            suffix_inflection("もう", "まく", &[], &["-く"]),
            suffix_inflection("ろう", "らく", &[], &["-く"]),
            suffix_inflection("よう", "よく", &[], &["-く"]),
            suffix_inflection("しゅう", "しく", &[], &["-く"]),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            '-て form of kansai-ben adjectives',

        "#},
        rules: &[
            suffix_inflection("うて", "くて", &["-て"], &["-て"]),
            suffix_inflection("こうて", "かくて", &["-て"], &["-て"]),
            suffix_inflection("ごうて", "がくて", &["-て"], &["-て"]),
            suffix_inflection("そうて", "さくて", &["-て"], &["-て"]),
            suffix_inflection("とうて", "たくて", &["-て"], &["-て"]),
            suffix_inflection("のうて", "なくて", &["-て"], &["-て"]),
            suffix_inflection("ぼうて", "ばくて", &["-て"], &["-て"]),
            suffix_inflection("もうて", "まくて", &["-て"], &["-て"]),
            suffix_inflection("ろうて", "らくて", &["-て"], &["-て"]),
            suffix_inflection("ようて", "よくて", &["-て"], &["-て"]),
            suffix_inflection("しゅうて", "しくて", &["-て"], &["-て"]),
        ],
    },
    Transform {
        name: "kansai-ben",
        description: indoc! {r#"
            'Negative form of kansai-ben adjectives',
        "#},
        rules: &[
            suffix_inflection("うない", "くない", &["adj-i"], &["adj-i"]),
            suffix_inflection("こうない", "かくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("ごうない", "がくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("そうない", "さくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("とうない", "たくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("のうない", "なくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("ぼうない", "ばくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("もうない", "まくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("ろうない", "らくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("ようない", "よくない", &["adj-i"], &["adj-i"]),
            suffix_inflection("しゅうない", "しくない", &["adj-i"], &["adj-i"]),
        ],
    },
];
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
