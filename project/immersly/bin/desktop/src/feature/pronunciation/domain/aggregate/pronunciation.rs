use features::shared::domain::value::muid::Muid;

use crate::feature::pronunciation::domain::cdc::pronunciation::PronunciationChangeCapture;
use crate::feature::pronunciation::domain::value::attribution::Attribution;
use crate::feature::pronunciation::domain::value::sex::Sex;

#[derive(Clone)]
pub struct Pronunciation {
    id: Muid,
    word: String,
    name: String,
    sex: Sex,
    language: String,
    path: String,
    attribution: Option<Attribution>,

    _changes: Vec<PronunciationChangeCapture>,
}

impl Pronunciation {
    pub fn new(
        id: Muid,
        word: String,
        name: String,
        sex: Sex,
        language: String,
        path: String,
    ) -> Self {
        let mut this = Self {
            id,
            word,
            name,
            sex,
            language,
            path,
            attribution: None,
            _changes: vec![],
        };

        this._changes
            .push(PronunciationChangeCapture::Created(this.clone()));

        this
    }

    pub fn id(&self) -> &Muid {
        &self.id
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sex(&self) -> &Sex {
        &self.sex
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn changes(&mut self) -> Vec<PronunciationChangeCapture> {
        std::mem::take(&mut self._changes)
    }
}
