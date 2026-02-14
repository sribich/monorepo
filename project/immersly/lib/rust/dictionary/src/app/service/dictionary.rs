use std::path::PathBuf;
use std::sync::Arc;

use railgun::di::Component;
use settings::app::service::settings::SettingService;
use settings::domain::value::setting::data_path::DataPath;
use shared::infra::database::Sqlite;

use crate::domain::value::dictionary_id::DictionaryId;

#[derive(Component)]
pub struct DictionaryService {
    db: Arc<Sqlite>,
    setting_service: Arc<SettingService>,
}

impl DictionaryService {
    pub async fn get_data_path(&self, dictionary_id: &DictionaryId) -> PathBuf {
        let mut path = self
            .setting_service
            .get_setting::<DataPath>()
            .await
            .to_path();

        path.push("dictionaries");
        path.push(dictionary_id.to_string());

        path
    }
}
