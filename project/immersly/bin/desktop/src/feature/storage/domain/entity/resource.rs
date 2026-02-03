use features::shared::domain::value::muid::Muid;
use prisma_client::model;

use crate::feature::storage::domain::values::ResourceId;

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
