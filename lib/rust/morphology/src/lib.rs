#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/*
use std::{
    collections::{HashMap, HashSet},
    fs::{self, read_dir},
    ops::Not,
    path::{Path, PathBuf},
    str::FromStr,
};

use color_eyre::eyre::Result;
use mecab::Tagger;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use srs_bridge_anki::client::{
    actions::{
        card::{CardsInfoRequest, FindCardsRequest},
        deck::{DeckNamesAndIdsRequest, DeckNamesRequest},
    },
    AnkiClient,
};
use subtitle::{format::srt::SrtLine, get_subtitle_file, get_subtitle_format, SubtitleFile};

mod subtitle;

fn known_words() -> Vec<&'static str> {
    vec![
        "いる",
        "する",
        "何",
        "これ",
        "学校",
        "行く",
        "たくさん",
        "子供",
        "連れる",
        "犬",
        "宿題",
        "私",
        "屋",
        "ある",
        "あそこ",
        "人",
        "教室",
        "そんな",
        "鳴る",
        "買う",
        "本",
        "有名",
        "読む",
        "上がる",
        "成績",
        "ここ",
        "言う",
        "この",
        "問題",
        "考える",
        "後",
        "毎日",
        "彼",
        "来る",
        "彼氏",
        "電話",
        "集まる",
        "事",
        "好き",
        "僕",
        "勉強",
        "英語",
        "教える",
        "分かる",
        "手",
        "上げる",
        "君",
        "全然",
        "友達",
        "会う",
        "生徒",
        "兄",
        "大学",
        "部",
        "方",
        "より",
        "力",
        "猫",
        "上",
        "置く",
        "色々",
        "かばん",
        "家",
        "美味しい",
        "遊ぶ",
        "明日",
        "料理",
        "そう",
        "立つ",
        "ください",
        "ゆっくり",
        "作る",
        "その",
        "通る",
        "高校",
        "彼女",
        "手伝う",
        "両親",
        "もらう",
        "手紙",
        "くれる",
        "そこ",
        "場所",
        "ありがとう",
        "様",
        "いらっしゃる",
        "数学",
        "解く",
        "難しい",
        "とても",
        "中",
        "忙しい",
        "仕事",
        "疲れる",
        "休憩",
        "取る",
        "任せる",
        "新しい",
        "前",
        "朝",
        "降る",
        "日",
        "なる",
        "聞く",
        "先生",
        "今年",
        "冬",
        "寒い",
        "誰",
        "店",
        "入る",
        "本当",
        "妹",
        "買い物",
        "携帯",
        "今日",
        "暑い",
        "ちょっと",
        "見る",
        "一緒",
        "映画",
        "面白い",
        "周り",
        "もちろん",
        "知る",
        "すごい",
        "みんな",
        "名前",
        "思い",
        "出す",
        "忘れる",
        "恥ずかしい",
        "乾かす",
        "髪",
        "変わる",
        "長い",
        "足",
        "強い",
        "別",
        "色",
        "まだ",
        "歳",
        "生きる",
        "二人",
        "環境",
        "息子",
        "受ける",
        "授業",
        "終わる",
        "俺",
        "もう",
        "待つ",
        "椅子",
        "すぐ",
        "試合",
        "始まる",
        "年",
        "時",
        "経つ",
        "最近",
        "座る",
        "走る",
        "必死",
        "机",
        "鉛筆",
        "子",
        "運動",
        "もっと",
        "夢",
        "不思議",
        "歌",
        "上手",
        "絵",
        "上手い",
        "どれ",
        "くらい",
        "褒める",
        "なかなか",
        "眠る",
        "思う",
        "綺麗",
        "花",
        "咲く",
        "桜",
        "春",
        "公園",
        "夜",
        "風呂",
        "脱ぐ",
        "部屋",
        "片付ける",
        "一人",
        "食べる",
        "全部",
        "月",
        "服",
        "山",
        "見える",
        "今夜",
        "優しい",
        "対する",
        "厳しい",
        "どう",
        "あなた",
        "頃",
        "小学生",
        "どんどん",
        "音",
        "雨",
        "聞こえる",
        "海",
        "波",
        "はっきり",
        "応援",
        "選手",
        "目指す",
        "将来",
        "野球",
        "物",
        "姉",
        "やる",
        "自分",
        "高い",
        "選ぶ",
        "間違う",
        "答え",
        "しまう",
        "濡れる",
        "止まる",
        "雪",
        "戦争",
        "世界",
        "平和",
        "弟",
        "車",
        "多い",
        "危ない",
        "道",
        "父親",
        "病院",
        "気持ち",
        "伝える",
        "飲む",
        "酒",
        "風",
        "冷たい",
        "吹く",
        "どんな",
        "危険",
        "こんな",
        "初めて",
        "肉",
        "昨日",
        "覚える",
        "今",
        "話す",
        "一番",
        "外",
        "電気",
        "暗い",
        "気をつける",
        "迎える",
        "質問",
        "答える",
        "出来る",
        "祈る",
        "受験",
        "合格",
        "休む",
        "しばらく",
        "写真",
        "家族",
        "飾る",
        "壁",
        "塗る",
        "旅行",
        "帰る",
        "遅れる",
        "今朝",
        "ごめん",
        "時間",
        "大きい",
        "卒業",
        "会社",
        "働く",
        "掛ける",
        "起きる",
        "早い",
        "出る",
        "悪い",
        "楽しい",
        "気",
        "意味",
        "得意",
        "母親",
        "欲しい",
        "だけ",
        "頑張る",
        "持つ",
        "痛い",
        "頭",
        "人気",
        "商品",
        "払う",
        "金",
        "小さい",
        "女優",
        "勝つ",
        "ため",
        "体",
        "当たり前",
        "過ぎる",
        "太る",
        "世話",
        "間",
        "食う",
        "飯",
        "着く",
        "話",
        "声",
        "男",
        "いつも",
        "合う",
        "通り",
        "いっぱい",
        "若い",
        "女",
        "どこ",
        "向かう",
        "次",
        "飛ぶ",
        "打つ",
        "探す",
        "捕まる",
        "事件",
        "逃げる",
        "犯人",
        "現実",
        "描く",
        "けが",
        "せい",
        "変",
        "様子",
        "見つける",
        "やっと",
        "素敵",
        "駅",
        "食事",
        "眼鏡",
        "町",
        "あれ",
        "洗う",
        "顔",
        "少し",
        "使う",
        "頼む",
        "違う",
        "空",
        "鳥",
        "そろそろ",
        "始める",
        "乗る",
        "生活",
        "週間",
        "続く",
        "赤い",
        "着る",
        "女の子",
        "泣く",
        "記憶",
        "失う",
        "昔",
        "それ",
        "無理",
        "ご飯",
        "絶対",
        "怒る",
        "ちゃんと",
        "倒れる",
        "席",
        "老人",
        "助ける",
        "捨てる",
        "所",
        "静か",
        "夏休み",
        "最後",
        "戦う",
        "調べる",
        "情報",
        "近く",
        "住む",
        "回る",
        "決まる",
        "当たる",
        "冗談",
        "付く",
        "失敗",
        "予定",
        "殴る",
        "誘う",
        "暮らす",
        "また",
        "いつ",
        "どういう",
        "関係",
        "時計",
        "大きな",
        "掛かる",
        "酷い",
        "おかげ",
        "けんか",
        "親",
        "安い",
        "大好き",
        "可愛い",
        "かなり",
        "数",
        "減る",
        "隣",
        "貸す",
        "頼る",
        "集中",
        "壊れる",
        "魚",
        "音楽",
        "楽しむ",
        "自然",
        "豊か",
        "表情",
        "体力",
        "吸う",
        "部活",
        "負ける",
        "辞める",
        "近づく",
        "削る",
        "口",
        "言葉",
        "我々",
        "人間",
        "笑う",
        "歩く",
        "お父さん",
        "病気",
        "死ぬ",
        "開く",
        "殺す",
        "戻る",
        "入れる",
        "敵",
        "以上",
        "振る",
        "動く",
        "必要",
        "他",
        "書く",
        "落ちる",
        "引く",
        "練習",
        "続ける",
        "行う",
        "残る",
        "ずっと",
        "今度",
        "包丁",
        "新鮮",
        "農家",
        "畑",
        "野菜",
        "切る",
        "あんな",
        "奴",
        "驚く",
        "黙る",
        "説明",
        "似る",
        "急",
        "困る",
        "借りる",
        "返す",
        "大丈夫",
        "時代",
        "以外",
        "気分",
        "隠す",
        "仲間",
        "送る",
        "許す",
        "どうぞ",
        "こちら",
        "向く",
        "起こす",
        "結果",
        "認める",
        "地震",
        "突然",
        "簡単",
        "怖い",
        "確認",
        "生まれる",
        "全員",
        "途中",
        "急ぐ",
        "抜く",
        "噂",
        "勝手に",
        "勝手",
        "約束",
        "遅い",
        "貼る",
        "届く",
        "挨拶",
        "先輩",
        "連絡",
        "渡す",
        "必ず",
        "特に",
        "無事",
        "済む",
        "大人",
        "売る",
        "準備",
        "嬉しい",
        "離す",
        "触る",
        "癖",
        "大事",
        "鍵",
        "今回",
        "奪う",
        "なくなる",
        "掴む",
        "まるで",
        "窓",
        "景色",
        "絵画",
        "本物",
        "気付く",
        "嫌い",
        "面倒",
        "迷惑",
        "助かる",
        "祭",
        "花火",
        "打ち上げ",
        "参加",
        "幸せ",
        "恋人",
        "寂しい",
        "過ごす",
        "断る",
        "真剣",
        "こう",
        "ばかり",
        "頂く",
        "分",
        "一度",
        "信じる",
        "成功",
        "望む",
        "おかしい",
        "うるさい",
        "大体",
        "なるほど",
        "嘘",
        "間に合う",
        "どうせ",
        "見つかる",
        "間違い",
        "久しぶり",
        "致す",
        "繋ぐ",
        "いきなり",
        "びっくり",
        "お母さん",
        "おる",
        "喋る",
        "構う",
        "詰まる",
        "仲",
        "暇",
        "腹",
        "どの",
        "参る",
        "苦手",
        "揃う",
        "さて",
        "大会",
        "悔しい",
        "腕",
        "悩む",
        "迷う",
        "行動",
        "学年",
        "狙う",
        "入学",
        "希望",
        "熱い",
        "下がる",
        "お礼",
        "素直",
        "直る",
        "嫌う",
        "機会",
        "まくる",
        "叱る",
        "散々",
        "日々",
        "格好",
        "めっちゃ",
        "大人しい",
        "拾う",
        "計画",
        "止める",
        "健康",
        "やめる",
        "遅刻",
        "漫画",
        "後悔",
        "上司",
        "めちゃくちゃ",
        "靴下",
        "紙",
        "方向",
        "磨く",
        "床",
        "夕方",
        "煙",
        "釣り",
        "趣味",
        "集める",
        "瞬間",
        "試す",
        "結ぶ",
        "狭い",
        "増やす",
        "学ぶ",
        "目",
        "そして",
        "心",
        "同じ",
        "遠い",
        "姿",
        "相手",
        "先",
        "胸",
        "呼ぶ",
        "水",
        "見せる",
        "娘",
        "神",
        "得る",
        "存在",
        "確か",
        "消える",
        "最初",
        "きっと",
        "場合",
        "感じ",
        "心配",
        "離れる",
        "思い出す",
        "点",
        "理由",
        "押す",
        "男の子",
        "想像",
        "近い",
        "女性",
        "事実",
        "寝る",
        "理解",
        "進む",
        "状態",
        "かしら",
        "完全",
        "当然",
        "向こう",
        "残す",
        "普通",
        "星",
        "守る",
        "命",
        "宇宙",
        "変える",
        "重い",
        "まさか",
        "落とす",
        "例",
        "方法",
        "状況",
        "自由",
        "決める",
        "結局",
        "用意",
        "秘密",
        "報告",
        "匂い",
        "調子",
        "緊張",
        "安心",
        "他人",
        "一体",
        "大変",
        "夏",
        "結婚",
        "興味",
        "事情",
        "少ない",
        "甘い",
        "元気",
        "作戦",
        "教師",
        "期待",
        "喜ぶ",
        "相談",
        "落ち着く",
        "正直",
        "味",
        "自信",
        "覚悟",
        "結構",
        "駄目",
        "邪魔",
        "代わり",
        "本気",
        "意外",
        "失礼",
        "証拠",
        "特別",
        "大切",
        "十分",
        "立派",
        "人生",
        "残念",
        "焼く",
        "本人",
        "野郎",
        "我慢",
        "責任",
        "全て",
        "茶",
        "多分",
        "弱い",
        "偶然",
        "願う",
        "活動",
        "親父",
        "別れる",
        "出会う",
        "付ける",
        "未来",
        "一応",
        "協力",
        "一生",
        "最高",
        "同士",
        "感謝",
        "紹介",
        "女子",
        "せめて",
        "平気",
        "事故",
        "足りる",
        "勝負",
        "恋",
        "真面目",
        "掃除",
        "似合う",
        "余計",
        "申し訳",
        "歌う",
        "無い",
        "良い",
        "お前",
        "訳",
        "やっぱり",
        "あいつ",
        "まま",
        "馬鹿",
        "嫌",
        "ただ",
        "はず",
        "達",
        "つもり",
        "さっき",
        "全く",
        "実",
        "なぜ",
        "しかし",
        "祖母",
        "ほど",
        "さすが",
        "こいつ",
        "あまり",
        "おはよう",
        "者",
        "まじ",
        "まず",
        "とにかく",
        "祖父",
        "そちら",
        "お願い",
        "付き合う",
        "今まで",
        "どっち",
        "下",
        "など",
        "しっかり",
        "辛い",
        "せっかく",
        "しかも",
        "円",
        "つまり",
        "形",
        "いくら",
        "無し",
        "とりあえず",
        "仕方ない",
        "諦める",
        "ただいま",
        "謝る",
        "お兄ちゃん",
        "もしもし",
        "感じる",
        "あっち",
        "無駄",
        "何度",
        "件",
        "ちょうど",
        "随分",
        "やばい",
        "第",
        "楽しみ",
        "側",
        "ばれる",
        "番",
        "お腹",
        "わざわざ",
        "まずい",
        "伯父",
        "いい加減",
        "おめでとう",
        "身",
        "くそ",
        "いろんな",
        "可能性",
        "互い",
        "臭い",
        "愛する",
        "たまに",
        "さっさと",
        "今更",
        "知り合い",
        "つい",
        "大した",
        "偉い",
        "一言",
        "相変わらず",
        "ほとんど",
        "よろしい",
        "工事",
        "ばばあ",
        "どちら",
        "増える",
        "そもそも",
        "逆",
        "裏",
        "不安",
        "既に",
        "そいつ",
        "深い",
        "限り",
        "珍しい",
        "傷つく",
        "納得",
        "懸命",
        "まあまあ",
        "慣れる",
        "可哀想",
        "現れる",
        "元々",
        "直す",
        "勘違い",
        "辺",
        "早速",
        "場",
        "年間",
        "素晴らしい",
        "正しい",
        "運ぶ",
        "求める",
        "苦しい",
        "今後",
        "休み",
        "羨ましい",
        "遠慮",
        "以来",
        "受け取る",
        "最悪",
        "意識",
        "越える",
        "正面",
        "完璧",
        "普段",
        "誤解",
        "下手",
        "たとえ",
        "固い",
        "きつい",
        "空気",
        "具合",
        "違い",
        "お姉ちゃん",
        "ちなみに",
        "思い出",
        "内容",
        "なめる",
        "昼",
        "去年",
        "温かい",
        "きっかけ",
        "手前",
        "バイト",
        "年生",
        "泳ぐ",
        "着替える",
        "男子",
        "水着",
        "セリフ",
        "少女",
        "会話",
        "変態",
        "衣装",
        "勝ち",
        "恋愛",
        "お弁当",
        "制服",
        "開始",
        "禁止",
        "体調",
        "告白",
        "どきどき",
        "まし",
        "中学",
        "用事",
        "泊まる",
        "ガキ",
        "全力",
        "指",
        "話し掛ける",
        "反応",
        "気合い",
        "試験",
        "やる気",
        "勇気",
        "穴",
        "ありがたい",
        "秒",
        "廊下",
        "光",
        "まさに",
        "初",
        "終了",
        "避ける",
        "攻撃",
        "背中",
        "友人",
        "性格",
        "広い",
        "手伝い",
        "遊園地",
        "待ち合わせ",
        "郵便局",
        "引っ越す",
        "教科書",
        "文房具",
        "天井",
        "投げる",
        "飽きる",
        "完了",
        "過去",
        "触れる",
        "微妙",
        "詳しい",
        "消す",
        "まとめる",
        "比べる",
        "証明",
        "満足",
        "才能",
        "永遠",
        "心臓",
        "勝利",
        "季節",
        "値段",
        "預ける",
        "与える",
        "慌てる",
    ]
}

