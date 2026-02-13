use std::marker::PhantomData;
use std::sync::Arc;

use forvo::action::word_pronunciations::WordPronunciation;
use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::infra::database::Sqlite;
use storage::domain::value::ResourceId;

use crate::domain::entity::pronunciation::Pronunciation;
use crate::domain::entity::pronunciation::PronunciationData;
use crate::domain::value::PronunciationId;
use crate::domain::value::sex::Sex;

#[derive(Component)]
pub struct PronunciationRepository {
    db: Arc<Sqlite>,
}

impl PronunciationRepository {
    fn model(&self) -> model::pronunciation::Actions {
        self.db.client().pronunciation()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl PronunciationRepository {
    pub async fn by_id(&self, id: &PronunciationId) -> Result<Option<Pronunciation>, QueryError> {
        self.model()
            .find_unique(model::pronunciation::id::equals(id.to_vec()))
            .exec()
            .await
            .map(Convert::into)
    }

    pub async fn count_words(&self, word: String) -> Result<i64, QueryError> {
        self.model()
            .count(vec![model::pronunciation::word::equals(word)])
            .exec()
            .await
    }

    pub async fn for_word(&self, word: String) -> Result<Vec<Pronunciation>, QueryError> {
        self.model()
            .find_many(vec![model::pronunciation::word::equals(word)])
            .exec()
            .await
            .map(Convert::into)
    }
}

//==============================================================================
// Writer
//==============================================================================
impl PronunciationRepository {
    pub async fn create(
        &self,
        pronunciation: &WordPronunciation,
        resource_id: &ResourceId,
    ) -> Result<Pronunciation, QueryError> {
        let params = model::pronunciation::Create {
            id: PronunciationId::new_now().as_bytes().to_vec(),
            word: pronunciation.word.clone(),
            name: pronunciation.username.clone(),
            sex: pronunciation.sex.clone(),
            language: pronunciation.langname.clone(),
            resource: model::resource::id::equals(resource_id.to_vec()),
            params: vec![],
        };

        params
            .to_query(self.db.client())
            .exec()
            .await
            .map(Convert::into)
    }
}

trait Convert<T> {
    #[must_use]
    fn into(self) -> T;
}

impl Convert<Pronunciation> for model::pronunciation::Data {
    fn into(self) -> Pronunciation {
        Pronunciation::from_data(PronunciationData {
            id: PronunciationId::from_slice_unchecked(&self.id),
            word: self.word,
            name: self.name,
            sex: Sex::from_str(&self.sex),
            language: self.language,
            resource_id: ResourceId::from_slice_unchecked(&self.resource_id),
            attribution: None,
        })
    }
}

impl Convert<Option<Pronunciation>> for Option<model::pronunciation::Data> {
    fn into(self) -> Option<Pronunciation> {
        self.map(Convert::into)
    }
}

impl Convert<Vec<Pronunciation>> for Vec<model::pronunciation::Data> {
    fn into(self) -> Vec<Pronunciation> {
        self.into_iter().map(Convert::into).collect::<Vec<_>>()
    }
}
