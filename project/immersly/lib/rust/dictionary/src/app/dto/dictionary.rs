use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

use crate::domain::entity::dictionary::Dictionary;

#[derive(Deserialize, Serialize, Typegen)]
#[serde(rename = "Dictionary", rename_all = "camelCase")]
pub struct DictionaryDto {}

impl From<Dictionary> for DictionaryDto {
    fn from(value: Dictionary) -> Self {
        Self {}
    }
}
