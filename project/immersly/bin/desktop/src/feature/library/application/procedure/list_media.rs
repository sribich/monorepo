use std::sync::Arc;

use features::shared::infra::Procedure;
use railgun_di::Component;

use crate::feature::library::application::dto::book::BookDto;
use crate::feature::library::domain::repository::book::BookReader;

pub struct MediaList {
    pub books: Vec<BookDto>,
}

#[derive(Component)]
pub struct ListMedia {
    book_reader: Arc<dyn BookReader>,
}

impl Procedure for ListMedia {
    type Err = core::convert::Infallible;
    type Req = ();
    type Res = MediaList;

    async fn run(&self, _: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let books = self.list_books().await;

        Ok(Self::Res { books })
    }
}

impl ListMedia {
    async fn list_books(&self) -> Vec<BookDto> {
        let result = self.book_reader.find_all().await;

        result.into_iter().map(Into::into).collect()
    }
}
