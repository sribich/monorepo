use std::path::PathBuf;
use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use railgun_di::Component;

use crate::feature::settings::app::service::settings::SettingService;
use crate::feature::settings::domain::value::setting::data_path::DataPath;

#[derive(Component)]
pub struct DictionaryService {
    setting_service: Arc<SettingService>,
}

impl DictionaryService {
    pub async fn get_data_path(&self, dictionary_id: &Muid) -> PathBuf {
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
