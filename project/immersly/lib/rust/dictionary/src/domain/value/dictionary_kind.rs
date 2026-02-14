#[derive(Clone, Hash, Eq, PartialEq)]
pub enum DictionaryKind {
    Word,
    Kanji,
    Name,
    PitchAccent,
    Frequency,
    Grammar,
}

impl ToString for DictionaryKind {
    fn to_string(&self) -> String {
        match self {
            DictionaryKind::Word => "word",
            DictionaryKind::Kanji => "kanji",
            DictionaryKind::Name => "name",
            DictionaryKind::PitchAccent => "accent",
            DictionaryKind::Frequency => "frequency",
            DictionaryKind::Grammar => "grammar",
        }
        .to_owned()
    }
}
