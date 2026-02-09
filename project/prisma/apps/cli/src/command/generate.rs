use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use dmmf::ValidatedSchemaDmmfExt;
use dmmf::dmmf_from_validated_schema;
use generator::GeneratorContext;
use generator::RustGenerator;
use generator::run_generators;
use psl::ConfiguredExt;
use psl::IntoConfiguredExt;
use psl::IntoValidatedExt;
use psl::Schema;
use psl::SchemaParser;

#[derive(Parser)]
#[command(name = "generate")]
pub struct GenerateArgs {}

pub fn run(args: GenerateArgs) -> Result<()> {
    let schema = Schema::new().parse().into_configured().into_validated();

    // Eat the cost of running schema stuff twice for now.
    let dmmf = dmmf_from_validated_schema(Arc::new(
        Schema::new()
            .parse()
            .into_configured()
            .into_validated()
            .into_context(),
    ));

    run_generators(Arc::new(schema), Arc::new(dmmf)).unwrap();

    /*
    let generator = RustGenerator {};

    GeneratorContext::new(Box::new(generator)).run(schema, dmmf);
     */

    Ok(())
}
