use std::sync::Arc;

use railgun::di::Component;
use railgun::error::Error;
use railgun::error::Location;
use sha256_util::sha256_digest;
use shared::infra::Procedure;

use crate::domain::value::ResourceId;
use crate::infra::repository::resource::ResourceRepository;

//==============================================================================
// Data
//==============================================================================
pub struct CommitResourceReq {
    pub resource: ResourceId,
}

pub struct CommitResourceRes {
    pub hash: String,
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
pub struct CommitResourceProcedure {
    resource_repository: Arc<ResourceRepository>,
}

impl Procedure for CommitResourceProcedure {
    type Err = Error;
    type Req = CommitResourceReq;
    type Res = CommitResourceRes;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let resource = self
            .resource_repository
            .from_id(&data.resource)
            .await
            .unwrap()
            .unwrap();

        let hash = sha256_digest(resource.path()).unwrap();

        let _ = self
            .resource_repository
            .commit_resource(resource.id(), hash.clone())
            .await
            .unwrap();

        Ok(CommitResourceRes { hash })
    }
}
