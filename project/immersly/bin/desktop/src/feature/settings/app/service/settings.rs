use std::sync::Arc;

use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun_di::Component;

use crate::feature::settings::domain::aggregate::setting::Setting;
use crate::feature::settings::domain::value::setting::SettingMeta;
use crate::feature::settings::domain::value::setting::data_path::DataPath;

#[derive(Component)]
pub struct SettingService {
    db: Arc<Sqlite>,
}

impl SettingService {
    pub async fn get_setting<T: SettingMeta>(&self) -> T {
        let result = self
            .db
            .client()
            .setting()
            .find_unique(model::setting::name::equals(T::NAME.to_owned()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        T::from_parts(result.name, result.kind, result.value)
    }

    pub async fn get_setting_by_name(name: String) {}

    pub async fn existing_settings(&self) -> Vec<String> {
        self.db
            .client()
            .setting()
            .find_many(vec![])
            .exec()
            .await
            .unwrap()
            .into_iter()
            .map(|it| it.name)
            .collect::<Vec<_>>()
    }

    pub fn default_settings(&self) -> [Setting; 1] {
        [DataPath::default().into_aggregate()]
    }

    // pub async fn get_setting<T>(setting: T) {}
}
