use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use railgun::error::Error;
use railgun::error::Location;
use railgun_di::Component;

use crate::domain::value::existing_path::ExistingPath;
use features::storage::domain::value::ResourceId;
use crate::feature::storage::repository::resource::ResourceRepository;
use crate::system::Procedure;

//==============================================================================
// Data
//==============================================================================
pub struct AddResourceReq {
    pub path: ExistingPath,
}

//==============================================================================
// Procedure
//==============================================================================
#[derive(Component)]
pub struct AddResourceProcedure {
    resource_repository: Arc<ResourceRepository>,
}

impl Procedure for AddResourceProcedure {
    type Err = core::convert::Infallible;
    type Req = AddResourceReq;
    type Res = Muid;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let id = self
            .resource_repository
            .writer()
            .create_from_path(&data.path)
            .await
            .unwrap();

        Ok(id)
    }
}
