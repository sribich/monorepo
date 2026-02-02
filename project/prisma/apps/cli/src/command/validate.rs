use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use prisma_fmt::lint;
use psl::Schema;
use tracing::Instrument;

#[derive(Parser)]
#[command(name = "validate")]
pub struct ValidateArgs;

pub async fn run(args: ValidateArgs) -> Result<()> {
    let start_time = Instant::now();

    let schemas = Schema::new()
        .schema_files()
        .iter()
        .map(|it| (it.path().to_owned(), it.content().to_owned()))
        .collect::<Vec<(String, String)>>();

    let result = lint(serde_json::to_string(&schemas).unwrap());

    for error in result {
        println!("{}", error.text);
    }

    Ok(())
}
