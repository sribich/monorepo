use shared::domain::value::existing_file::ExistingFile;
use shared::entity_data_fns;
use storage::domain::value::ResourceId;

use crate::domain::value::book_id::BookId;

pub struct BookData {
    pub id: BookId,
    pub title: String,
    pub path: ExistingFile,
    pub rendered_path: ExistingFile,
    pub timing_path: ExistingFile,
    pub audio_resource: ResourceId,
    pub image_resource: Option<ResourceId>,
}

pub struct Book(BookData);
entity_data_fns!(Book);

impl Book {
    pub fn id(&self) -> &BookId {
        &self.0.id
    }

    pub fn title(&self) -> &String {
        &self.0.title
    }

    pub fn path(&self) -> &ExistingFile {
        &self.0.path
    }

    pub fn rendered_path(&self) -> &ExistingFile {
        &self.0.rendered_path
    }

    pub fn timing_path(&self) -> &ExistingFile {
        &self.0.timing_path
    }

    pub fn audio_resource_id(&self) -> &ResourceId {
        &self.0.audio_resource
    }

    pub fn image_resource_id(&self) -> Option<&ResourceId> {
        self.0.image_resource.as_ref()
    }
}
