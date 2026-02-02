use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "db")]
pub struct DbArgs {}

pub async fn run(args: DbArgs) -> Result<()> {
    Ok(())
}
