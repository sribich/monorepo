use std::path::PathBuf;

use chrono::DateTime;
use chrono::Utc;
use mime::Mime;

use crate::domain::value::ResourceId;
use crate::domain::value::ResourceState;

pub struct ResourceData {
    pub id: ResourceId,
    pub state: ResourceState,
    pub hash: Option<String>,
    pub path: PathBuf,
    pub mime_type: Option<Mime>,
    pub managed: bool,
    pub last_access: Option<DateTime<Utc>>,
}

pub struct Resource {
    id: ResourceId,
    state: ResourceState,
    hash: Option<String>,
    path: PathBuf,
    mime_type: Option<Mime>,
    managed: bool,
    last_access: Option<DateTime<Utc>>,
}

impl Resource {
    pub fn from_data(data: ResourceData) -> Resource {
        Resource {
            id: data.id,
            state: data.state,
            hash: data.hash,
            path: data.path,
            mime_type: data.mime_type,
            managed: data.managed,
            last_access: data.last_access,
        }
    }

    pub fn id(&self) -> &ResourceId {
        &self.id
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

pub enum ResourceChangeEvent {}

/*
use features::shared::domain::value::muid::Muid;
use features::storage::domain::value::ResourceId;
use prisma_client::model;

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: ResourceId,
    pub path: String,
}

impl TryFrom<model::resource::Data> for Resource {
    type Error = core::convert::Infallible;

    fn try_from(value: model::resource::Data) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::from_slice_unchecked(&value.id),
            path: value.path,
        })
    }
}

impl TryFrom<Box<model::resource::Data>> for Resource {
    type Error = core::convert::Infallible;

    fn try_from(value: Box<model::resource::Data>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::from_slice_unchecked(&value.id),
            path: value.path,
        })
    }
}

// pub struct Resource {
//     id: ResourceId,
//     mime_type: MimeType,
//     path: String,
//     hash: String,
// }
//
*/
