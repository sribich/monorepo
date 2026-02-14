use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;
use storage::domain::value::ResourceId;

use crate::domain::entity::book::Book;
use crate::domain::value::book_id::BookId;

#[derive(Deserialize, Serialize, Typegen)]
#[serde(rename = "Book", rename_all = "camelCase")]
pub struct BookDto {
    pub id: BookId,
    pub title: String,
    pub image_id: Option<ResourceId>,
}

impl From<Book> for BookDto {
    fn from(value: Book) -> Self {
        let value = value.to_inner();

        Self {
            id: value.id,
            title: value.title,
            image_id: value.image_resource,
        }
    }
}
