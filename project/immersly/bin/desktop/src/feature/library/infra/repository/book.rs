use std::sync::Arc;

use async_trait::async_trait;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model::book::Actions;
use prisma_client::model::{self};

use crate::domain::value::existing_path::ExistingPath;
use crate::feature::library::domain::aggregate::book::Book;
use crate::feature::library::domain::aggregate::book::BookId;
use crate::feature::library::domain::repository::book::BookReader;
use crate::feature::library::domain::repository::book::BookRepository;
use crate::feature::library::domain::repository::book::BookWriter;
use crate::impl_repository;

impl_repository!(
    SqliteBook,
    Book,
    db: Arc<Sqlite>,
);

impl SqliteBookReader {
    fn client(&self) -> Actions {
        self.db.client().book()
    }
}

#[async_trait]
impl BookReader for SqliteBookReader {
    async fn find_all(&self) -> Vec<Book> {
        let result = self.client().find_many(vec![]).exec().await.unwrap();

        result
            .into_iter()
            .map(|it| {
                Book::from_parts(
                    BookId::from_slice_unchecked(&it.id),
                    it.title.unwrap(),
                    // TODO: Move this to new -- figure out what to do when invariant is invalid?
                    ExistingPath::new_unchecked(it.path),
                    ExistingPath::new_unchecked(it.rendered_path),
                    ExistingPath::new_unchecked(it.rendered_audio_path.unwrap()),
                    Muid::from_slice_unchecked(&it.audio_resource_id.unwrap()),
                    it.image_resource_id
                        .map(|it| Muid::from_slice_unchecked(&it)),
                )
            })
            .collect::<Vec<_>>()
    }
}

impl SqliteBookWriter {
    fn client(&self) -> Actions {
        self.db.client().book()
    }
}

#[async_trait]
impl BookWriter for SqliteBookWriter {
    async fn create(&self, book: &Book) {
        self.client()
            .create(
                book.id().to_vec(),
                book.path().as_str().to_owned(),
                book.rendered_path().as_str().to_owned(),
                vec![
                    model::book::title::set(Some(book.title().to_owned())),
                    model::book::rendered_audio_path::set(Some(
                        book.timing_path().as_str().to_owned(),
                    )),
                    model::book::audio_resource_id::set(Some(
                        book.audio_resource_id().as_bytes().to_vec(),
                    )),
                ],
            )
            .exec()
            .await
            .unwrap();
    }
}
