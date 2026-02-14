use std::sync::Arc;

use prisma_client::model;
use railgun::di::Component;
use shared::infra::Procedure;

use crate::domain::value::book_id::BookId;
use crate::infra::repository::book_progress::BookProgressRepository;

//==============================================================================
// Data
//==============================================================================
pub struct SetProgressReq {
    pub book_id: BookId,
    pub progress: i64,
}

pub struct SetProgressRes {}

//==============================================================================
// UseCase
//==============================================================================
#[derive(Component)]
pub struct SetProgressProcedure {
    book_progress_repository: Arc<BookProgressRepository>,
}

impl Procedure for SetProgressProcedure {
    type Err = core::convert::Infallible;
    type Req = SetProgressReq;
    type Res = SetProgressRes;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        self.book_progress_repository
            .set_progress(&data.book_id, data.progress)
            .await
            .unwrap();

        Ok(SetProgressRes {})
    }
}
