use std::sync::Arc;

use railgun::di::Component;
use railgun::error::Error;
use railgun::error::Location;
use sha256_util::sha256_digest;
use shared::infra::Procedure;

use crate::domain::entity::resource::Resource;
use crate::domain::value::ResourceId;
use crate::infra::repository::resource::ResourceRepository;

//==============================================================================
// Data
//==============================================================================
pub struct GetResourceReq {
    pub id: ResourceId,
}

//==============================================================================
// Error
//==============================================================================
#[derive(Error)]
pub enum Error {
    #[error(display("Unable to locate a suitable data directory"))]
    NoDataDirectory { location: Location },
}

//==============================================================================
// Procedure
//==============================================================================
#[derive(Component)]
pub struct GetResourceProcedure {
    resource_repository: Arc<ResourceRepository>,
}

impl Procedure for GetResourceProcedure {
    type Err = Error;
    type Req = GetResourceReq;
    type Res = Option<Resource>;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        Ok(self.resource_repository.from_id(&data.id).await.unwrap())
    }
}
