use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "version")]
pub struct VersionArgs;

pub async fn run(args: VersionArgs) -> Result<()> {
    println!("Prisma v{} ({})", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"));

    Ok(())
}
