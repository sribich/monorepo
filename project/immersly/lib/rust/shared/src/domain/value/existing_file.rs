use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use railgun::error::Error;
use railgun::error::Location;
use railgun::ext::UnwrapInfallible;

/// Represents a file that currently exists on the filesystem.
///
#[derive(Clone, Debug)]
pub struct ExistingFile {
    inner: PathBuf,
}

impl ExistingFile {
    pub fn new<S: AsRef<str>>(path: S) -> Result<Self, ExistingFileError> {
        let path = Self::new_unchecked(path);

        path.validate()?;

        Ok(path)
    }

    pub fn new_unchecked<S: AsRef<str>>(path: S) -> Self {
        let path_str = path.as_ref().to_owned();
        let path = PathBuf::from_str(&path_str).unwrap_infallible();

        Self { inner: path }
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self { inner: path }
    }

    pub fn validate(&self) -> Result<(), ExistingFileError> {
        if !self.inner.exists() {
            return DoesNotExistContext {
                path: self.as_str().to_owned(),
            }
            .fail();
        }

        Ok(())
    }

    pub fn as_path(&self) -> &Path {
        &self.inner
    }

    pub fn as_str(&self) -> &str {
        self.inner
            .to_str()
            .expect("AsRef<str> in constructors will always be UTF-8")
    }
}

#[derive(Error)]
pub enum ExistingFileError {
    #[error(display("Path '{path}' does not exist"))]
    DoesNotExist { path: String, location: Location },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_on_existing_path() {
        let path = ExistingFile::new("./Cargo.toml");

        path.unwrap();
    }

    #[test]
    fn err_on_nonexistent_path() {
        let path = ExistingFile::new("./THIS_FILE_DOES_NOT_EXIST");

        path.unwrap_err();
    }
}
