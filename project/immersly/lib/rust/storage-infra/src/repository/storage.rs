/*
use std::marker::PhantomData;
use std::sync::Arc;

use railgun::di::Component;
use shared::domain::value::muid::Muid;
use shared::infra::database::QueryError;
use shared::infra::database::Sqlite;
use shared::infra::database::model;
use storage_domain::value::ResourceId;

pub struct Reader;
pub struct Writer;

#[derive(Component)]
pub struct ResourceRepository<T = ()>
where
    T: 'static + Send + Sync,
{
    db: Arc<Sqlite>,
    _phantom: PhantomData<T>,
}

impl ResourceRepository<()> {
    pub fn reader(&self) -> ResourceRepository<Reader> {
        ResourceRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }

    pub fn writer(&self) -> ResourceRepository<Writer> {
        ResourceRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }
}

impl ResourceRepository<Reader> {
    pub async fn from_id(
        &self,
        id: &ResourceId
    ) -> core::result::Result<Option<model::resource::Data>, QueryError> {
        self.db
            .client()
            .resource()
            .find_unique(model::resource::id::equals(id.to_vec()))
            .exec()
            .await
    }
}

impl ResourceRepository<Writer> {
    pub async fn prepare_resource(
        &self,
        id: &Uuid,
        path: String,
    ) -> core::result::Result<model::resource::Data, QueryError> {
        self.db
            .client()
            .resource()
            .create(
                id.as_bytes().to_vec(),
                "preparing".to_owned(),
                String::new(),
                String::new(),
                path,
                String::new(),
                vec![],
            )
            .exec()
            .await
    }

    pub async fn commit(
        &self,
        id: &ResourceId,
        hash: String,
    ) -> core::result::Result<(), QueryError> {
        self.db
            .client()
            .resource()
            .update(
                model::resource::id::equals(id.to_vec()),
                vec![
                    model::resource::state::set("committed".to_owned()),
                    model::resource::hash::set(hash),
                ],
            )
            .exec()
            .await?;

        Ok(())
    }

    pub async fn create_from_path(
        &self,
        path: &ExistingPath,
    ) -> core::result::Result<Muid, QueryError> {
        let id = Muid::new_now();

        self.db
            .client()
            .resource()
            .create(
                id.as_bytes().to_vec(),
                "committed".to_owned(),
                String::new(),
                String::new(),
                path.as_str().to_owned(),
                String::new(),
                vec![],
            )
            .exec()
            .await?;

        Ok(id)
    }

    //     pub async fn save(&self, mut setting: Setting) {
    //         let transaction = self.db.client()._transaction();
    //
    //         let result = transaction
    //             .run(|client| async move {
    //                 for event in setting.change_events() {
    //                     match event {
    //                         SettingChange::Created(setting) => {
    //                             self.create_setting(&client, &setting).await;
    //                         },
    //                     }
    //                 }
    //
    //                 Ok(()) as core::result::Result<(), QueryError>
    //             })
    //             .await
    //             .unwrap();
    //     }
    //
    //     async fn create_setting(&self, client: &PrismaClient, setting: &Setting) {
    //         let value = setting.value();
    //
    //         client
    //             .setting()
    //             .create(
    //                 setting.id().as_bytes().to_vec(),
    //                 setting.name().to_owned(),
    //                 value.kind(),
    //                 value.value(),
    //                 vec![],
    //             )
    //             .exec()
    //             .await
    //             .unwrap();
    //     }
}
*/
