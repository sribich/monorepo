use std::fs;
use std::time::Instant;

use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;
use indoc::indoc;
use psl::Schema;
use psl::SchemaParser;
use psl::SourceFile;

#[derive(Parser)]
#[command(name = "format")]
pub struct FormatArgs {
    /// Write the formatted files back to disk
    #[arg(short, long, default_value_t = false)]
    write: bool,
}

pub async fn run(args: FormatArgs) -> Result<()> {
    let start_time = Instant::now();

    let schemas = Schema::new()
        .schema_files()
        .iter()
        .map(|it| (it.path().to_owned(), it.content().to_owned()))
        .collect::<Vec<(String, String)>>();

    let formatted = prisma_fmt::format(
        serde_json::to_string(&schemas).unwrap(),
        indoc! { r#"
            {
                "textDocument": { "uri": "file:/dev/null" },
                "options": {
                    "tabSize": 4,
                    "insertSpaces": true
                }
            }
        "#},
    );

    let output = serde_json::from_str::<Vec<(String, String)>>(&formatted)
        .unwrap()
        .into_iter()
        .map(|it| (it.0, it.1))
        .collect::<Vec<(_, _)>>();

    if args.write {
        for (path, contents) in output {
            fs::write(path, contents).unwrap();
        }

        println!(
            "Formatted {} files in {}ms",
            schemas.len(),
            start_time.elapsed().as_millis()
        );
    } else {
        for (path, contents) in output {
            let Some(original) = schemas.iter().find(|it| it.0 == path) else {
                return Err(anyhow!(
                    "Formatted schema file {} was not found in the original list",
                    path
                ));
            };

            if contents != original.1 {
                eprintln!("There are unformatted files. Run prisma format --write to format them.");

                std::process::exit(1);
            }
        }

        println!("All files are formatted correctly.");
    }

    Ok(())
}
