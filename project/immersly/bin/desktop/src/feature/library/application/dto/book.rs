use features::shared::domain::value::muid::Muid;

use crate::feature::library::domain::aggregate::book::Book;
use crate::feature::library::domain::aggregate::book::BookId;

pub struct BookDto {
    pub id: BookId,
    pub title: String,
    pub image_id: Option<Muid>,
}

impl From<Book> for BookDto {
    fn from(value: Book) -> Self {
        let value = value.take();

        Self {
            id: value.id,
            title: value.title,
            image_id: value.image_resource,
        }
    }
}
