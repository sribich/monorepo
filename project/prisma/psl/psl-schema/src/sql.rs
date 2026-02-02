use crate::locator::LocatedFiles;

#[derive(Clone, Debug)]
pub struct SqlFiles {}

impl From<&LocatedFiles> for SqlFiles {
    fn from(value: &LocatedFiles) -> SqlFiles {
        SqlFiles {}
    }
}

impl From<&LocatedFiles> for Option<SqlFiles> {
    fn from(value: &LocatedFiles) -> Option<SqlFiles> {
        Some(SqlFiles {})
    }
}
