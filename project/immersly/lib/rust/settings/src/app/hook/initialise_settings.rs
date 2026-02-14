use std::sync::Arc;

use async_trait::async_trait;
use railgun::di::Component;
use shared::OnStartup;

use crate::{app::service::settings::SettingService, infra::repository::setting::SettingRepository};

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
                self.setting_repository.initialize(&setting).await.unwrap();
            }
        }

        Ok(())
    }
}
