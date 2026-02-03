use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun_di::Component;

use crate::system::UseCase;

//==============================================================================
// Data
//==============================================================================
#[derive(Debug)]
pub struct Req {
    pub media_id: Muid,
    pub progress: i64,
}

pub struct Res {}

//==============================================================================
// UseCase
//==============================================================================
#[derive(Component)]
pub struct SetProgressUseCase {
    db: Arc<Sqlite>,
}

impl UseCase for SetProgressUseCase {
    type Err = core::convert::Infallible;
    type Req = Req;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        self.db
            .client()
            .book_progress()
            .upsert(
                model::book_progress::book_id::equals(data.media_id.as_bytes().to_vec()),
                model::book_progress::create(
                    Muid::new_now().as_bytes().to_vec(),
                    data.progress,
                    model::book::id::equals(data.media_id.as_bytes().to_vec()),
                    vec![],
                ),
                vec![model::book_progress::timestamp::set(data.progress)],
            )
            .exec()
            .await
            .unwrap();

        Ok(Res {})
    }
}
