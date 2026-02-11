use std::sync::Arc;

use railgun::error::Error;
use railgun::error::Location;
use railgun_di::Component;
use sha256_util::sha256_digest;

use features::storage::domain::value::ResourceId;
use crate::feature::storage::repository::resource::ResourceRepository;
use crate::system::Procedure;

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

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let resource = self
            .resource_repository
            .reader()
            .from_id(&data.resource)
            .await
            .unwrap()
            .unwrap();

        let hash = sha256_digest(&resource.path).unwrap();

        self.resource_repository
            .writer()
            .commit(
                &ResourceId::from_slice_unchecked(&resource.id),
                hash.clone(),
            )
            .await
            .unwrap();

        Ok(CommitResourceRes { hash })
    }
}
