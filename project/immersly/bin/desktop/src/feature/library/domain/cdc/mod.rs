use super::aggregate::media::Media;
use super::aggregate::series::Series;
use super::entity::book::Book;

#[derive(Debug, Clone)]
pub enum MediaChange {
    Created(Media),
    DeleteBook(Book),
    SetBook(Book),
    SetKind(String),
}

#[derive(Clone, Debug)]
pub enum SeriesChange {
    Created(Series),
}
