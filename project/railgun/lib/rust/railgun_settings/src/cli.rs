use std::ffi::OsString;

use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use clap::error::ErrorKind;
use railgun_core::ServiceInfo;
use railgun_core::bootstrap::ArgumentErrorContext;
use railgun_core::bootstrap::BootstrapResult;
use railgun_error::Error;
use railgun_error::Location;
use railgun_error::ResultExt;

use crate::Settings;

#[derive(Error)]
#[error(crate_path = "railgun_error")]
pub enum CliError {
    MissingRequiredArgument { location: Location },
}

/// A CLI helper to allow for services to implement basic
/// argument parsing.
///
/// The following options are added by default:
///
///   - `-c`, `--config` - Specifies an existing configuration file.
///   - `-g`, `--generate` - Generates a default configuration file.
///   - `-h`, `--help` - Prints the help information and exits.
///   - `-v`, `--version` - Prints the service version and exits.
///
/// Additional options can be added by passing `args` to [`Cli::new`].
pub struct Cli<S: Settings> {
    /// Parsed service settings.
    pub settings: S,

    /// Parsed [`ArgMatches`] for custom arg parsing which is delegated to
    /// the consumer.
    pub arg_matches: ArgMatches,
}

impl<S: Settings> Cli<S> {
    pub fn new(service_info: &ServiceInfo, custom_args: Vec<Arg>) -> BootstrapResult<Self> {
        Self::from_os_args(service_info, custom_args, std::env::args_os())
    }

    /// Bootstraps a new command line interface for the service with
    /// manually defined arguments.
    ///
    /// This method is useful for services which want to programatically
    /// define their configs on startup.
    pub fn from_os_args(
        service_info: &ServiceInfo,
        custom_args: Vec<Arg>,
        os_args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    ) -> BootstrapResult<Self> {
        let mut command = Command::new(service_info.name)
            .version(service_info.version)
            .author(service_info.author)
            .about(service_info.description)
            .arg(
                Arg::new("config")
                    .required_unless_present("generate")
                    .action(ArgAction::Set)
                    .long("config")
                    .short('c')
                    .help("Specifies the path to the configuration file to run the service with"),
            )
            .arg(
                Arg::new("generate")
                    .action(ArgAction::Set)
                    .long("generate")
                    .short('g')
                    .help("Generates a new default configuration file for the service"),
            );

        for arg in custom_args {
            command = command.arg(arg);
        }

        let arg_matches = get_arg_matches(command, os_args)?;
        let settings = get_settings(&arg_matches)?;

        Ok(Self {
            settings,
            arg_matches,
        })
    }
}

fn get_arg_matches(
    command: Command,
    os_args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
) -> BootstrapResult<ArgMatches> {
    command
        .try_get_matches_from(os_args)
        .map_err(|err| {
            let kind = err.kind();

            if kind == ErrorKind::DisplayHelp || kind == ErrorKind::DisplayVersion {
                err.exit()
            }

            err.into()
            // Box::new(err) as Box<dyn core::error::Error + Send + Sync>
        })
        .context(ArgumentErrorContext {})
}

fn get_settings<S: Settings>(arg_matches: &ArgMatches) -> BootstrapResult<S> {
    if let Some(path) = arg_matches.get_one::<String>("generate") {
        let settings = S::default();

        crate::to_file(&settings, path)?;

        return Ok(settings);
    }

    if let Some(path) = arg_matches.get_one::<String>("config") {
        return crate::from_file(path);
    }

    unreachable!("clap should require one of these options be present")
}

#[cfg(test)]
mod test {

    use railgun_core::service_info;
    use railgun_settings_derive::settings;

    use super::Cli;

    #[settings(crate_path = "crate")]
    struct TestSettings {
        name: String,
    }

    #[test]
    fn ensure_setting_argument_is_required() {
        let service_info = service_info!();

        let cli: Cli<TestSettings> =
            Cli::from_os_args(&service_info, vec![], vec!["unknown_value"]).unwrap();
    }
}
