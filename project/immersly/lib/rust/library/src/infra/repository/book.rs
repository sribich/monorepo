use std::sync::Arc;

use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::domain::value::existing_file::ExistingFile;
use shared::infra::database::Sqlite;
use storage::domain::value::ResourceId;

use crate::domain::entity::book::Book;
use crate::domain::entity::book::BookData;
use crate::domain::value::book_id::BookId;

#[derive(Component)]
pub struct BookRepository {
    db: Arc<Sqlite>,
}

//==============================================================================
// Util
//==============================================================================
impl BookRepository {
    pub fn model(&self) -> model::book::Actions {
        self.db.client().book()
    }
}

//==============================================================================
// Reader
//==============================================================================
impl BookRepository {
    pub async fn find_all(&self) -> Result<Vec<Book>, QueryError> {
        self.model()
            .find_many(vec![])
            .exec()
            .await
            .map(Convert::into)
    }
}

//==============================================================================
// Writer
//==============================================================================
impl BookRepository {
    pub async fn create(&self, book: &Book) {
        self.model()
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

//==============================================================================
// Transforms
//==============================================================================
trait Convert<T> {
    #[must_use]
    fn into(self) -> T;
}

impl Convert<Book> for model::book::Data {
    fn into(self) -> Book {
        BookData {
            id: BookId::from_slice_unchecked(&self.id),
            title: self.title.unwrap(),
            path: ExistingFile::new_unchecked(self.path),
            rendered_path: ExistingFile::new_unchecked(self.rendered_path),
            timing_path: ExistingFile::new_unchecked(self.rendered_audio_path.unwrap()),
            audio_resource: ResourceId::from_slice_unchecked(&self.audio_resource_id.unwrap()),
            image_resource: self
                .image_resource_id
                .map(|it| ResourceId::from_slice_unchecked(&it)),
        }
        .into()
    }
}

impl Convert<Option<Book>> for Option<model::book::Data> {
    fn into(self) -> Option<Book> {
        self.map(Convert::into)
    }
}

impl Convert<Vec<Book>> for Vec<model::book::Data> {
    fn into(self) -> Vec<Book> {
        self.into_iter().map(Convert::into).collect::<Vec<_>>()
    }
}
