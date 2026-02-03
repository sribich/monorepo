use std::sync::Arc;

use features::shared::infra::database::Sqlite;
use lexorank::LexoRank;
use lexorank::Rankable;
use prisma_client::model::SortOrder;
use prisma_client::model::{self};
use railgun_di::Component;

#[derive(Component)]
pub struct DictionaryRanker {
    #[inject(default)]
    ranker: LexoRank,
    db: Arc<Sqlite>,
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
        self.db
            .client()
            .dictionary()
            .find_first(vec![])
            .order_by(model::dictionary::rank::order(SortOrder::Asc))
            .exec()
            .await
            .unwrap()
            .map(|it| it.rank)
    }

    async fn last(&self) -> Option<String> {
        self.db
            .client()
            .dictionary()
            .find_first(vec![])
            .order_by(model::dictionary::rank::order(SortOrder::Desc))
            .exec()
            .await
            .unwrap()
            .map(|it| it.rank)
    }
}
