use std::collections::BTreeMap;

use typegen::{
    cache::TypeCache,
    export::{ExportError, TypeExporter},
};

use crate::router::Route;

pub mod typescript;
pub use typegen_typescript;

pub trait ClientExporter: TypeExporter {
    fn export_client(
        options: Self::Options,
        prefix: Option<String>,
        data: Vec<Route>,
        cache: &TypeCache,
    ) -> Result<BTreeMap<String, String>, ExportError>;
}
