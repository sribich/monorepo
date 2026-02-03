use std::marker::PhantomData;
use std::sync::Arc;

use dictionary_parser::Accent;
use dictionary_parser::Frequency;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model::SortOrder;
use prisma_client::model::{self};
use railgun_di::Component;

use crate::feature::dictionary::domain::aggregate::dictionary::Dictionary;
use crate::feature::dictionary::domain::cdc::dictionary::DictionaryChangeEvent;
use crate::feature::dictionary::domain::cdc::dictionary::DictionaryChangeUniqueEvent;
use crate::feature::dictionary::domain::entity::definition::Definition;
use crate::feature::dictionary::domain::value::dictionary_kind::DictionaryKind;
use crate::feature::dictionary::infra::ranker::dictionary::DictionaryRanker;

pub struct Reader;
pub struct Writer;

#[derive(Component)]
pub struct DictionaryRepository<T = ()>
where
    T: 'static + Send + Sync,
{
    db: Arc<Sqlite>,
    ranker: Arc<DictionaryRanker>,
    _marker: PhantomData<T>,
}

impl DictionaryRepository<()> {
    pub fn reader(&self) -> DictionaryRepository<Reader> {
        DictionaryRepository {
            db: Arc::clone(&self.db),
            ranker: Arc::clone(&self.ranker),
            _marker: Default::default(),
        }
    }

    pub fn writer(&self) -> DictionaryRepository<Writer> {
        DictionaryRepository {
            db: Arc::clone(&self.db),
            ranker: Arc::clone(&self.ranker),
            _marker: Default::default(),
        }
    }
}

impl DictionaryRepository<Reader> {
    pub async fn x(&self, word: String, reading: Option<String>) {
        let result = self
            .db
            .client()
            .word()
            .find_many(vec![
                model::word::word::equals(word.clone()),
                model::word::reading::equals(reading.clone().unwrap_or_default()),
            ])
            .with(model::word::dictionary::fetch())
            .order_by(model::word::dictionary::order(vec![
                model::dictionary::rank::order(SortOrder::Asc),
            ]))
            .exec()
            .await;
    }
}

impl DictionaryRepository<Writer> {
    pub async fn save(&self, mut aggregate: Dictionary) {
        let transaction = self.db.client()._transaction().with_timeout(60000);

        transaction
            .run(|client| async move {
                for event in aggregate.changes() {
                    match event {
                        DictionaryChangeEvent::Created(dictionary) => {
                            self.create_dictionary(&client, &dictionary).await;
                        }
                        DictionaryChangeEvent::AddDefinitions(vec) => {
                            self.add_definitions(&client, &vec, aggregate.id()).await;
                        }
                        DictionaryChangeEvent::AddAccents(vec) => {
                            self.add_accents(&client, &vec, aggregate.id()).await;
                        }
                        DictionaryChangeEvent::AddFrequencies(vec) => {
                            self.add_frequencies(&client, &vec, aggregate.id()).await;
                        }
                    }
                }

                for event in aggregate.unique_changes() {
                    match event {
                        DictionaryChangeUniqueEvent::ChangedKinds => {
                            self.set_kinds(&client, &aggregate.kinds(), aggregate.id())
                                .await;
                        }
                    }
                }

                Ok(()) as core::result::Result<(), QueryError>
            })
            .await
            .unwrap();
    }

    async fn create_dictionary(&self, client: &PrismaClient, dictionary: &Dictionary) {
        client
            .dictionary()
            .create(
                dictionary.id().as_bytes().to_vec(),
                dictionary.title().to_owned(),
                dictionary.language_type().to_str().to_owned(),
                String::new(),
                dictionary.file_path().to_owned(),
                dictionary.data_path().to_owned(),
                self.ranker.next().await,
                vec![],
            )
            .exec()
            .await
            .unwrap();
    }

    async fn add_definitions(
        &self,
        client: &PrismaClient,
        definitions: &[Definition],
        dictionary_id: &Muid,
    ) {
        client
            .word()
            .create_many(
                definitions
                    .iter()
                    .map(|def| {
                        model::word::create_unchecked(
                            def.id().as_bytes().to_vec(),
                            def.word().to_owned(),
                            def.reading().to_owned(),
                            def.definition().to_owned(),
                            dictionary_id.as_bytes().to_vec(),
                            vec![],
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .exec()
            .await
            .unwrap();
    }

    async fn add_accents(&self, client: &PrismaClient, accents: &[Accent], dictionary_id: &Muid) {
        client
            .pitch_accent()
            .create_many(
                accents
                    .iter()
                    .map(|def| {
                        model::pitch_accent::create_unchecked(
                            Muid::new_now().as_bytes().to_vec(),
                            def.word.clone(),
                            def.reading.clone(),
                            def.position as i32,
                            dictionary_id.as_bytes().to_vec(),
                            vec![],
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .exec()
            .await
            .unwrap();
    }

    async fn add_frequencies(
        &self,
        client: &PrismaClient,
        frequencies: &[Frequency],
        dictionary_id: &Muid,
    ) {
        for chunk in frequencies.chunks(100000) {
            client
                .frequency()
                .create_many(
                    chunk
                        .iter()
                        .map(|def| {
                            model::frequency::create_unchecked(
                                Muid::new_now().as_bytes().to_vec(),
                                def.word.clone(),
                                def.reading.clone(),
                                def.frequency as i32,
                                dictionary_id.as_bytes().to_vec(),
                                vec![model::frequency::display::set(def.display.clone())],
                            )
                        })
                        .collect::<Vec<_>>(),
                )
                .exec()
                .await
                .unwrap();
        }
    }

    async fn set_kinds(
        &self,
        client: &PrismaClient,
        kinds: &[DictionaryKind],
        dictionary_id: &Muid,
    ) {
        let data = kinds
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        client
            .dictionary()
            .update(
                model::dictionary::id::equals(dictionary_id.as_bytes().to_vec()),
                vec![model::dictionary::kinds::set(data)],
            )
            .exec()
            .await
            .unwrap();
    }
}
