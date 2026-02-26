use std::collections::HashMap;
use std::range::Range;

use lazy_static::lazy_static;

pub const HALFWIDTH_RANGE: Range<usize> = Range {
    start: 0xFF61,
    end: 0xFF9F + 1, // End is non-inclusive
};

pub const ANY_DAKUTEN_RANGE: Range<usize> = Range {
    start: 0x3099,
    end: 0x309C + 1,
};

pub const DAKUTEN_COMB: char = '\u{3099}';
pub const DAKUTEN_FULL: char = '\u{309B}';
pub const DAKUTEN_HALF: char = '\u{FF9E}'; // Halfwidth Katakana Voiced Sound Mark `ﾞ`
pub const DAKUTEN_HALF_BYTES: [u8; 3] = char_bytes(DAKUTEN_HALF);

pub const HANDAKUTEN_COMB: char = '\u{309A}';
pub const HANDAKUTEN_FULL: char = '\u{309C}';
pub const HANDAKUTEN_HALF: char = '\u{FF9F}'; // Halfwidth Katakana Semi-Voiced Sound Mark `ﾟ`
pub const HANDAKUTEN_HALF_BYTES: [u8; 3] = char_bytes(HANDAKUTEN_HALF);

const fn char_bytes<const N: usize>(c: char) -> [u8; N] {
    let mut buffer = [0_u8; N];

    c.encode_utf8(&mut buffer);

    buffer
}

pub const fn is_halfwidth(c: char) -> bool {
    HALFWIDTH_RANGE.contains(&(c as usize))
}

pub const fn is_dakuten(c: char) -> bool {
    const DAKUTEN_HALF_USIZE: usize = DAKUTEN_HALF as usize;
    const HANDAKUTEN_HALF_USIZE: usize = HANDAKUTEN_HALF as usize;

    let i = c as usize;

    ANY_DAKUTEN_RANGE.contains(&i) || i == DAKUTEN_HALF_USIZE || i == HANDAKUTEN_HALF_USIZE
}

pub fn is_cjk_compatibility_character(c: char) -> bool {
    const CJK_COMPATIBILITY_RANGE: Range<usize> = Range {
        start: 0x3300,
        end: 0x33FF + 1,
    };

    CJK_COMPATIBILITY_RANGE.contains(&(c as usize))
}

pub fn is_punctuation(c: char) -> bool {
    const CJK_PUNCTUATION_RANGE: Range<usize> = Range {
        start: 0x3000,
        end: 0x303F + 1,
    };

    CJK_PUNCTUATION_RANGE.contains(&(c as usize)) || c == '!'
}

pub fn get_single_char(s: &str) -> Option<char> {
    let mut chars = s.chars();

    match chars.next() {
        Some(c) if chars.next().is_none() => Some(c),
        _ => None,
    }
}

