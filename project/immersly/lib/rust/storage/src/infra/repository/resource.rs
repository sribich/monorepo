use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use mime::Mime;
use railgun::di::Component;
use shared::infra::database::QueryError;
use shared::infra::database::Sqlite;
use shared::infra::database::model;
use crate::domain::entity::resource::Resource;
use crate::domain::entity::resource::ResourceData;
use crate::domain::value::ResourceId;
use crate::domain::value::ResourceState;

#[derive(Component)]
pub struct ResourceRepository {
    db: Arc<Sqlite>,
}

impl ResourceRepository {
    fn model(&self) -> model::resource::Actions {
        self.db.client().resource()
    }
}

//==============================================================================
// Read
//==============================================================================
impl ResourceRepository {
    pub async fn from_id(&self, id: &ResourceId) -> Result<Option<Resource>, QueryError> {
        self.model()
            .find_unique(model::resource::id::equals(id.to_vec()))
            .exec()
            .await
            .map(Convert::into)
    }
}

//==============================================================================
// Write
//==============================================================================
impl ResourceRepository {
    #[must_use]
    pub async fn prepare_resource(
        &self,
        id: &ResourceId,
        path: String,
    ) -> Result<Resource, QueryError> {
        let params = model::resource::Create {
            id: id.to_vec(),
            state: ResourceState::Preparing.to_string(),
            path,
            managed: true,
            params: vec![],
        };

        params
            .to_query(self.db.client())
            .exec()
            .await
            .map(Convert::into)
    }

    #[must_use]
    pub async fn commit_resource(&self, id: &ResourceId, hash: String) -> Result<Resource, QueryError> {
        self.model()
            .update(
                model::resource::id::equals(id.to_vec()),
                vec![
                    model::resource::state::set(ResourceState::Committed.to_string()),
                    model::resource::hash::set(Some(hash)),
                ],
            )
            .exec()
            .await
            .map(Convert::into)
    }

    #[must_use]
    pub async fn create_existing_resource(&self, path: &PathBuf) -> Result<Resource, QueryError> {
        let params = model::resource::Create {
            id: ResourceId::new_now().to_vec(),
            state: ResourceState::Committed.to_string(),
            path: path.to_str().unwrap().to_owned(),
            managed: false,
            params: vec![],
        };

        params
            .to_query(self.db.client())
            .exec()
            .await
            .map(Convert::into)
    }
}

trait Convert<T> {
    #[must_use]
    fn into(self) -> T;
}

impl Convert<Resource> for model::resource::Data {
    fn into(self) -> Resource {
        Resource::from_data(ResourceData {
            id: ResourceId::from_slice_unchecked(&self.id),
            state: ResourceState::from_str(&self.state).unwrap(),
            hash: self.hash,
            path: PathBuf::from(self.path),
            mime_type: self.mime_type.map(|it| Mime::from_str(&it).unwrap()),
            managed: self.managed,
            last_access: self.last_access.map(|it| it.to_utc()),
        })
    }
}

impl Convert<Option<Resource>> for Option<model::resource::Data> {
    fn into(self) -> Option<Resource> {
        self.map(Convert::into)
    }
}
