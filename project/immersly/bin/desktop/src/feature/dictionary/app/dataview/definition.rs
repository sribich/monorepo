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
