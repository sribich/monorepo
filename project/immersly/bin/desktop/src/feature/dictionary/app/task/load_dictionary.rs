use std::sync::Arc;

use dictionary_parser::load_dictionary;
use features::shared::domain::value::muid::Muid;
use railgun_di::Component;

use crate::domain::value::existing_path::ExistingPath;
use crate::feature::dictionary::app::service::dictionary::DictionaryService;
use crate::feature::dictionary::domain::aggregate::dictionary::Dictionary;
use crate::feature::dictionary::domain::entity::definition::Definition;
use crate::feature::dictionary::domain::value::language_type::LanguageType;
use crate::feature::dictionary::infra::repository::dictionary::DictionaryRepository;
use crate::system::actor::Actor;
use crate::system::actor::Task;

#[derive(Clone, Component)]
pub struct LoadDictionaryTask {
    actor: Arc<Actor>,
    dictionary_repository: Arc<DictionaryRepository>,
    dictionary_service: Arc<DictionaryService>,
}

struct LoadDictionaryWrapper {
    task: LoadDictionaryTask,
    payload: LoadDictionaryPayload,
}

struct LoadDictionaryPayload {
    path: ExistingPath,
    language_type: LanguageType,
}

impl LoadDictionaryTask {
    pub async fn send(&self, path: ExistingPath, language_type: LanguageType) {
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
        let muid = Muid::new_now();

        let file_path = self.payload.path.clone();
        let data_path = self.task.dictionary_service.get_data_path(&muid).await;

        let data = load_dictionary(&self.payload.path.as_path(), &data_path).unwrap();

        let info = data.info();
        let mut dictionary = Dictionary::new(
            Some(muid),
            info.title,
            self.payload.language_type.clone(),
            file_path.as_str().to_owned(),
            data_path.to_string_lossy().to_string(),
        );

        let words = data.words(Some(format!(
            "/rpc/dictionary_image/{}/",
            dictionary.id().to_string()
        )));

        let accents = data.accents();
        let frequencies = data.frequencies();

        let words = words
            .into_iter()
            .map(|word| Definition::new(word.word, word.reading, word.definition))
            .collect::<Vec<_>>();

        dictionary.add_definitions(words);
        dictionary.add_accents(accents);
        dictionary.add_frequencies(frequencies);

        self.task
            .dictionary_repository
            .writer()
            .save(dictionary)
            .await;

        ();
    }
}
