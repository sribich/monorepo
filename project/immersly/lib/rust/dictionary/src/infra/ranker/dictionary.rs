use std::sync::Arc;

use lexorank::LexoRank;
use lexorank::Rankable;
use railgun::di::Component;

use crate::infra::repository::dictionary::DictionaryRepository;

#[derive(Component)]
pub struct DictionaryRanker {
    #[inject(default)]
    ranker: LexoRank,
    dictionary_repository: Arc<DictionaryRepository>,
}

impl DictionaryRanker {
    pub async fn next(&self) -> String {
        self.ranker.next(self).await.to_string()
    }

    pub async fn prev(&self) -> String {
        self.ranker.next(self).await.to_string()
    }
}

impl Rankable for &DictionaryRanker {
    async fn first(&self) -> Option<String> {
        self.dictionary_repository.first_rank().await.unwrap()
    }

    async fn last(&self) -> Option<String> {
        self.dictionary_repository.last_rank().await.unwrap()
    }
}
