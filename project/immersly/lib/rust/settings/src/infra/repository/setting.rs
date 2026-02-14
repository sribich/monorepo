use std::sync::Arc;

use prisma_client::PrismaClient;
use prisma_client::QueryError;
use prisma_client::model;
use railgun::di::Component;
use shared::infra::database::Sqlite;

use crate::domain::entity::setting::Setting;

#[derive(Component)]
pub struct SettingRepository {
    db: Arc<Sqlite>,
}

//==============================================================================
// Utils
//==============================================================================
impl SettingRepository {
    pub fn client(&self) -> &PrismaClient {
        self.db.client()
    }

    pub fn model(&self) -> model::setting::Actions {
        self.db.client().setting()
    }
}
//==============================================================================
// Reader
//==============================================================================

//==============================================================================
// Writer
//==============================================================================
impl SettingRepository {
    pub async fn initialize(&self, setting: &Setting) -> Result<(), QueryError> {
        let value = setting.value();
        let data = setting.as_inner();

        let params = model::setting::Create {
            id: data.id.to_vec(),
            name: data.name.clone(),
            kind: value.kind(),
            value: value.value(),
            params: vec![],
        };

        params.to_query(self.client()).exec().await?;

        Ok(())
    }
}

//==============================================================================
// Transformer
//==============================================================================
