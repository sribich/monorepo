use std::marker::PhantomData;
use std::sync::Arc;

use dictionary_parser::Accent;
use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::infra::database::Sqlite;

use crate::domain::value::dictionary_id::DictionaryId;
use crate::domain::value::pitch_accent_id::PitchAccentId;

#[derive(Component)]
pub struct PitchAccentRepository {
    db: Arc<Sqlite>,
}

//==============================================================================
// Util
//==============================================================================
impl PitchAccentRepository {
    pub fn model(&self) -> model::pitch_accent::Actions {
        self.db.client().pitch_accent()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl PitchAccentRepository {}

//==============================================================================
// Writer
//==============================================================================
impl PitchAccentRepository {
    pub async fn create_many(
        &self,
        client: &PrismaClient,
        dictionary_id: &DictionaryId,
        accents: Vec<Accent>,
    ) -> Result<(), QueryError> {
        for chunk in accents.into_chunks::<10000>() {
            let params = chunk
                .into_iter()
                .map(|data| model::pitch_accent::CreateUnchecked {
                    id: PitchAccentId::new_now().to_vec(),
                    word: data.word,
                    reading: data.reading,
                    position: data.position as i32,
                    dictionary_id: dictionary_id.to_vec(),
                    params: vec![],
                })
                .collect::<Vec<_>>();

            client.pitch_accent().create_many(params).exec().await?;
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
