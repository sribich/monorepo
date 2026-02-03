use std::sync::Arc;

use async_trait::async_trait;
use railgun_di::Component;

use crate::feature::settings::app::service::settings::SettingService;
use crate::feature::settings::infra::repository::setting::SettingRepository;
use crate::system::OnStartup;

#[derive(Component)]
#[component(implements(Vec<dyn OnStartup>))]
pub struct InitialiseSettings {
    setting_repository: Arc<SettingRepository>,
    setting_service: Arc<SettingService>,
}

#[async_trait]
impl OnStartup for InitialiseSettings {
    async fn run(&self) -> core::result::Result<(), Box<dyn core::error::Error>> {
        let stored_settings = self.setting_service.existing_settings().await;
        let available_settings = self.setting_service.default_settings();

        for setting in available_settings {
            if !stored_settings.iter().any(|it| it == setting.name()) {
                self.setting_repository.writer().save(setting).await;
            }
        }

        Ok(())
    }
}
