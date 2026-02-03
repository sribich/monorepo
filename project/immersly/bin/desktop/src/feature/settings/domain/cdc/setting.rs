use crate::feature::settings::domain::aggregate::setting::Setting;

#[derive(Clone)]
pub enum SettingChange {
    Created(Setting),
}
