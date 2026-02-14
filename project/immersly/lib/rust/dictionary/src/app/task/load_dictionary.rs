use std::sync::Arc;

use dictionary_parser::load_dictionary;
use prisma_client::QueryError;
use railgun::di::Component;
use shared::domain::value::existing_file::ExistingFile;
use shared::infra::actor::Actor;
use shared::infra::actor::Task;
use shared::infra::database::Sqlite;

use crate::app::service::dictionary::DictionaryService;
use crate::domain::value::dictionary_id::DictionaryId;
use crate::domain::value::language_type::LanguageType;
use crate::infra::repository::dictionary::DictionaryRepository;
use crate::infra::repository::frequency::FrequencyRepository;
use crate::infra::repository::pitch_accent::PitchAccentRepository;
use crate::infra::repository::word::WordRepository;

#[derive(Clone, Component)]
pub struct LoadDictionaryTask {
    actor: Arc<Actor>,
    db: Arc<Sqlite>,
    dictionary_repository: Arc<DictionaryRepository>,
    dictionary_service: Arc<DictionaryService>,
    frequency_repository: Arc<FrequencyRepository>,
    pitch_accent_repository: Arc<PitchAccentRepository>,
    word_repository: Arc<WordRepository>,
}

struct LoadDictionaryWrapper {
    task: LoadDictionaryTask,
    payload: LoadDictionaryPayload,
}

struct LoadDictionaryPayload {
    path: ExistingFile,
    language_type: LanguageType,
}

impl LoadDictionaryTask {
    pub async fn send(&self, path: ExistingFile, language_type: LanguageType) {
        self.actor
            .send(LoadDictionaryWrapper {
                task: self.clone(),
                payload: LoadDictionaryPayload {
                    path,
                    language_type,
                },
            })
            .await
            .expect("TODO");
    }
}

impl Task<()> for LoadDictionaryWrapper {
    async fn execute(&self) {
        let dictionary_id = DictionaryId::new_now();

        let file_path = self.payload.path.clone();
        let data_path = self
            .task
            .dictionary_service
            .get_data_path(&dictionary_id)
            .await;

        let dictionary = load_dictionary(&self.payload.path.as_path(), &data_path).unwrap();

        let transaction = self.task.db.client()._transaction();

        transaction
            .run(|client| async move {
                let accents = dictionary.accents();
                let frequencies = dictionary.frequencies();
                let words = dictionary.words(Some(format!(
                    "/rpc/dictionary_image/{}/",
                    dictionary_id.to_string()
                )));

                self.task
                    .dictionary_repository
                    .create(
                        &client,
                        &dictionary_id,
                        file_path.as_path(),
                        &data_path,
                        self.payload.language_type.to_str().to_owned(),
                        &dictionary,
                    )
                    .await?;
                self.task
                    .pitch_accent_repository
                    .create_many(&client, &dictionary_id, accents)
                    .await?;
                self.task
                    .frequency_repository
                    .create_many(&client, &dictionary_id, frequencies)
                    .await?;
                self.task
                    .word_repository
                    .create_many(&client, &dictionary_id, words)
                    .await?;

                Ok(()) as Result<(), QueryError>
            })
            .await
            .unwrap();
    }
}
