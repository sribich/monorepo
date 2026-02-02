use railgun_error::Location;
use railgun_error_derive::Error;

pub type Result<T> = core::result::Result<T, PrismaError>;

#[derive(Error)]
#[error(crate_path = railgun_error)]
pub enum PrismaError {
    #[error(transparent)]
    IoError {
        error: std::io::Error,
        location: Location,
    },

    #[error(display("{reason}"))]
    ExternalError {
        reason: &'static str,
        error: Box<dyn core::error::Error>,
        location: Location,
    },
    #[error(display("{reason}"))]
    GenericError {
        reason: &'static str,
        error: String,
        location: Location,
    },
    #[error(display("{reason}"))]
    StringError {
        reason: &'static str,
        location: Location,
    },
}