#[derive(Debug, Serialize, Deserialize)]
struct SubData {
    files: Vec<String>,
}

// fn find_serialized_data() -> Option<PathBuf> {
// }

#[derive(Debug, Clone)]
struct Sentence {
    words: Vec<Word>,
}

#[derive(Debug, Clone)]
struct Word {
    unit: String,
    _0: String,
    _1: String,
    _2: String,
    _3: String,
    _4: String,
    _5: String,
    _6: String,
    _7: String,
    _8: String,
    _9: String,
    _10: String,
    _11: String,
    _12: String,
    _13: String,
    _14: String,
    _15: String,
    _16: String,
}

#[tokio::main]
async fn maine() -> Result<()> {
    color_eyre::install();

    let client = AnkiClient::default();

    let deck_info = client.request(DeckNamesAndIdsRequest {}).await?;
    let card_info = client
        .request(FindCardsRequest {
            query: "deck:\"Refold JP1K v2\"".into(),
        })
        .await?;
    let card_data = client
        .request(CardsInfoRequest { cards: card_info })
        .await?;

    let info: Vec<String> = card_data
        .iter()
        .map(|it| it.fields.get("Word").unwrap().value.clone())
        .collect();

    println!("{:#?}", info);

    //         println!("{:?}", deck_info);
    //  println!("{:?}", card_info);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // let paths = get_resource_paths();

    let path = PathBuf::from(
        "The.Gentle.12.1991.1080p.HDTV.AAC1.0.H265.10bit-dougal_JP.srt",
    );
    let paths = vec![path];

    paths
        .par_iter()
        .map(|path| {
            let mut format = get_subtitle_file(path.clone());

            if let Err(_) = format {
                return ();
            }

            match format.as_mut().unwrap() {
                SubtitleFile::SubRipFile(file) => {
                    let parsed_content = file.parse();

                    if let Ok(_) = parsed_content {
                        process_lines(&file.lines);
                    }
                },
                _ => {},
            }

            /*
            if let Some(ext) = path.extension() {
                let strext = ext.to_str().unwrap().to_string();

                map.insert(strext.clone(), map.get(&strext).unwrap_or(&0) + 1);
            } else {
                map.insert("no_ext".into(), map.get("no_ext".into()).unwrap_or(&0) + 1);
            }

            data.files.push(path.as_path().to_str().unwrap().to_string());
             */
        })
        .collect::<Vec<()>>();

    /*
    let serialized = serde_json::to_string(&data);

    if let Ok(jsonData) = serialized {
        fs::write("./data.json", jsonData);
        println!("{:#?}", data);
    } else {
        println!("ERROR");
    }
    */

    Ok(())
}

