use std::path::PathBuf;

use railgun::error::Error;
use railgun::error::Location;

use super::Kind;
use crate::feature::settings::domain::SettingKind;

#[derive(Clone, Debug)]
pub struct ExistingPath {
    path: String,
}

impl ExistingPath {
    pub fn new(path: String) -> core::result::Result<Self, Error> {
        if !PathBuf::from(&path).exists() {
            return PathDoesNotExistContext { path }.fail()?;
        }

        Ok(Self { path })
    }

    pub fn new_unchecked(path: String) -> core::result::Result<Self, Error> {
        Ok(Self { path })
    }

    pub fn to_path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

impl Kind for ExistingPath {
    const KIND: &'static str = "ExistingPath";

    fn name(&self) -> String {
        Self::KIND.into()
    }

    fn as_value(&self) -> String {
        self.path.clone()
    }

    fn as_constraints(&self) -> Option<String> {
        None
    }

    fn parse(data: &prisma_client::model::setting::Data) -> SettingKind {
        // TODO: unwrap
        SettingKind::ExistingPath(ExistingPath {
            path: data.value.clone(),
        })
    }

    fn as_kind(&self) -> SettingKind {
        SettingKind::ExistingPath(self.clone())
    }

    fn from_parts(kind: String, value: String) -> Self {
        assert!((kind == Self::KIND), "wrong kind");

        Self { path: value }
    }
}

#[derive(Error)]
pub enum Error {
    #[error(display("Path '{path}' does not exist"))]
    PathDoesNotExist { path: String, location: Location },
}
