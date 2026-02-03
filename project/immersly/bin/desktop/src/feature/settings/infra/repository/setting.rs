use std::marker::PhantomData;
use std::sync::Arc;

use features::shared::infra::database::Sqlite;
use prisma_client::PrismaClient;
use prisma_client::QueryError;
use railgun_di::Component;

use crate::feature::settings::domain::aggregate::setting::Setting;
use crate::feature::settings::domain::cdc::setting::SettingChange;

pub struct Reader;
pub struct Writer;

#[derive(Component)]
pub struct SettingRepository<T = ()>
where
    T: 'static + Send + Sync,
{
    db: Arc<Sqlite>,
    _phantom: PhantomData<T>,
}

impl SettingRepository<()> {
    pub fn reader(&self) -> SettingRepository<Reader> {
        SettingRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }

    pub fn writer(&self) -> SettingRepository<Writer> {
        SettingRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }
}

impl SettingRepository<Writer> {
    pub async fn save(&self, mut setting: Setting) {
        let transaction = self.db.client()._transaction();

        transaction
            .run(|client| async move {
                for event in setting.change_events() {
                    match event {
                        SettingChange::Created(setting) => {
                            self.create_setting(&client, &setting).await;
                        }
                    }
                }

                Ok(()) as core::result::Result<(), QueryError>
            })
            .await
            .unwrap();
    }

    async fn create_setting(&self, client: &PrismaClient, setting: &Setting) {
        let value = setting.value();

        client
            .setting()
            .create(
                setting.id().as_bytes().to_vec(),
                setting.name().to_owned(),
                value.kind(),
                value.value(),
                vec![],
            )
            .exec()
            .await
            .unwrap();
    }
}

impl SettingRepository<Reader> {}
