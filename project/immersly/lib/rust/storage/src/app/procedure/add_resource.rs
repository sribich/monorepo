use std::sync::Arc;

use railgun::di::Component;
use railgun::error::Error;
use railgun::error::Location;
use shared::domain::value::existing_file::ExistingFile;
use shared::infra::Procedure;

use crate::domain::entity::resource::Resource;
use crate::infra::repository::resource::ResourceRepository;

//==============================================================================
// Data
//==============================================================================
pub struct AddResourceReq {
    pub path: ExistingFile,
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
    type Res = Resource;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        Ok(self
            .resource_repository
            .create_existing_resource(&data.path.as_path().to_owned())
            .await
            .unwrap())
    }
}
