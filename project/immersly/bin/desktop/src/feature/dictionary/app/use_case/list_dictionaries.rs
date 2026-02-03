use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use railgun::typegen::Typegen;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::system::UseCase;

#[derive(Debug, Deserialize, Typegen)]
pub struct ListDictionariesQueryRequest {}

#[derive(Debug, Serialize, Typegen)]
pub struct ListDictionariesQueryResponse {
    dictionaries: Vec<ListDictionariesDictionary>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct ListDictionariesDictionary {
    id: String,
    title: String,
}

#[derive(Component)]
pub struct ListDictionariesQuery {
    db: Arc<Sqlite>,
}

impl UseCase for ListDictionariesQuery {
    type Err = core::convert::Infallible;
    type Req = ListDictionariesQueryRequest;
    type Res = ListDictionariesQueryResponse;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let result = self
            .db
            .client()
            .dictionary()
            .find_many(vec![])
            .exec()
            .await
            .unwrap();

        Ok(Self::Res {
            dictionaries: result
                .into_iter()
                .map(|it| ListDictionariesDictionary {
                    id: Muid::try_from_slice(&it.id).unwrap().to_string(),
                    title: it.title,
                })
                .collect(),
        })
    }
}
