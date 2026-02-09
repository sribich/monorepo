mod command;
mod env;
mod logger;
mod path;
mod slug;
mod url;
mod util;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use clap_verbosity_flag::Verbosity;
use command::db::DbArgs;
use command::format::FormatArgs;
use command::generate::GenerateArgs;
use command::migrate::MigrateCli;
use command::validate::ValidateArgs;
use command::version::VersionArgs;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Command {
    Migrate(MigrateCli),
    Db(DbArgs),
    Generate(GenerateArgs),
    Version(VersionArgs),
    Validate(ValidateArgs),
    Format(FormatArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger();
    // tracing::info!(git_hash = env!("GIT_HASH"), "Starting schema engine RPC server",);

    // let context = SchemaContext::load(None).unwrap();
    // load_envs(&context).unwrap();

    let args = Cli::parse();

    match args.command {
        Command::Migrate(migrate_command) => command::migrate::run(migrate_command).await,
        Command::Db(args) => command::db::run(args).await,
        Command::Generate(args) => command::generate::run(args),
        Command::Version(args) => command::version::run(args).await,
        Command::Validate(args) => command::validate::run(args).await,
        Command::Format(args) => command::format::run(args).await,
    }?;

    Ok(())
}
