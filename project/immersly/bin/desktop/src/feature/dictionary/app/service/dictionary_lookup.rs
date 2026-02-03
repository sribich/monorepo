use std::sync::Arc;

use features::shared::infra::database::Sqlite;
use prisma_client::QueryError;
use prisma_client::model::SortOrder;
use prisma_client::model::{self};
use railgun_di::Component;

#[derive(Component)]
pub struct DictionaryLookupService {
    db: Arc<Sqlite>,
}

impl DictionaryLookupService {
    pub async fn lookup_word(
        &self,
        word: String,
        reading: Option<String>,
    ) -> Result<Vec<model::word::Data>, QueryError> {
        let find_many_criteria = match reading {
            Some(reading) => vec![
                model::word::word::equals(word),
                model::word::reading::equals(reading),
            ],
            None => vec![model::word::word::equals(word)],
        };

        self.db
            .client()
            .word()
            .find_many(find_many_criteria)
            .with(model::word::dictionary::fetch())
            .order_by(model::word::dictionary::order(vec![
                model::dictionary::rank::order(SortOrder::Asc),
            ]))
            .exec()
            .await
    }

    pub async fn lookup_word_exact(
        &self,
        word: String,
        reading: Option<String>,
    ) -> Result<(), QueryError> {
        let definitions = self
            .find_many_by(vec![
                model::word::word::equals(word.clone()),
                model::word::reading::equals(reading.clone().unwrap_or_default()),
                model::word::dictionary::is(vec![model::dictionary::language_type::equals(
                    "mono".to_owned(),
                )]),
            ])
            .await
            .unwrap();

        let words = self
            .find_many_by(vec![
                model::word::word::equals(word.clone()),
                model::word::reading::equals(reading.clone().unwrap_or_default()),
                model::word::dictionary::is(vec![model::dictionary::language_type::equals(
                    "bi".to_owned(),
                )]),
            ])
            .await
            .unwrap();

        Ok(())
    }

    async fn find_many_by(
        &self,
        data: Vec<model::word::WhereParam>,
    ) -> Result<Vec<model::word::Data>, QueryError> {
        self.db
            .client()
            .word()
            .find_many(data)
            .with(model::word::dictionary::fetch())
            .order_by(model::word::dictionary::order(vec![
                model::dictionary::rank::order(SortOrder::Asc),
            ]))
            .exec()
            .await
    }
}
