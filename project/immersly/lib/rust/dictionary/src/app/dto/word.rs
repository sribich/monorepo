use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Typegen)]
#[serde(rename = "Word", rename_all = "camelCase")]
pub struct WordDto {}
