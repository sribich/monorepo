use std::fs::create_dir_all;
use std::sync::Arc;

use railgun::di::Component;
use railgun::error::Error;
use railgun::error::Location;
use railgun::error::OptionExt;
use shared::infra::Procedure;
use shared::infra::dirs::get_data_dir;

use crate::domain::value::ResourceId;
use crate::infra::repository::resource::ResourceRepository;

//==============================================================================
// Data
//==============================================================================
pub struct PrepareResourceReq {
    pub filename: String,
}

pub struct PrepareResourceRes {
    pub resource: ResourceId,
    pub path: String,
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
pub struct PrepareResourceProcedure {
    resource_repository: Arc<ResourceRepository>,
}

impl Procedure for PrepareResourceProcedure {
    type Err = Error;
    type Req = PrepareResourceReq;
    type Res = PrepareResourceRes;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let data_dir = get_data_dir(); // .context(NoDataDirectoryContext {})?;

        let uuid = ResourceId::new_now();
        let (dir1, dir2) = (uuid.as_bytes()[14], uuid.as_bytes()[15]);

        let dirname = data_dir
            .join(hex::encode(&[dir1]))
            .join(hex::encode(&[dir2]));

        create_dir_all(&dirname).unwrap();

        let filename = dirname.join(data.filename);

        let data = self
            .resource_repository
            .prepare_resource(&uuid, filename.to_str().unwrap().to_owned())
            .await
            .unwrap();

        Ok(PrepareResourceRes {
            resource: data.id().clone(),
            path: data.path().to_str().unwrap().to_string(),
        })
    }
}
