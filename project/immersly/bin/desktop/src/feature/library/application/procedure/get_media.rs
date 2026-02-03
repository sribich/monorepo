use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun_di::Component;

use crate::system::Procedure;

pub struct Media {
    pub id: String,
    pub title: String,
    pub kind: String,
}

#[derive(Component)]
pub struct GetMedia {
    db: Arc<Sqlite>,
}

impl Procedure for GetMedia {
    type Err = core::convert::Infallible;
    type Req = Muid;
    type Res = Media;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let result = self
            .db
            .client()
            .media()
            .find_unique(model::media::id::equals(data.as_bytes().to_vec()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        Ok(Self::Res {
            id: Muid::from_slice_unchecked(&result.id).to_string(),
            title: result.title,
            kind: result.kind,
        })
    }
}