fn get_resource_paths() -> Vec<PathBuf> {
    recurse("../../../_/SUBRIPS")
}

fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
    let Ok(entries) = read_dir(path) else {
        return vec![];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return recurse(entry.path());
            }
            if meta.is_file() {
                return vec![entry.path()];
            }
            vec![]
        })
        .collect()
}
*/

/*
pub fn process(text: &str) {
    let tagger = Tagger::new("-Ounidic --dicdir=./_/unidic-csj");
    // Tagger::new("-Ounidic
    // --dicdir=./_/unidic-cwj-202302");

    let result = tagger.parse_str(text);

    for result_line in result.split("\n").into_iter() {
        println!("{:?}", result_line);
    }
}
*/
/*


fn process_lines(lines: &Vec<SrtLine>) {
    // let path =
    // PathBuf::from_str("./_/SUBRIPS/
    // JP-Subtitles/女の中にいる他人/女の中にいる他人＃07.srt")?; let mut format
    // = get_subtitle_file(path);
    let tagger =
        Tagger::new("-Ounidic --dicdir=./_/unidic-csj");

    let mut sentences: Vec<Sentence> = vec![];

    for line in lines {
        let mut sentence = Sentence { words: vec![] };

        for text in &line.text {
            let input = &text[..];
            // println!("INPUT: {}", input);

            // gets tagged result as String
            let result = tagger.parse_str(input);

            for result_line in result.split("\n").into_iter() {
                if result_line == "UNK" || result_line == "EOS" || result_line == "BOS" {
                    continue;
                }

                let mut items = result_line.split(",");

                let word = Word {
                    unit: items.nth(0).unwrap_or("").to_string(),
                    _0: items.nth(0).unwrap_or("").to_string(),
                    _1: items.nth(0).unwrap_or("").to_string(),
                    _2: items.nth(0).unwrap_or("").to_string(),
                    _3: items.nth(0).unwrap_or("").to_string(),
                    _4: items.nth(0).unwrap_or("").to_string(),
                    _5: items.nth(0).unwrap_or("").to_string(),
                    _6: items.nth(0).unwrap_or("").to_string(),
                    _7: items.nth(0).unwrap_or("").to_string(),
                    _8: items.nth(0).unwrap_or("").to_string(),
                    _9: items.nth(0).unwrap_or("").to_string(),
                    _10: items.nth(0).unwrap_or("").to_string(),
                    _11: items.nth(0).unwrap_or("").to_string(),
                    _12: items.nth(0).unwrap_or("").to_string(),
                    _13: items.nth(0).unwrap_or("").to_string(),
                    _14: items.nth(0).unwrap_or("").to_string(),
                    _15: items.nth(0).unwrap_or("").to_string(),
                    _16: items.nth(0).unwrap_or("").to_string(),
                };

                //  GOOD 連体詞
                // FIGURE OUT 接尾辞 -
                if &word._0 == "補助記号" // Supplementary symbol
                    || &word.unit == ""
                    || &word._0 == "助詞" // Particle
                    || &word._0 == "感動詞"
                    || &word._0 == "助動詞"
                    || &word._0 == "接続詞"
                // Interjection
                {
                    continue;
                }

                // TODO: We should create "two" words for the prefix-word & word along?
                if (&word._0 == "接頭辞") {
                    continue;
                }

                if (&word._0 == "") {
                    println!("{:?}", &word.unit)
                }

                sentence.words.push(word);
            }

            /*
             * node-format-unidic = %m\t%f[9]\t%f[6]\t%f[7]\t%F-[0,1,2,3]\t%f[4]\t%f[5]\t%f[13]\n
             * # $0: pos	品詞 - Part of speech
             * # $1: pos1      - 品詞大分類 Part of speech (major
             * classification) # $2: pos2      - 品詞中分類 Part
             * of speech (divided) # $3: pos3      - 品詞小分類
             * Part of speech (minor) # $4: pos4      -
             * 品詞細分類 ? # $5: cType     - 活用型
             * Conjugation type # $6: cForm     - 活用形
             * COnjugation form # $7: lForm     - 語彙素読み
             * Lexeme reading # $8: lemma     -
             * 語彙素（＋語彙素細分類） # $9: orth      -
             * 書字形出現形 - Written form # $10: pron     -
             * 発音形出現形 # $11: orthBase - 書字形基本形
             * # $12: pronBase - 発音形基本形
             * # $13: goshu    - 語種
             * # $14: iType    - 語頭変化化型
             * # $15: iForm    - 語頭変化形
             * # $16: fType    - 語末変化化型
             * # $17: fForm    - 語末変化形
             */
            /*
                        https://www.dampfkraft.com/nlp/japanese-tokenizer-dictionaries.html

                        mecab -Ounidic --dicdir=./_/unidic-cs
                        https://clrd.ninjal.ac.jp/unidic/faq.html
                        https://github.com/jordwest/mecab-docs-en

                        https://taku910.github.io/mecab/#format
                        https://taku910.github.io/mecab/format.html

                        表層形\t品詞,品詞細分類1,品詞細分類2,品詞細分類3,活用形,活用型,原形,読み,発音

            or in English:

            Original Form\t
            Part of Speech,
            Part of Speech section 1,
            Part of Speech section 2,
            Part of Speech section 3,
            Conjugated form,
            Inflection,
            Reading,
            Pronounciation
            */
        }

        if sentence.words.len() != 0 {
            sentences.push(sentence);
        }
    }

    /*
    let target_word = "呼ぶ";

    sentences.iter().find(|sentence| {
        sentence.words.iter().find(|word| {
            if word._7 == target_word {
                println!("{:?}", word);
            }
            false
        });
        false
    });
    */

    let mut pos = HashMap::new();
    let mut seen = HashSet::new();
    let mut unique = HashSet::new();
    let mut wordc: HashMap<String, usize> = HashMap::new();

    for sentence in sentences.clone() {
        println!("{{");
        // println!("{:?}", sentence);
        for word in sentence.words {
            println!("{} {} {}", word._7, word.unit, word._0);

            let actual = word._7.clone();
            let exist = wordc.get(&actual).unwrap_or(&0_usize);
            wordc.insert(actual, exist + 1);

            if seen.contains(&word._7).not() {
                seen.insert(word._7.clone());
                let item = pos.get(&word._0);
                pos.insert(word._0, item.unwrap_or(&0) + 1);
                unique.insert(word._7);
            } else {
                unique.remove(&word._7);
            }
        }
        println!("}}");
    }

    // println!("{:#?}", sentences);
    // println!("{:?}", sentences.len());
    // println!("{:#?}", pos);
    // println!("{:?}", unique.len());

    let known = known_words();

    let x: Vec<&&str> = known
        .iter()
        .filter(|it| seen.contains(&it.to_string()))
        .collect();
    let y: Vec<&String> = seen
        .iter()
        .filter(|it| known.contains(&&(*it)[..]))
        .collect();

    println!(
        "{:?}/{:?} {:?}/{:?}",
        x.len(),
        known.len(),
        y.len(),
        seen.len()
    );

    let mut x = wordc
        .iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect::<Vec<(usize, String)>>();
    x.sort();

    // println!("{:#?}", x);
    // println!("{:?}", x.len());
    let mut count_map: HashMap<usize, Vec<String>> = HashMap::new();

    for (occurances, word) in x {
        let curr = count_map.get_mut(&occurances);
        if let Some(vec) = curr {
            vec.push(word);
        } else {
            count_map.insert(occurances, vec![word]);
        }
    }

    let mut count_map = count_map
        .iter()
        .map(|(k, v)| (*k, v.clone()))
        .collect::<Vec<(usize, Vec<String>)>>();
    count_map.sort();
    count_map.reverse();

    // println!("{:#?}", count_map);
}

// 代名詞
// 動詞
*/
