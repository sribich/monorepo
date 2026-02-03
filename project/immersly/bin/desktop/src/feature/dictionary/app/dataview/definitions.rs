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
