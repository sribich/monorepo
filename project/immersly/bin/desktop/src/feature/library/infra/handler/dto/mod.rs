use railgun::typegen::Typegen;
use serde::Serialize;

use crate::feature::library::application::dto::book::BookDto;

#[derive(Serialize, Typegen)]
#[serde(rename = "Book")]
pub struct BookView {
    pub id: String,
    pub title: String,
    pub image_id: Option<String>,
}

impl From<BookDto> for BookView {
    fn from(value: BookDto) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            image_id: value.image_id.map(|it| it.to_string()),
        }
    }
}
