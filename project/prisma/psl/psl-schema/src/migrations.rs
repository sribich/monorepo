use crate::locator::LocatedFiles;

#[derive(Clone, Debug)]
pub struct Migrations {}

impl From<&LocatedFiles> for Migrations {
    fn from(_value: &LocatedFiles) -> Migrations {
        Migrations {}
    }
}

impl From<&LocatedFiles> for Option<Migrations> {
    fn from(_value: &LocatedFiles) -> Option<Migrations> {
        Some(Migrations {})
    }
}
