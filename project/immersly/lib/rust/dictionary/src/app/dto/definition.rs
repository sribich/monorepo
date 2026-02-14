use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Typegen)]
#[serde(rename = "Definition", rename_all = "camelCase")]
pub struct DefinitionDto {}

/*
use features::shared::domain::value::muid::Muid;
use prisma_client::model;
use railgun_di::Component;

#[derive(Debug)]
pub struct Definition {
    pub dictionary_id: Muid,
    pub word: String,
    pub reading: String,
    pub definition: String,
}

#[derive(Component)]
pub struct DefinitionDataView {}

impl DefinitionDataView {
    pub fn from_data(data: &model::word::Data) -> Definition {
        Definition {
            dictionary_id: Muid::from_slice_unchecked(&data.dictionary_id),
            word: data.word.clone(),
            reading: data.reading.clone(),
            definition: data.definition.clone(),
        }
    }
}

use std::sync::Arc;

use prisma_client::model;
use railgun_di::Component;

use super::definition::Definition;
use super::definition::DefinitionDataView;
use crate::feature::dictionary::app::service::dictionary_lookup::DictionaryLookupService;

#[derive(Debug)]
pub struct Definitions {
    pub bilingual: Vec<Definition>,
    pub monolingual: Vec<Definition>,
}

#[derive(Component)]
pub struct DefinitionsDataView {
    dictionary_lookup: Arc<DictionaryLookupService>,
}

impl DefinitionsDataView {
    pub async fn from_word(&self, word: String, reading: Option<String>) -> Definitions {
        let data = self
            .dictionary_lookup
            .lookup_word(word, reading)
            .await
            .unwrap();

        Self::from_data(data)
    }

    pub fn from_data(data: Vec<model::word::Data>) -> Definitions {
        let bilingual = data
            .iter()
            .filter(|word| word.dictionary.as_ref().unwrap().language_type == "bi")
            .map(DefinitionDataView::from_data)
            .collect::<Vec<_>>();
        let monolingual = data
            .iter()
            .filter(|word| word.dictionary.as_ref().unwrap().language_type == "mono")
            .map(DefinitionDataView::from_data)
            .collect::<Vec<_>>();

        Definitions {
            bilingual,
            monolingual,
        }
    }
}
*/
