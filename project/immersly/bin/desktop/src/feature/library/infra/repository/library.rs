use std::{marker::PhantomData, sync::Arc};

use prisma_client::{PrismaClient, QueryError, model};
use railgun_di::Component;

use crate::{
    domain::value::existing_path::ExistingPath,
    feature::{
        library::domain::{
            aggregate::{library_title::LibraryTitle, media::Media},
            cdc::MediaChange,
            entity::book::Book,
            value::{library_id::LibraryId, media_kind::MediaKind},
        },
        storage::domain::values::ResourceId,
    },
};

pub struct Reader;
pub struct Writer;

#[derive(Component)]
pub struct LibraryRepository<T = ()>
where
    T: 'static + Send + Sync,
{
    db: Arc<Sqlite>,
    _phantom: PhantomData<T>,
}

impl LibraryRepository<()> {
    pub fn reader(&self) -> LibraryRepository<Reader> {
        LibraryRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }

    pub fn writer(&self) -> LibraryRepository<Writer> {
        LibraryRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }
}

impl LibraryRepository<Reader> {
    pub async fn from_id(
        &self,
        id: &LibraryId,
    ) -> Result<Option<Media>, Box<dyn core::error::Error>> {
        let result = self
            .db
            .client()
            .media()
            .find_unique(model::media::id::equals(id.as_bytes().to_vec()))
            .with(model::media::book::fetch())
            .exec()
            .await
            .unwrap();

        let translator = MediaTranslator {};

        Ok(result.map(|it| translator.translate(it)))
    }

    pub async fn list_media(&self) -> Result<Vec<Media>, Box<dyn core::error::Error>> {
        let result = self
            .db
            .client()
            .media()
            .find_many(vec![])
            .with(model::media::book::fetch())
            .exec()
            .await?;

        let translator = MediaTranslator {};

        Ok(result
            .into_iter()
            .map(|it| translator.translate(it))
            .collect::<Vec<_>>())
    }

    pub async fn get_media(
        &self,
        id: &LibraryId,
    ) -> Result<Option<Media>, Box<dyn core::error::Error>> {
        let result = self
            .db
            .client()
            .media()
            .find_unique(model::media::id::equals(id.as_bytes().to_vec()))
            .with(model::media::book::fetch())
            .exec()
            .await
            .unwrap();

        Ok(result.map(|it| {
            let translator = MediaTranslator {};

            translator.translate(it)
        }))
    }
}

impl LibraryRepository<Writer> {
    pub async fn update_title(&self, title: &LibraryId, resource_id: &ResourceId) {
        self.db
            .client()
            .library()
            .update(
                model::library::id::equals(title.as_bytes().to_vec()),
                vec![model::library::image_resource_id::set(Some(
                    resource_id.as_bytes().to_vec(),
                ))],
            )
            .exec()
            .await
            .unwrap();
    }

    pub async fn save_media(&self, mut media: Media) {
        let events = media.changes();

        let transaction = self.db.client()._transaction();

        println!("{events:#?}");

        transaction
            .run(|client| async move {
                for event in events {
                    match event {
                        MediaChange::Created(media) => self.create_media(&client, &media).await,
                        MediaChange::DeleteBook(book) => self.delete_book(&client, &book).await,
                        MediaChange::SetBook(book) => {
                            self.set_book(&client, &book, media.id()).await;
                        },
                        MediaChange::SetKind(kind) => {
                            client
                                .media()
                                .update(
                                    model::media::id::equals(media.id().as_bytes().to_vec()),
                                    vec![model::media::kind::set(kind)],
                                )
                                .exec()
                                .await
                                .unwrap();
                        },
                    }
                }

                Ok(()) as Result<(), QueryError>
            })
            .await
            .unwrap();
    }

    async fn create_media(&self, client: &PrismaClient, media: &Media) {
        client
            .media()
            .create(
                media.id().as_bytes().to_vec(),
                media.title().to_string(),
                media.kind().to_string(),
                vec![model::media::library_id::set(Some(
                    media.series_id().as_bytes().to_vec(),
                ))],
            )
            .exec()
            .await
            .unwrap();
    }

    async fn delete_book(&self, client: &PrismaClient, book: &Book) {
        client
            .book()
            .delete(model::book::id::equals(book.id().as_bytes().to_vec()))
            .exec()
            .await
            .unwrap();
    }

    async fn set_book(&self, client: &PrismaClient, book: &Book, media_id: &LibraryId) {
        client
            .book()
            .create(
                book.id().as_bytes().to_vec(),
                book.path().as_str().to_string(),
                book.rendered_path().as_str().to_string(),
                model::media::id::equals(media_id.as_bytes().to_vec()),
                vec![model::book::title::set(Some(book.title().to_owned()))],
            )
            .exec()
            .await
            .unwrap();
    }

    pub async fn add_media(&self, media: Media) -> Result<(), ()> {
        self.db
            .client()
            .media()
            .create(
                media.id().as_bytes().to_vec(),
                media.title().to_string(),
                media.kind().to_string(),
                vec![],
            )
            .exec()
            .await
            .unwrap();

        Ok(())
    }
}

struct MediaTranslator {}

impl MediaTranslator {
    pub fn translate(&self, from: model::media::Data) -> Media {
        // TODO: unwrap
        Media::create_unchecked(
            LibraryId::from_slice_unchecked(&from.id),
            from.title.clone(),
            Self::get_connected_record_type(&from),
            Muid::from_slice_unchecked(&from.library_id.unwrap()),
        )
    }

    pub fn get_connected_record_type(from: &model::media::Data) -> MediaKind {
        let book_translator = BookTranslator {};

        match &*from.kind {
            "Book" => MediaKind::Book(book_translator.translate(from.book().unwrap().unwrap())),
            "None" => MediaKind::None,
            _ => {
                panic!("Unknown kind {}", from.kind);
            },
        }
    }
}

struct BookTranslator {}

impl BookTranslator {
    pub fn translate(&self, from: &model::book::Data) -> Book {
        let id = LibraryId::from_slice(&from.id).unwrap();
        let path = ExistingPath::new(&from.path).unwrap();
        let rendered_path = ExistingPath::new(&from.rendered_path).unwrap();

        Book::from_parts(id, from.title.clone().unwrap(), path, rendered_path)
    }
}
