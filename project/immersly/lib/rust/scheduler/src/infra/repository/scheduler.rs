use std::marker::PhantomData;
use std::sync::Arc;

use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::infra::database::Sqlite;
use storage::domain::value::ResourceId;

use crate::domain::entity::card::Card;
use crate::domain::entity::card::CardData;
use crate::domain::value::card_id::CardId;
use crate::domain::value::card_state::CardState;
use crate::domain::value::next_card_state::NextCardState;

#[derive(Component)]
pub struct SchedulerRepository
{
    db: Arc<Sqlite>,
}

impl SchedulerRepository {
    pub fn model(&self) -> model::card::Actions {
        self.db.client().card()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl SchedulerRepository {
    pub async fn from_id(&self, id: &CardId) -> Result<Option<Card>, QueryError> {
        self.model()
            .find_unique(model::card::id::equals(id.to_vec()))
            .exec()
            .await
            .map(Convert::into)
    }

    pub async fn get_next_due(
        &self,
    ) -> Result<Option<Card>, QueryError> {
        let now = chrono::Local::now();

        self.model()
            .find_first(vec![
                model::card::state::not("new".to_owned()),
                model::card::due::lt(now.timestamp()),
            ])
            .exec()
            .await
            .map(Convert::into)
    }

    pub async fn list_new_cards(&self, count: u32) -> Result<Vec<Card>, QueryError> {
        self.model()
            .find_many(vec![model::card::state::equals("new".to_owned())])
            .take(i64::from(count))
            .exec()
            .await
            .map(Convert::into)
    }
}

//==============================================================================
// Writer
//==============================================================================
impl SchedulerRepository {
    pub async fn set_image(&self, card_id: &CardId, resource_id: &ResourceId) -> Result<Card, QueryError> {
        self.model()
            .update(
                model::card::id::equals(card_id.to_vec()),
                vec![model::card::image_id::set(Some(
                    resource_id.to_vec(),
                ))],
            )
            .exec()
            .await
            .map(Convert::into)
    }





    pub async fn apply_learning_state(&self, card: &Card) {
        let card_data = card.as_data();

        self.model()
            .update(
                model::card::id::equals(card.as_data().id.to_vec()),
                vec![
                    model::card::state::set(card_data.state.to_string()),
                    model::card::step::set(card.step()),
                    model::card::due::set(card_data.due as i64),
                    model::card::last_review::set(card_data.last_review.map(|it| it as i64)),
                    model::card::difficulty::set(card_data.difficulty),
                    model::card::stability::set(card_data.stability),
                ],
            )
            .exec()
            .await
            .unwrap();
    }

    pub async fn create_card(
        &self,
        word: String,
        reading: String,
        sentence: String,
        reading_audio_id: ResourceId,
        sentence_audio_id: ResourceId,
    ) {
        let reading_audio_id = self
            .db
            .client()
            .pronunciation()
            .find_unique(model::pronunciation::id::equals(
                reading_audio_id.as_bytes().to_vec(),
            ))
            .exec()
            .await
            .unwrap()
            .unwrap()
            .resource_id;

        self.db
            .client()
            .card()
            .create(
                CardId::new_now().as_bytes().to_vec(),
                "new".to_owned(),
                0,
                word,
                reading,
                sentence,
                vec![
                    model::card::reading_audio_id::set(Some(reading_audio_id)),
                    model::card::sentence_audio_id::set(Some(
                        sentence_audio_id.as_bytes().to_vec(),
                    )),
                ],
            )
            .exec()
            .await
            .unwrap();
    }

    pub async fn schedule_new_cards(&self, cards: &[(Card, NextCardState)]) {
        let tx = self.db.client()._transaction();

        tx.run(async move |client| {
            for (card, state) in cards {
                client
                    .card()
                    .update(
                        model::card::id::equals(card.as_data().id.to_vec()),
                        vec![
                            model::card::state::set(state.state.to_string()),
                            model::card::step::set(state.state.get_step().map(i32::from)),
                            model::card::due::set(state.due as i64),
                            model::card::stability::set(state.stability),
                            model::card::difficulty::set(state.difficulty),
                        ],
                    )
                    .exec()
                    .await
                    .unwrap();
            }

            Ok(()) as Result<(), QueryError>
        })
        .await
        .unwrap();
    }
}

//==============================================================================
// ...
//==============================================================================
trait Convert<T> {
    #[must_use]
    fn into(self) -> T;
}

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
