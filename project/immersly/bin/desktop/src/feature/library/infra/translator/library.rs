use prisma_client::model;

use crate::feature::library::domain::{
    aggregate::library_title::LibraryTitle, value::library_id::LibraryId,
};

impl TryFrom<model::library::Data> for LibraryTitle {
    type Error = core::convert::Infallible;

    fn try_from(value: model::library::Data) -> Result<Self, Self::Error> {
        Ok(LibraryTitle::create_unchecked(
            LibraryId::from_slice(&value.id).unwrap(),
            value.title,
            value
                .image_resource
                .map_or_default(|it| it.map_or_default(|it| it.try_into().ok())),
        ))
    }
}
