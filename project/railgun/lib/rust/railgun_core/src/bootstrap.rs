use railgun_error::Error;
use railgun_error::Location;

pub type BootstrapResult<T> = core::result::Result<T, BootstrapError>;

/// A service initialisation error.
#[derive(Error)]
#[error(crate_path = "railgun_error")]
pub enum BootstrapError {
    #[error(display("Failed to parse CLI arguments"))]
    ArgumentError {
        error: Box<dyn core::error::Error + Send + Sync>,
        location: Location,
    },
    #[error(display("{reason}"))]
    Generic {
        reason: &'static str,
        location: Location,
    },

    #[error(display("failed to initiailize application"))]
    Initialization {
        #[error(impl_from)]
        error: Box<dyn core::error::Error + Send + Sync>,
        location: Location,
    },
}
