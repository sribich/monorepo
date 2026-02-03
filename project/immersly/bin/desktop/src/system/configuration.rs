use std::path::PathBuf;

use railgun::core::ServiceInfo;
use railgun::core::bootstrap::BootstrapResult;
use railgun::error::ResultExt;
use railgun::settings::cli::Cli;
use railgun::settings::net::SocketAddr;
use railgun::settings::settings;
use railgun::telemetry::settings::TelemetrySettings;

use super::dirs::get_app_dir;

/// A struct containing the possible values for the configuration
/// file persisted on disk.
///
/// These configs should only contain values that are absolutely
/// necessary for startup at a point before we can establish a
/// database connection. All other configs should be defined in
/// the database `config` table for easier modification through
/// the UI.
///
/// These values are all `Option` types since they are intended
/// to assume their default types, and in most cases a config
/// file won't even exist.
///
/// For the "final" struct, see [`Config`].
#[settings(impl_default = false)]
pub struct Configuration {
    pub database_file: String,
    pub port: u16,

    pub addr: SocketAddr,

    pub telemetry: TelemetrySettings,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            database_file: "data/prelearning.db".to_owned(),
            port: 7057,
            addr: Default::default(),
            telemetry: Default::default(),
        }
    }
}

pub fn get_configuration(service_info: &ServiceInfo) -> BootstrapResult<Configuration> {
    let config_path = get_config_path()?;

    let path_arg = std::env::args_os()
        .next()
        .expect("All programs have a 0th argument");

    if !config_path.exists() {
        Cli::<Configuration>::from_os_args(
            service_info,
            vec![],
            vec![
                path_arg.clone(),
                "--generate".into(),
                config_path.clone().into(),
            ],
        )
        .boxed()?;
    }

    let cli = Cli::<Configuration>::from_os_args(
        service_info,
        vec![],
        vec![path_arg, "--config".into(), config_path.into()],
    )?;

    Ok(cli.settings)
}

pub fn get_config_path() -> BootstrapResult<PathBuf> {
    let app_dir = get_app_dir();
    let config_path = app_dir.join("config.toml");

    Ok(config_path)
}
