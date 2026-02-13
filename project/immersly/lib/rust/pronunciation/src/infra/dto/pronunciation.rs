use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

use crate::domain::entity::pronunciation::Pronunciation;

#[derive(Deserialize, Serialize, Typegen)]
#[serde(rename = "Pronunciation", rename_all = "camelCase")]
pub struct PronunciationDto {
    id: String,
    speaker: String,
    resource_id: String,
}

impl From<Pronunciation> for PronunciationDto {
    fn from(value: Pronunciation) -> Self {
        Self {
            id: value.id().to_string(),
            speaker: value.name().to_owned(),
            resource_id: value.resource_id().to_string(),
        }
    }
}
