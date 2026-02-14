use shared::entity_data_fns;

use crate::domain::SettingKind;
use crate::domain::value::setting_id::SettingId;

pub struct SettingData {
    pub id: SettingId,
    pub name: String,
    pub value: SettingKind,
}

pub struct Setting(SettingData);

entity_data_fns!(Setting);

impl Setting {
    pub fn new(name: String, value: SettingKind) -> Self {
        Self(SettingData {
            id: SettingId::new_now(),
            name,
            value,
        })
    }

    pub fn id(&self) -> &SettingId {
        &self.0.id
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn value(&self) -> &SettingKind {
        &self.0.value
    }
}
