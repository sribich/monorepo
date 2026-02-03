use std::marker::PhantomData;
use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::QueryError;
use prisma_client::model;
use railgun_di::Component;

use crate::feature::scheduler::domain::card::Card;
use crate::feature::scheduler::domain::card::CardId;
use crate::feature::scheduler::domain::card::NextCardState;

pub struct Reader;
pub struct Writer;

#[derive(Component)]
pub struct SchedulerRepository<T = ()>
where
    T: 'static + Send + Sync,
{
    db: Arc<Sqlite>,
    _phantom: PhantomData<T>,
}

impl SchedulerRepository<()> {
    pub fn reader(&self) -> SchedulerRepository<Reader> {
        SchedulerRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }

    pub fn writer(&self) -> SchedulerRepository<Writer> {
        SchedulerRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }
}

impl SchedulerRepository<Reader> {
    fn client(&self) -> model::card::Actions {
        self.db.client().card()
    }

    pub async fn from_id(&self, id: CardId) -> Result<Option<model::card::Data>, QueryError> {
        self.client()
            .find_unique(model::card::id::equals(id.to_vec()))
            .exec()
            .await
    }

    pub async fn get_next_due(
        &self,
        // next_day_threshold: usize,
    ) -> Result<Option<model::card::Data>, QueryError> {
        let now = chrono::Local::now();

        self.client()
            .find_first(vec![
                model::card::state::not("new".to_owned()),
                model::card::due::lt(now.timestamp()),
            ])
            // .find_first(vec![or(vec![
            //     and(vec![model::card::due::gt(now.timestamp())]),
            //     and(vec![model::card::due::gt(0)]),
            // ])])
            .exec()
            .await
    }

    pub async fn list_new_cards(&self, count: u32) -> Result<Vec<model::card::Data>, QueryError> {
        self.client()
            .find_many(vec![model::card::state::equals("new".to_owned())])
            .take(i64::from(count))
            .exec()
            .await
    }
}

impl SchedulerRepository<Writer> {
    pub async fn apply_learning_state(&self, card: &Card) {
        self.db
            .client()
            .card()
            .update(
                model::card::id::equals(card.id.to_vec()),
                vec![
                    model::card::state::set(card.state.to_string()),
                    model::card::step::set(card.step()),
                    model::card::due::set(card.due as i64),
                    model::card::last_review::set(card.last_review.map(|it| it as i64)),
                    model::card::difficulty::set(card.difficulty),
                    model::card::stability::set(card.stability),
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
        reading_audio_id: Muid,
        sentence_audio_id: Muid,
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
                Muid::new_now().as_bytes().to_vec(),
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
                        model::card::id::equals(card.id.to_vec()),
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
