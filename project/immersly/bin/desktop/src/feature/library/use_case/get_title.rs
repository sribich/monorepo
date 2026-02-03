use core::result::Result;
use std::sync::Arc;

use railgun_di::Component;

use crate::feature::library::domain::value::library_id::LibraryId;
use crate::feature::storage::domain::values::ResourceId;
use crate::system::Procedure;

pub struct Res {
    pub title: String,
    pub image_id: Option<ResourceId>,
}

#[derive(Component)]
pub struct GetTitleProcedure {
    // repository: Arc<LibraryRepository>,
}

impl Procedure for GetTitleProcedure {
    type Err = core::convert::Infallible;
    type Req = LibraryId;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        todo!();
        /*
        let data = self.repository.reader().get_title(&data).await.unwrap();

        Ok(Self::Res {
            title: data.title().to_owned(),
            image_id: data.image_id().cloned(),
        })
        */
    }
}
