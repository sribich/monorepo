use std::collections::HashSet;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

use dictionary_parser::Accent;
use dictionary_parser::Frequency;
use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model;
use prisma_client::model::SortOrder;
use railgun::di::Component;
use shared::infra::database::Sqlite;

use crate::domain::entity::dictionary::Dictionary;
use crate::domain::entity::dictionary::DictionaryData;
use crate::domain::value::dictionary_id::DictionaryId;
use crate::domain::value::language_type::LanguageType;
use crate::infra::ranker::dictionary::DictionaryRanker;

#[derive(Component)]
pub struct DictionaryRepository {
    db: Arc<Sqlite>,
    ranker: Arc<DictionaryRanker>,
}

//==============================================================================
//
//==============================================================================
impl DictionaryRepository {
    pub fn model(&self) -> model::dictionary::Actions {
        self.db.client().dictionary()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl DictionaryRepository {
    pub async fn list(&self) -> Result<Vec<Dictionary>, QueryError> {
        self.model()
            .find_many(vec![])
            .exec()
            .await
            .map(Convert::into)
    }

    pub async fn first_rank(&self) -> Result<Option<String>, QueryError> {
        self.model()
            .find_first(vec![])
            .order_by(model::dictionary::rank::order(SortOrder::Asc))
            .exec()
            .await
            .map(|it| it.map(|it| it.rank))
    }

    pub async fn last_rank(&self) -> Result<Option<String>, QueryError> {
        self.model()
            .find_first(vec![])
            .order_by(model::dictionary::rank::order(SortOrder::Desc))
            .exec()
            .await
            .map(|it| it.map(|it| it.rank))
    }
}

//==============================================================================
// Writer
//==============================================================================
impl DictionaryRepository {
    pub async fn create(&self,
        client: &PrismaClient,
        dictionary_id: &DictionaryId,
        file_path: &Path,
        data_path: &Path,
        language_type: String,
        dictionary: &Arc<dyn dictionary_parser::Dictionary>
    ) -> Result<(), QueryError> {
        let info = dictionary.info();

        let params = model::dictionary::Create {
            id: dictionary_id.to_vec(),
            title: info.title,
            language_type,
            kinds: "".to_owned(),
            file_path: file_path.to_str().unwrap().to_owned(),
            data_path: data_path.to_str().unwrap().to_owned(),
            rank: self.ranker.next().await,
            params: vec![],
        };

        params.to_query(client).exec().await?;

        Ok(())
    }
}

//==============================================================================
//
//==============================================================================
trait Convert<T> {
    #[must_use]
    fn into(self) -> T;
}

impl Convert<Dictionary> for model::dictionary::Data {
    fn into(self) -> Dictionary {
        DictionaryData {
            id: DictionaryId::from_slice_unchecked(&self.id),
            title: self.title,
            language_type: LanguageType::from_str(self.language_type).unwrap(),
            kinds: HashSet::default(), // TODO(sr): self.kinds,
            file_path: self.file_path,
            data_path: self.data_path,
        }.into()
    }
}

impl Convert<Option<Dictionary>> for Option<model::dictionary::Data> {
    fn into(self) -> Option<Dictionary> {
        self.map(Convert::into)
    }
}

impl Convert<Vec<Dictionary>> for Vec<model::dictionary::Data> {
    fn into(self) -> Vec<Dictionary> {
        self.into_iter().map(Convert::into).collect::<Vec<_>>()
    }
}

/*
impl Convert<Card> for model::card::Data {
    fn into(self) -> Card {
        Card::from_data(CardData {
            id: CardId::from_slice_unchecked(&self.id),
            state: CardState::from_str(&self.state, self.step).unwrap(),
            word: self.word,
            reading: self.reading,
            reading_audio: self
                .reading_audio_id
                .map(|it| ResourceId::from_slice_unchecked(&it)),
            sentence: self.sentence,
            sentence_audio: self
                .sentence_audio_id
                .map(|it| ResourceId::from_slice_unchecked(&it)),
            image_id: self
                .image_id
                .map(|it| ResourceId::from_slice_unchecked(&it)),
            due: self.due as u64,
            last_review: self.last_review.map(|it| it as u64),
            stability: self.stability,
            difficulty: self.difficulty,
        })
    }
}

impl Convert<Option<Card>> for Option<model::card::Data> {
    fn into(self) -> Option<Card> {
        self.map(Convert::into)
    }
}

impl Convert<Vec<Card>> for Vec<model::card::Data> {
    fn into(self) -> Vec<Card> {
        self.into_iter().map(Convert::into).collect::<Vec<_>>()
    }
}
*/

/*
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
*/
