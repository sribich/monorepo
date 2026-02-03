use std::marker::PhantomData;
use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use forvo::action::word_pronunciations::WordPronunciation;
use prisma_client::model;
use railgun_di::Component;

pub struct Reader;
pub struct Writer;

#[derive(Component)]
pub struct PronunciationRepository<T = ()>
where
    T: 'static + Send + Sync,
{
    db: Arc<Sqlite>,
    _marker: PhantomData<T>,
}

impl PronunciationRepository<()> {
    pub fn reader(&self) -> PronunciationRepository<Reader> {
        PronunciationRepository {
            db: Arc::clone(&self.db),
            _marker: Default::default(),
        }
    }

    pub fn writer(&self) -> PronunciationRepository<Writer> {
        PronunciationRepository {
            db: Arc::clone(&self.db),
            _marker: Default::default(),
        }
    }
}

impl PronunciationRepository<Reader> {}

impl PronunciationRepository<Writer> {
    pub async fn create(
        &self,
        pronunciation: &WordPronunciation,
        resource_id: Vec<u8>,
    ) -> model::pronunciation::Data {
        self.db
            .client()
            .pronunciation()
            .create(
                Muid::new_now().as_bytes().to_vec(),
                pronunciation.word.clone(),
                pronunciation.username.clone(),
                pronunciation.sex.clone(),
                pronunciation.langname.clone(),
                model::resource::id::equals(resource_id),
                vec![],
            )
            .exec()
            .await
            .unwrap()
    }
}
