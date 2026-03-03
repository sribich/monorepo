use crate::locator::LocatedFiles;

#[derive(Clone, Debug)]
pub struct SqlFiles {}

impl From<&LocatedFiles> for SqlFiles {
    fn from(_value: &LocatedFiles) -> SqlFiles {
        SqlFiles {}
    }
}

impl From<&LocatedFiles> for Option<SqlFiles> {
    fn from(_value: &LocatedFiles) -> Option<SqlFiles> {
        Some(SqlFiles {})
    }
}
