use std::sync::Arc;

use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::domain::value::muid::Muid;
use shared::infra::database::Sqlite;

use crate::domain::value::book_id::BookId;

#[derive(Component)]
pub struct BookProgressRepository {
    db: Arc<Sqlite>,
}

//==============================================================================
// Util
//==============================================================================
impl BookProgressRepository {
    pub fn model(&self) -> model::book_progress::Actions {
        self.db.client().book_progress()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl BookProgressRepository {}

//==============================================================================
// Writer
//==============================================================================
impl BookProgressRepository {
    pub async fn set_progress(&self, book_id: &BookId, timestamp: i64) -> Result<(), QueryError> {
        let create_params = model::book_progress::Create {
            id: Muid::new_now().to_vec(),
            timestamp,
            book: model::book::id::equals(book_id.to_vec()),
            params: vec![],
        };

        self.model()
            .upsert(
                model::book_progress::book_id::equals(book_id.to_vec()),
                create_params,
                vec![model::book_progress::timestamp::set(timestamp)],
            )
            .exec()
            .await?;

        Ok(())
    }
}
