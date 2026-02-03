use std::fs::read_to_string;
use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun::typegen::Typegen;
use railgun_di::Component;
use serde::Serialize;

use crate::system::Procedure;

#[derive(Debug, Serialize, Typegen)]
pub struct SeriesDto {
    pub text: String,
    pub audio_id: String,
    pub progress: Option<i64>,
}

#[derive(Component)]
pub struct ReadBookProcedure {
    db: Arc<Sqlite>,
}

impl Procedure for ReadBookProcedure {
    type Err = core::convert::Infallible;
    type Req = Muid;
    type Res = SeriesDto;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let book = self
            .db
            .client()
            .book()
            .find_unique(model::book::id::equals(data.as_bytes().to_vec()))
            .with(model::book::progress::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        let progress = if let Some(progress) = book.progress.flatten() {
            Some(progress.timestamp)
        } else {
            None
        };

        let data = read_to_string(book.rendered_audio_path.unwrap()).unwrap();

        Ok(Self::Res {
            text: data,
            audio_id: Muid::from_slice_unchecked(&book.audio_resource_id.unwrap()).to_string(),
            progress,
        })
    }
}