lazy_static! {
    pub static ref HANDAKUTEN_MAP: HashMap<char, char> = [
        ('\u{30CF}', '\u{30D1}'),   //  ハ	30CF	パ	30D1
        ('\u{30D2}', '\u{30D4}'),   //  ヒ	30D2	ピ	30D4
        ('\u{30D5}', '\u{30D7}'),   //  フ	30D5	プ	30D7
        ('\u{30D8}', '\u{30DA}'),   //  ヘ	30D8	ペ	30DA
        ('\u{30DB}', '\u{30DD}'),   //  ホ	30DB	ポ	30DD
        ('\u{306F}', '\u{3071}'),   //  は	306F	ぱ	3071
        ('\u{3072}', '\u{3074}'),   //  ひ	3072	ぴ	3074
        ('\u{3075}', '\u{3077}'),   //  ふ	3075	ぷ	3077
        ('\u{3078}', '\u{307A}'),   //  へ	3078	ぺ	307A
        ('\u{307B}', '\u{307D}'),   //  ほ	307B	ぽ	307D
    ].iter().copied().collect();

    pub static ref DAKUTEN_MAP: HashMap<char, char> = [
        ('\u{30A6}', '\u{30F4}'),   //  ウ	30A6	ヴ	30F4
        ('\u{30AB}', '\u{30AC}'),   //  カ	30AB	ガ	30AC
        ('\u{30AD}', '\u{30AE}'),   //  キ	30AD	ギ	30AE
        ('\u{30AF}', '\u{30B0}'),   //  ク	30AF	グ	30B0
        ('\u{30B1}', '\u{30B2}'),   //  ケ	30B1	ゲ	30B2
        ('\u{30B3}', '\u{30B4}'),   //  コ	30B3	ゴ	30B4
        ('\u{30B5}', '\u{30B6}'),   //  サ	30B5	ザ	30B6
        ('\u{30B7}', '\u{30B8}'),   //  シ	30B7	ジ	30B8
        ('\u{30B9}', '\u{30BA}'),   //  ス	30B9	ズ	30BA
        ('\u{30BB}', '\u{30BC}'),   //  セ	30BB	ゼ	30BC
        ('\u{30BD}', '\u{30BE}'),   //  ソ	30BD	ゾ	30BE
        ('\u{30BF}', '\u{30C0}'),   //  タ	30BF	ダ	30C0
        ('\u{30C1}', '\u{30C2}'),   //  チ	30C1	ヂ	30C2
        ('\u{30C4}', '\u{30C5}'),   //  ツ	30C4	ヅ	30C5
        ('\u{30C6}', '\u{30C7}'),   //  テ	30C6	デ	30C7
        ('\u{30C8}', '\u{30C9}'),   //  ト	30C8	ド	30C9
        ('\u{30CF}', '\u{30D0}'),   //  ハ	30CF	バ	30D0
        ('\u{30D2}', '\u{30D3}'),   //  ヒ	30D2	ビ	30D3
        ('\u{30D5}', '\u{30D6}'),   //  フ	30D5	ブ	30D6
        ('\u{30D8}', '\u{30D9}'),   //  ヘ	30D8	ベ	30D9
        ('\u{30DB}', '\u{30DC}'),   //  ホ	30DB	ボ	30DC
        ('\u{30EF}', '\u{30F7}'),   //  ワ	30EF	ヷ	30F7
        ('\u{30F0}', '\u{30F8}'),   //  ヰ	30F0	ヸ	30F8
        ('\u{30F1}', '\u{30F9}'),   //  ヱ	30F1	ヹ	30F9
        ('\u{30F2}', '\u{30FA}'),   //  ヲ	30F2	ヺ	30FA
        ('\u{3046}', '\u{3094}'),   //  う	3046	ゔ	3094
        ('\u{304B}', '\u{304C}'),   //  か	304B	が	304C
        ('\u{304D}', '\u{304E}'),   //  き	304D	ぎ	304E
        ('\u{304F}', '\u{3050}'),   //  く	304F	ぐ	3050
        ('\u{3051}', '\u{3052}'),   //  け	3051	げ	3052
        ('\u{3053}', '\u{3054}'),   //  こ	3053	ご	3054
        ('\u{3055}', '\u{3056}'),   //  さ	3055	ざ	3056
        ('\u{3057}', '\u{3058}'),   //  し	3057	じ	3058
        ('\u{3059}', '\u{305A}'),   //  す	3059	ず	305A
        ('\u{305B}', '\u{305C}'),   //  せ	305B	ぜ	305C
        ('\u{305D}', '\u{305E}'),   //  そ	305D	ぞ	305E
        ('\u{305F}', '\u{3060}'),   //  た	305F	だ	3060
        ('\u{3061}', '\u{3062}'),   //  ち	3061	ぢ	3062
        ('\u{3064}', '\u{3065}'),   //  つ	3064	づ	3065
        ('\u{3066}', '\u{3067}'),   //  て	3066	で	3067
        ('\u{3068}', '\u{3069}'),   //  と	3068	ど	3069
        ('\u{306F}', '\u{3070}'),   //  は	306F	ば	3070
        ('\u{3072}', '\u{3073}'),   //  ひ	3072	び	3073
        ('\u{3075}', '\u{3076}'),   //  ふ	3075	ぶ	3076
        ('\u{3078}', '\u{3079}'),   //  へ	3078	べ	3079
        ('\u{307B}', '\u{307C}'),   //  ほ	307B	ぼ	307C
        ('\u{309D}', '\u{309E}'),   //  ゝ	309D	ゞ	309E
    ].iter().copied().collect();

    pub static ref HALF_WIDTH_HANDAKUTEN_MAP: HashMap<char, char> = [
        ('\u{FF8A}', '\u{30D1}'),   //  ﾊ	FF8A	パ	30D1
        ('\u{FF8B}', '\u{30D4}'),   //  ﾋ	FF8B	ピ	30D4
        ('\u{FF8C}', '\u{30D7}'),   //  ﾌ	FF8C	プ	30D7
        ('\u{FF8D}', '\u{30DA}'),   //  ﾍ	FF8D	ペ	30DA
        ('\u{FF8E}', '\u{30DD}'),   //  ﾎ	FF8E	ポ	30DD
    ].iter().copied().collect();

    pub static ref HALF_WIDTH_DAKUTEN_MAP: HashMap<char, char> = [
        ('\u{FF66}', '\u{30FA}'),   //  ｦ	FF66	ヺ	30FA
        ('\u{FF73}', '\u{30F4}'),   //  ｳ	FF73	ヴ	30F4
        ('\u{FF76}', '\u{30AC}'),   //  ｶ	FF76	ガ	30AC
        ('\u{FF77}', '\u{30AE}'),   //  ｷ	FF77	ギ	30AE
        ('\u{FF78}', '\u{30B0}'),   //  ｸ	FF78	グ	30B0
        ('\u{FF79}', '\u{30B2}'),   //  ｹ	FF79	ゲ	30B2
        ('\u{FF7A}', '\u{30B4}'),   //  ｺ	FF7A	ゴ	30B4
        ('\u{FF7B}', '\u{30B6}'),   //  ｻ	FF7B	ザ	30B6
        ('\u{FF7C}', '\u{30B8}'),   //  ｼ	FF7C	ジ	30B8
        ('\u{FF7D}', '\u{30BA}'),   //  ｽ	FF7D	ズ	30BA
        ('\u{FF7E}', '\u{30BC}'),   //  ｾ	FF7E	ゼ	30BC
        ('\u{FF7F}', '\u{30BE}'),   //  ｿ	FF7F	ゾ	30BE
        ('\u{FF80}', '\u{30C0}'),   //  ﾀ	FF80	ダ	30C0
        ('\u{FF81}', '\u{30C2}'),   //  ﾁ	FF81	ヂ	30C2
        ('\u{FF82}', '\u{30C5}'),   //  ﾂ	FF82	ヅ	30C5
        ('\u{FF83}', '\u{30C7}'),   //  ﾃ	FF83	デ	30C7
        ('\u{FF84}', '\u{30C9}'),   //  ﾄ	FF84	ド	30C9
        ('\u{FF8A}', '\u{30D0}'),   //  ﾊ	FF8A	バ	30D0
        ('\u{FF8B}', '\u{30D3}'),   //  ﾋ	FF8B	ビ	30D3
        ('\u{FF8C}', '\u{30D6}'),   //  ﾌ	FF8C	ブ	30D6
        ('\u{FF8D}', '\u{30D9}'),   //  ﾍ	FF8D	ベ	30D9
        ('\u{FF8E}', '\u{30DC}'),   //  ﾎ	FF8E	ボ	30DC
        ('\u{FF9C}', '\u{30F7}'),   //  ﾜ	FF9C	ヷ	30F7
    ].iter().copied().collect();

    pub static ref HALF_WIDTH_MAP: HashMap<char, char> = [
        ('\u{FF61}', '\u{3002}'),   //  ｡	FF61	。	3002
        ('\u{FF62}', '\u{300C}'),   //  ｢	FF62	「	300C
        ('\u{FF63}', '\u{300D}'),   //  ｣	FF63	」	300D
        ('\u{FF64}', '\u{3001}'),   //  ､	FF64	、	3001
        ('\u{FF65}', '\u{30FB}'),   //  ･	FF65	・	30FB
        ('\u{FF66}', '\u{30F2}'),   //  ｦ	FF66	ヲ	30F2
        ('\u{FF67}', '\u{30A1}'),   //  ｧ	FF67	ァ	30A1
        ('\u{FF68}', '\u{30A3}'),   //  ｨ	FF68	ィ	30A3
        ('\u{FF69}', '\u{30A5}'),   //  ｩ	FF69	ゥ	30A5
        ('\u{FF6A}', '\u{30A7}'),   //  ｪ	FF6A	ェ	30A7
        ('\u{FF6B}', '\u{30A9}'),   //  ｫ	FF6B	ォ	30A9
        ('\u{FF6C}', '\u{30E3}'),   //  ｬ	FF6C	ャ	30E3
        ('\u{FF6D}', '\u{30E5}'),   //  ｭ	FF6D	ュ	30E5
        ('\u{FF6E}', '\u{30E7}'),   //  ｮ	FF6E	ョ	30E7
        ('\u{FF6F}', '\u{30C3}'),   //  ｯ	FF6F	ッ	30C3
        ('\u{FF70}', '\u{30FC}'),   //  ｰ	FF70	ー	30FC
        ('\u{FF71}', '\u{30A2}'),   //  ｱ	FF71	ア	30A2
        ('\u{FF72}', '\u{30A4}'),   //  ｲ	FF72	イ	30A4
        ('\u{FF73}', '\u{30A6}'),   //  ｳ	FF73	ウ	30A6
        ('\u{FF74}', '\u{30A8}'),   //  ｴ	FF74	エ	30A8
        ('\u{FF75}', '\u{30AA}'),   //  ｵ	FF75	オ	30AA
        ('\u{FF76}', '\u{30AB}'),   //  ｶ	FF76	カ	30AB
        ('\u{FF77}', '\u{30AD}'),   //  ｷ	FF77	キ	30AD
        ('\u{FF78}', '\u{30AF}'),   //  ｸ	FF78	ク	30AF
        ('\u{FF79}', '\u{30B1}'),   //  ｹ	FF79	ケ	30B1
        ('\u{FF7A}', '\u{30B3}'),   //  ｺ	FF7A	コ	30B3
        ('\u{FF7B}', '\u{30B5}'),   //  ｻ	FF7B	サ	30B5
        ('\u{FF7C}', '\u{30B7}'),   //  ｼ	FF7C	シ	30B7
        ('\u{FF7D}', '\u{30B9}'),   //  ｽ	FF7D	ス	30B9
        ('\u{FF7E}', '\u{30BB}'),   //  ｾ	FF7E	セ	30BB
        ('\u{FF7F}', '\u{30BD}'),   //  ｿ	FF7F	ソ	30BD
        ('\u{FF80}', '\u{30BF}'),   //  ﾀ	FF80	タ	30BF
        ('\u{FF81}', '\u{30C1}'),   //  ﾁ	FF81	チ	30C1
        ('\u{FF82}', '\u{30C4}'),   //  ﾂ	FF82	ツ	30C4
        ('\u{FF83}', '\u{30C6}'),   //  ﾃ	FF83	テ	30C6
        ('\u{FF84}', '\u{30C8}'),   //  ﾄ	FF84	ト	30C8
        ('\u{FF85}', '\u{30CA}'),   //  ﾅ	FF85	ナ	30CA
        ('\u{FF86}', '\u{30CB}'),   //  ﾆ	FF86	ニ	30CB
        ('\u{FF87}', '\u{30CC}'),   //  ﾇ	FF87	ヌ	30CC
        ('\u{FF88}', '\u{30CD}'),   //  ﾈ	FF88	ネ	30CD
        ('\u{FF89}', '\u{30CE}'),   //  ﾉ	FF89	ノ	30CE
        ('\u{FF8A}', '\u{30CF}'),   //  ﾊ	FF8A	ハ	30CF
        ('\u{FF8B}', '\u{30D2}'),   //  ﾋ	FF8B	ヒ	30D2
        ('\u{FF8C}', '\u{30D5}'),   //  ﾌ	FF8C	フ	30D5
        ('\u{FF8D}', '\u{30D8}'),   //  ﾍ	FF8D	ヘ	30D8
        ('\u{FF8E}', '\u{30DB}'),   //  ﾎ	FF8E	ホ	30DB
        ('\u{FF8F}', '\u{30DE}'),   //  ﾏ	FF8F	マ	30DE
        ('\u{FF90}', '\u{30DF}'),   //  ﾐ	FF90	ミ	30DF
        ('\u{FF91}', '\u{30E0}'),   //  ﾑ	FF91	ム	30E0
        ('\u{FF92}', '\u{30E1}'),   //  ﾒ	FF92	メ	30E1
        ('\u{FF93}', '\u{30E2}'),   //  ﾓ	FF93	モ	30E2
        ('\u{FF94}', '\u{30E4}'),   //  ﾔ	FF94	ヤ	30E4
        ('\u{FF95}', '\u{30E6}'),   //  ﾕ	FF95	ユ	30E6
        ('\u{FF96}', '\u{30E8}'),   //  ﾖ	FF96	ヨ	30E8
        ('\u{FF97}', '\u{30E9}'),   //  ﾗ	FF97	ラ	30E9
        ('\u{FF98}', '\u{30EA}'),   //  ﾘ	FF98	リ	30EA
        ('\u{FF99}', '\u{30EB}'),   //  ﾙ	FF99	ル	30EB
        ('\u{FF9A}', '\u{30EC}'),   //  ﾚ	FF9A	レ	30EC
        ('\u{FF9B}', '\u{30ED}'),   //  ﾛ	FF9B	ロ	30ED
        ('\u{FF9C}', '\u{30EF}'),   //  ﾜ	FF9C	ワ	30EF
        ('\u{FF9D}', '\u{30F3}'),   //  ﾝ	FF9D	ン	30F3
        ('\u{FF9E}', '\u{3099}'),   //  ﾞ	FF9E	 ゙	3099
        ('\u{FF9F}', '\u{309A}'),   //  ﾟ	FF9F	 ゚	309A
    ].iter().copied().collect();
}
