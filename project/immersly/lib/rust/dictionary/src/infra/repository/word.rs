use std::marker::PhantomData;
use std::sync::Arc;

use dictionary_parser::DictionaryWord;
use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::infra::database::Sqlite;

use crate::domain::value::dictionary_id::DictionaryId;
use crate::domain::value::pitch_accent_id::PitchAccentId;

#[derive(Component)]
pub struct WordRepository {
    db: Arc<Sqlite>,
}

//==============================================================================
// Util
//==============================================================================
impl WordRepository {
    pub fn model(&self) -> model::word::Actions {
        self.db.client().word()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl WordRepository {}

//==============================================================================
// Writer
//==============================================================================
impl WordRepository {
    pub async fn create_many(
        &self,
        client: &PrismaClient,
        dictionary_id: &DictionaryId,
        accents: Vec<DictionaryWord>,
    ) -> Result<(), QueryError> {
        for chunk in accents.into_chunks::<10000>() {
            let params = chunk
                .into_iter()
                .map(|data| model::word::CreateUnchecked {
                    id: PitchAccentId::new_now().to_vec(),
                    word: data.word,
                    reading: data.reading,
                    definition: data.definition,
                    dictionary_id: dictionary_id.to_vec(),
                    params: vec![],
                })
                .collect::<Vec<_>>();

            client.word().create_many(params).exec().await?;
        }

        Ok(())
    }
}

//==============================================================================
// Transformer
//==============================================================================
trait Convert<T> {
    #[must_use]
    fn into(self) -> T;
}
