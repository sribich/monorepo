use features::shared::domain::value::muid::Muid;

use crate::feature::library::domain::cdc::MediaChange;
use crate::feature::library::domain::entity::book::Book;
use crate::feature::library::domain::value::library_id::LibraryId;
use crate::feature::library::domain::value::media_kind::MediaKind;

#[derive(Debug, Clone)]
pub struct Media {
    id: LibraryId,
    title: String,
    kind: MediaKind,
    series_id: Muid,
    _changes: Vec<MediaChange>,
}

impl Media {
    pub fn new(title: String, series_id: Muid) -> Self {
        let mut media = Self {
            _changes: vec![],
            id: LibraryId::new_now(),
            title,
            kind: MediaKind::None,
            series_id,
        };

        media._changes.push(MediaChange::Created(media.clone()));

        media
    }

    pub fn create_unchecked(
        id: LibraryId,
        title: String,
        kind: MediaKind,
        series_id: Muid,
    ) -> Self {
        Self {
            _changes: vec![],
            id,
            title,
            kind,
            series_id,
        }
    }

    pub fn id(&self) -> &LibraryId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn kind(&self) -> &MediaKind {
        &self.kind
    }

    pub fn series_id(&self) -> &Muid {
        &self.series_id
    }

    pub fn changes(&mut self) -> Vec<MediaChange> {
        std::mem::take(&mut self._changes)
    }

    pub fn set_book(&mut self, book: Book) {
        let existing = self.kind.clone();

        self.kind = MediaKind::Book(book.clone());

        match existing {
            MediaKind::None => {}
            MediaKind::Book(book) => self._changes.push(MediaChange::DeleteBook(book)),
        }

        self._changes
            .push(MediaChange::SetKind(self.kind.to_string()));
        self._changes.push(MediaChange::SetBook(book));
    }
}
