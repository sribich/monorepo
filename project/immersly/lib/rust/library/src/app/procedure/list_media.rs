use std::sync::Arc;

use railgun::di::Component;
use shared::{IntoVec, infra::Procedure};

use crate::{app::dto::book::BookDto, infra::repository::book::BookRepository};

pub struct MediaListRes {
    pub books: Vec<BookDto>,
}

#[derive(Component)]
pub struct ListMediaProcedure {
    book_repository: Arc<BookRepository>,
}

impl Procedure for ListMediaProcedure {
    type Err = core::convert::Infallible;
    type Req = ();
    type Res = MediaListRes;

    async fn run(&self, _: Self::Req) -> Result<Self::Res, Self::Err> {
        let books = self.book_repository.find_all().await.unwrap().into_vec();

        Ok(Self::Res { books })
    }
}
