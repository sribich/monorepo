use features::shared::domain::value::muid::Muid;

use crate::feature::settings::domain::SettingKind;
use crate::feature::settings::domain::cdc::setting::SettingChange;

#[derive(Clone)]
pub struct Setting {
    id: Muid,
    name: String,
    value: SettingKind,
    _changes: Vec<SettingChange>,
}

impl Setting {
    pub fn new(name: String, value: SettingKind) -> Self {
        let mut this = Self {
            id: Muid::new_now(),
            name,
            value,
            _changes: vec![],
        };

        this._changes.push(SettingChange::Created(this.clone()));

        this
    }

    pub fn id(&self) -> &Muid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &SettingKind {
        &self.value
    }

    pub fn change_events(&mut self) -> Vec<SettingChange> {
        std::mem::take(&mut self._changes)
    }
}
