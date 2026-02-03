use crate::domain::value::existing_path::ExistingPath;
use crate::feature::library::domain::value::library_id::LibraryId;

#[derive(Clone, Debug)]
pub struct Book {
    id: LibraryId,
    title: String,
    path: ExistingPath,
    rendered_path: ExistingPath,
}

impl Book {
    pub fn new(title: String, path: ExistingPath, rendered_path: ExistingPath) -> Self {
        Self {
            id: LibraryId::new_now(),
            title,
            path,
            rendered_path,
        }
    }

    pub fn from_parts(
        id: LibraryId,
        title: String,
        path: ExistingPath,
        rendered_path: ExistingPath,
    ) -> Self {
        Self {
            id,
            title,
            path,
            rendered_path,
        }
    }

    pub fn id(&self) -> &LibraryId {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn path(&self) -> &ExistingPath {
        &self.path
    }

    pub fn rendered_path(&self) -> &ExistingPath {
        &self.rendered_path
    }
}
