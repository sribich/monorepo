use std::path::PathBuf;

use super::TypeExporter;

#[derive(Default)]
pub struct ExportConfig<T: TypeExporter> {
    pub path: PathBuf,
    pub options: T::Options,
}

impl<T: TypeExporter> ExportConfig<T> {
    pub fn new(path: impl Into<PathBuf>, options: T::Options) -> Self {
        Self {
            path: path.into(),
            options,
        }
    }

    /*
    #[must_use]
    pub fn path(self, path: impl Into<PathBuf>) -> Self {
        Self {
            path: Some(path.into()),
        }
    }
    */
}
