use std::marker::PhantomData;
use std::sync::Arc;

use dictionary_parser::Accent;
use dictionary_parser::Frequency;
use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::infra::database::Sqlite;

use crate::domain::value::dictionary_id::DictionaryId;
use crate::domain::value::frequency_id::FrequencyId;

#[derive(Component)]
pub struct FrequencyRepository {
    db: Arc<Sqlite>,
}

//==============================================================================
// Util
//==============================================================================
impl FrequencyRepository {
    pub fn model(&self) -> model::frequency::Actions {
        self.db.client().frequency()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl FrequencyRepository {}

//==============================================================================
// Writer
//==============================================================================
impl FrequencyRepository {
    pub async fn create_many(
        &self,
        client: &PrismaClient,
        dictionary_id: &DictionaryId,
        frequencies: Vec<Frequency>,
    ) -> Result<(), QueryError> {
        for chunk in frequencies.into_chunks::<10000>() {
            let params = chunk
                .into_iter()
                .map(|data| model::frequency::CreateUnchecked {
                    id: FrequencyId::new_now().to_vec(),
                    word: data.word,
                    reading: data.reading,
                    frequency: data.frequency as i32,
                    dictionary_id: dictionary_id.to_vec(),
                    params: vec![model::frequency::display::set(data.display)],
                })
                .collect::<Vec<_>>();

            client.frequency().create_many(params).exec().await?;
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
