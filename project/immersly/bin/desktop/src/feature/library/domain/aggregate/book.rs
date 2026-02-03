use features::shared::domain::value::muid::Muid;
use features::shared::muid_newtype;
use railgun_di::Take;

use crate::domain::value::existing_path::ExistingPath;

muid_newtype!(BookId);

#[derive(Debug, Take)]
pub struct Book {
    id: BookId,
    title: String,
    path: ExistingPath,
    rendered_path: ExistingPath,
    timing_path: ExistingPath,
    audio_resource: Muid,
    image_resource: Option<Muid>,
}

impl Book {
    pub fn new(
        title: String,
        path: ExistingPath,
        rendered_path: ExistingPath,
        timing_path: ExistingPath,
        audio_resource: Muid,
    ) -> Self {
        Self {
            id: BookId::new_now(),
            title,
            path,
            rendered_path,
            timing_path,
            audio_resource,
            image_resource: None,
        }
    }

    pub fn from_parts(
        id: BookId,
        title: String,
        path: ExistingPath,
        rendered_path: ExistingPath,
        timing_path: ExistingPath,
        audio_resource: Muid,
        image_resource: Option<Muid>,
    ) -> Self {
        Self {
            id,
            title,
            path,
            rendered_path,
            timing_path,
            audio_resource,
            image_resource,
        }
    }

    pub fn id(&self) -> &BookId {
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

    pub fn timing_path(&self) -> &ExistingPath {
        &self.timing_path
    }

    pub fn audio_resource_id(&self) -> &Muid {
        &self.audio_resource
    }

    pub fn image_resource_id(&self) -> Option<&Muid> {
        self.image_resource.as_ref()
    }
}
