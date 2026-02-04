pub mod v2;
pub mod v3;

mod container;
mod package;

use std::io::Read;
use std::io::Seek;

pub use container::*;
pub use package::*;
use zip::ZipArchive;

use crate::error::Result;

pub(crate) trait FromZip<'a> {
    type Type;

    fn read(zip: &'a mut ZipArchive<impl Read + Seek>) -> Result<String>;

    fn parse<S: AsRef<str>>(data: S) -> Result<Self::Type>;
}

pub(crate) trait FromParameterizedZip<'a> {
    type Params;
    type Type;

    fn parse(zip: &'a mut ZipArchive<impl Read + Seek>, data: Self::Params) -> Result<Self::Type>;
}

#[derive(Debug)]
pub enum GeneralPackage {
    V2(v2::Package),
    V3(v3::Package),
}

impl GeneralPackage {}
