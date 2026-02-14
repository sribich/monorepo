use std::sync::Arc;

use railgun::di::Component;
use shared::infra::Procedure;

use crate::app::dto::dictionary::DictionaryDto;
use crate::domain::entity::dictionary::Dictionary;
use crate::infra::repository::dictionary::DictionaryRepository;

pub struct ListDictionariesReq {}

pub struct ListDictionariesRes {
    pub dictionaries: Vec<DictionaryDto>,
}

#[derive(Component)]
pub struct ListDictionariesProcedure {
    dictionary_repository: Arc<DictionaryRepository>,
}

impl Procedure for ListDictionariesProcedure {
    type Err = core::convert::Infallible;
    type Req = ListDictionariesReq;
    type Res = ListDictionariesRes;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        Ok(ListDictionariesRes {
            dictionaries: self.dictionary_repository.list().await.unwrap().into_iter().map(Into::into).collect::<Vec<_>>()
        })
    }
}
