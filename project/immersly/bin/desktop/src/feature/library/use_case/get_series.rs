use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun::typegen::Typegen;
use railgun_di::Component;
use serde::Serialize;

use crate::system::Query;

#[derive(Debug, Serialize, Typegen)]
pub struct SeriesView {
    #[typegen(inline)]
    media: Vec<MediaView>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct MediaView {
    id: String,
    title: String,
    kind: String,
}

#[derive(Component)]
pub struct GetSeriesQuery {
    db: Arc<Sqlite>,
}

impl Query for GetSeriesQuery {
    type Err = core::convert::Infallible;
    type Req = Muid;
    type Res = SeriesView;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        todo!();
        /*
        let result = self
            .db
            .client()
            .media()
            .find_many(vec![model::media::library_id::equals(Some(
                data.as_bytes().to_vec(),
            ))])
            .exec()
            .await
            .unwrap();

        let media = result
            .into_iter()
            .map(|it| MediaView {
                id: Muid::from_slice(&it.id).unwrap().to_string(),
                title: it.title,
                kind: it.kind,
            })
            .collect::<Vec<_>>();

        Ok(Self::Res { media })
         */
    }
}
