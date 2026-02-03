use railgun_error::Error;
use railgun_error::Location;

#[derive(Error)]
#[error(crate_path = "railgun_error")]
pub enum PluginError {
    #[error(display("An error has occurred in MPV"))]
    MpvError {
        #[error(impl_from)]
        error: mpv_client::Error,
        location: Location,
    },
}
