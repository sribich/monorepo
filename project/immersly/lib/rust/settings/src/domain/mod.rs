use value::kind::Kind;
use value::kind::existing_path::ExistingPath;

pub mod entity;
pub mod value;

#[derive(Clone)]
// #[serde(tag = "kind")]
pub enum SettingKind {
    ExistingPath(ExistingPath),
}

impl SettingKind {
    pub fn kind(&self) -> String {
        match self {
            SettingKind::ExistingPath(kind) => kind.name(),
        }
    }

    pub fn value(&self) -> String {
        match self {
            SettingKind::ExistingPath(kind) => kind.as_value(),
        }
    }

    pub fn constraints(&self) -> Option<String> {
        match self {
            SettingKind::ExistingPath(kind) => kind.as_constraints(),
        }
    }
}
