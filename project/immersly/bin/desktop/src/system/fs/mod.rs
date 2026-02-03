use std::fs::metadata;
use std::path::PathBuf;
use std::str::FromStr;

use color_eyre::eyre::Context;
use color_eyre::eyre::eyre;
use dirs::data_local_dir;
use tokio::io::AsyncWriteExt;

use self::error::FsError;

pub mod error;

// TODO: Implement this
// pub fn check_path_traversal() {}

pub struct Fs {
    base: PathBuf,
}

impl Fs {
    pub fn new(base: String) -> Self {
        Self {
            base: PathBuf::from_str(&base).unwrap(),
        }
    }

    pub async fn write<P: AsRef<str>>(&self, subpath: P, text: String) -> PathBuf {
        assert!(
            !subpath.as_ref().starts_with('/'),
            "subpath can not be absolute"
        );

        let path = self.base.join(subpath.as_ref());

        match metadata(&path) {
            Ok(_) => {}
            #[allow(non_exhaustive_omitted_patterns)]
            Err(it) => {
                if it.kind() == std::io::ErrorKind::NotFound {
                    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                }
            }
        }

        let mut file = tokio::fs::File::create(path.clone()).await.unwrap();

        file.write_all(text.as_bytes()).await.unwrap();

        path
    }
}

/// Returns the path to the data dir that the application can use to
/// persist data to the filesystem.
///
/// The requested path will be created if it does not already exist
/// and any error in doing so will propogate to the caller.
///
/// TODO: Ensure path is utf8. Don't allow non utf8
pub fn get_data_dir(subpath: Option<&'_ str>) -> Result<PathBuf, FsError> {
    if let Some(path) = subpath
        && PathBuf::from(path).is_absolute()
    {
        return Err(FsError::NotRelative(path.to_string()));
    }

    let base_path = data_local_dir()
        .ok_or_else(|| FsError::Other(eyre!("Unable to locate the user data dir")))?;

    #[cfg(debug_assertions)]
    let data_path = base_path.join("prelearning/application-debug");
    #[cfg(not(debug_assertions))]
    let data_path = base_path.join("prelearning/application");

    let full_path = if let Some(path) = subpath {
        data_path.join(path)
    } else {
        data_path
    };
    //

    match metadata(&full_path) {
        Ok(meta) => {
            if meta.is_dir() {
                Ok(full_path)
            } else {
                Err(FsError::NotADirectory(
                    full_path.to_string_lossy().into_owned(),
                ))
            }
        }
        #[allow(non_exhaustive_omitted_patterns)]
        Err(it) => match it.kind() {
            std::io::ErrorKind::NotFound => {
                std::fs::create_dir_all(&full_path)
                    .map_err(|e| FsError::Other(e.into()))
                    .wrap_err(format!(
                        "Failed to create data directory at '{}'",
                        full_path.to_string_lossy()
                    ))?;
                Ok(full_path)
            }
            _ => Err(FsError::Other(it.into())),
        },
    }
}

pub fn get_data_file(subpath: Option<&'_ str>) -> Result<PathBuf, FsError> {
    if let Some(path) = subpath
        && PathBuf::from(path).is_absolute()
    {
        return Err(FsError::NotRelative(path.to_string()));
    }

    let full_path = if let Some(path) = subpath {
        let file_path = PathBuf::from(path);

        let file_root =
            if let Some(dirname) = file_path.parent() {
                Some(dirname.to_str().ok_or_else(|| {
                    FsError::Other(eyre!("File does not contain a valid OS path."))
                })?)
            } else {
                None
            };

        let file_base = file_path
            .file_name()
            .ok_or_else(|| FsError::Other(eyre!("Unable to parse file basename.")))?
            .to_str()
            .ok_or_else(|| FsError::Other(eyre!("File does not contain a valid OS path.")))?;

        let data_path = get_data_dir(file_root)?;
        data_path.join(file_base)
    } else {
        get_data_dir(None)?
    };

    match metadata(&full_path) {
        Ok(meta) => {
            if meta.is_file() {
                Ok(full_path)
            } else {
                Err(FsError::NotAFile(full_path.to_string_lossy().into_owned()))
            }
        }
        #[allow(non_exhaustive_omitted_patterns)]
        Err(it) => match it.kind() {
            std::io::ErrorKind::NotFound => Ok(full_path),
            _ => Err(FsError::Other(it.into())),
        },
    }
}
