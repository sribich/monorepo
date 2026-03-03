mod args;
mod config;
mod dmmf;
mod error;
mod jsonrpc;
mod rust;

use std::sync::Arc;

use args::GeneratorArgs;
#[rustfmt::skip]
use ::dmmf::DataModelMetaFormat;
pub use error::PrismaError;
pub use error::Result;
use psl::Schema;
use psl::Validated;
use railgun_error::ResultExt;
pub use rust::RustGenerator;

pub trait Generator {
    fn name(&self) -> &'static str;
    fn default_output(&self) -> &'static str;

    fn generate(&self, args: GeneratorArgs) -> Result<()>;
}

pub struct GeneratorContext {
    generator: Box<dyn Generator>,
}

impl GeneratorContext {
    pub fn new(generator: Box<dyn Generator>) -> Self {
        Self { generator }
    }

    pub fn run(
        self,
        config: psl::Generator,
        schema: Arc<Schema<Validated>>,
        dmmf: Arc<DataModelMetaFormat>,
    ) -> Result<()> {
        self.generator
            .generate(GeneratorArgs::new(config, dmmf, schema))?;

        Ok(())
    }
}

pub fn run_generators(
    schema: Arc<Schema<Validated>>,
    dmmf: Arc<DataModelMetaFormat>,
) -> Result<()> {
    for generator in &schema.context().configuration.generators {
        run_generator(generator, Arc::clone(&schema), Arc::clone(&dmmf))?;
    }

    Ok(())
}

fn run_generator(
    generator: &psl::Generator,
    schema: Arc<Schema<Validated>>,
    dmmf: Arc<DataModelMetaFormat>,
) -> Result<()> {
    let provider = generator.provider.value().unwrap();

    let boxed_generator: Box<dyn Generator> = match &provider[..] {
        "rust" => Box::new(RustGenerator),
        _ => panic!("Unknown generator '{provider}'"),
    };

    GeneratorContext::new(boxed_generator).run(generator.clone(), schema, dmmf)?;

    Ok(())
}
