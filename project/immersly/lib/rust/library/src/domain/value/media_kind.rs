use railgun::error::Error;
use railgun::error::Location;

use crate::feature::library::domain::entity::book::Book;

#[derive(Debug, Clone)]
pub enum MediaKind {
    None,
    Book(Book),
}

impl MediaKind {
    pub fn to_string(&self) -> String {
        match self {
            MediaKind::Book(_) => "Book",
            MediaKind::None => "None",
        }
        .to_owned()
    }
}
