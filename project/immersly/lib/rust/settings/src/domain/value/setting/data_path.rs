use std::path::PathBuf;

use dirs::data_local_dir;

use super::SettingMeta;
use crate::domain::entity::setting::Setting;
use crate::domain::value::kind::Kind;
use crate::domain::value::kind::existing_path::ExistingPath;

#[derive(Debug)]
pub struct DataPath {
    value: ExistingPath,
}

impl Default for DataPath {
    fn default() -> Self {
        Self {
            value: ExistingPath::new_unchecked(format!(
                "{}/{}/data",
                data_local_dir().unwrap().to_str().unwrap().to_owned(),
                env!("CARGO_PKG_NAME"),
            ))
            .unwrap(),
        }
    }
}

impl DataPath {
    pub fn to_path(&self) -> PathBuf {
        self.value.to_path()
    }

    pub fn to_string(&self) -> String {
        self.value.as_value()
    }
}

impl SettingMeta for DataPath {
    type Value = ExistingPath;

    const DESCRIPTION: &'static str = "";
    const DISPLAY_NAME: &'static str = "";
    const NAME: &'static str = "data_path";

    fn into_aggregate(&self) -> Setting {
        Setting::new(Self::NAME.to_owned(), Self::default().value.as_kind())
    }

    fn from_parts(name: String, kind: String, value: String) -> Self {
        Self {
            value: ExistingPath::from_parts(kind, value),
        }
    }
}
