use railgun_error::Error;
use railgun_error::Location;

use crate::cache::TypeCache;

pub mod config;
pub mod resolver;

#[derive(Error)]
#[error(crate_path = "railgun_error")]
pub enum ExportError {
    #[error(display("The route '{path}' is defined multiple times."))]
    DuplicateRoute { path: String, location: Location },
    #[error(display("The type '{ty}' was never resolved"))]
    MissingType { ty: String, location: Location },
    #[error(display("{msg}"))]
    InvariantError { msg: String, location: Location },
}

pub trait TypeExporter {
    type Options;
    type Data;

    fn export(
        options: Self::Options,
        data: Self::Data,
        cache: &TypeCache,
        // config: ExportConfig,
        // prefix: Option<String>,
        // cache: &TypeCache,
        // procedures: &Vec<(String, String, NamedDataType, NamedDataType)>,
    ) -> Result<String, ExportError>;
}
