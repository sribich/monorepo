use features::shared::domain::value::muid::Muid;

#[derive(Clone)]
pub struct Definition {
    id: Muid,
    word: String,
    reading: String,
    definition: String,
}

impl Definition {
    pub fn new(word: String, reading: String, definition: String) -> Self {
        Self {
            id: Muid::new_now(),
            word,
            reading,
            definition,
        }
    }

    pub fn id(&self) -> &Muid {
        &self.id
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn reading(&self) -> &str {
        &self.reading
    }

    pub fn definition(&self) -> &str {
        &self.definition
    }

    //     pub fn info(&self) -> &DictionaryInfo {
    //         &self.info
    //     }
    //
    //     pub fn info_owned(self) -> DictionaryInfo {
    //         self.info
    //     }
}
