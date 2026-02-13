use storage::domain::entity::resource::Resource;
use storage::domain::value::ResourceId;

use crate::domain::value::PronunciationId;
use crate::domain::value::attribution::Attribution;
use crate::domain::value::sex::Sex;

pub struct PronunciationData {
    pub id: PronunciationId,
    pub word: String,
    pub name: String,
    pub sex: Sex,
    pub language: String,
    pub resource_id: ResourceId,
    pub attribution: Option<Attribution>,
}

pub struct Pronunciation {
    id: PronunciationId,
    word: String,
    name: String,
    sex: Sex,
    language: String,
    resource_id: ResourceId,
    attribution: Option<Attribution>,
}

impl Pronunciation {
    pub fn from_data(data: PronunciationData) -> Self {
        Self {
            id: data.id,
            word: data.word,
            name: data.name,
            sex: data.sex,
            language: data.language,
            resource_id: data.resource_id,
            attribution: data.attribution,
        }
    }

    pub fn id(&self) -> &PronunciationId {
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

    pub fn resource_id(&self) -> &ResourceId {
        &self.resource_id
    }
}
