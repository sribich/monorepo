use railgun::error::Error;
use railgun::error::Location;
use zip::result::ZipError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error)]
#[error(module)]
pub enum ParseError {
    #[error(display("Expected file {path} to exist, but it does not."))]
    MissingRequiredFile {
        error: ZipError,
        path: String,
        location: Location,
    },
    #[error(transparent)]
    IoError {
        error: std::io::Error,
        location: Location,
    },
}

#[derive(Error)]
pub enum Error {
    #[error(display("Unable to locate package document."))]
    NoPackages {
        location: Location,
    },

    #[error(display("Validation failed: {invariant}"))]
    Validation {
        invariant: &'static str,
        location: Location,
    },

    #[error(display("Unable to parse epub file"))]
    Unsupported {
        source: UnsupportedReason,
    },

    InvalidEpub {
        reason: String,
        location: Location,
    },

    #[error(transparent)]
    ParseError {
        #[error(impl_from)]
        error: ParseError,
    },

    // #[error(transparent)]
    // Other(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    IoError {
        error: std::io::Error,
    },
    #[error(transparent)]
    XmlError {
        #[error(impl_from)]
        error: quick_xml::DeError,
    },

    #[error(transparent)]
    Other {
        // #[error(from(core::error::Error, Box::new))]
        // #[error(impl_from)]
        error: Box<dyn core::error::Error>,
    },
}

#[derive(Error)]
pub enum UnsupportedReason {
    #[error(display("Encrypted epubs are not supported"))]
    Encrypted,
}

impl From<ZipError> for Error {
    #[track_caller]
    fn from(value: ZipError) -> Self {
        println!("{:#?}", value);
        let caller = core::panic::Location::caller();
        println!("{:#?}", caller);

        match value {
            ZipError::Io(_) => todo!(),
            ZipError::InvalidArchive(_) => todo!(),
            ZipError::UnsupportedArchive(_) => todo!(),
            ZipError::FileNotFound => Error::Other { error: "".into() },
            ZipError::InvalidPassword => Error::Unsupported {
                source: UnsupportedReason::Encrypted,
            },
            _ => unreachable!(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        todo!()
    }
}
