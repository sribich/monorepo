use crate::locator::LocatedFiles;

#[derive(Clone, Debug)]
pub struct Migrations {}

impl From<&LocatedFiles> for Migrations {
    fn from(value: &LocatedFiles) -> Migrations {
        Migrations {}
    }
}

impl From<&LocatedFiles> for Option<Migrations> {
    fn from(value: &LocatedFiles) -> Option<Migrations> {
        Some(Migrations {})
    }
}
