use psl_ast::Parsed;
use psl_schema::Schema;
use psl_schema::SchemaFile;
use psl_schema::SchemaRefiner;

use crate::Configuration;
use crate::validate_configuration;

pub struct Configured;

#[derive(Clone)]
pub struct ConfiguredSchema {
    configuration: Configuration,
}

pub struct ConfiguredFile {}

impl SchemaRefiner for Configured {
    type FileContext = ConfiguredFile;
    type From = Parsed;
    type SchemaContext = ConfiguredSchema;

    fn refine_context(&self, from: &Schema<Self::From>) -> Self::SchemaContext {
        ConfiguredSchema {
            configuration: Configuration::default(),
        }
    }

    fn refine_file(
        &self,
        from: &Schema<Self::From>,
        context: &mut Self::SchemaContext,
        file: &SchemaFile<<Self::From as SchemaRefiner>::FileContext>,
    ) -> Self::FileContext {
        let configuration = &mut context.configuration;

        let config =
            validate_configuration(file.context().ast(), &mut from.diagnostics().borrow_mut());
        configuration.extend(config);

        ConfiguredFile {}
    }
}

pub trait ConfiguredExt {
    fn configuration(&self) -> &Configuration;
}

impl ConfiguredExt for Schema<Configured> {
    fn configuration(&self) -> &Configuration {
        &self.context().configuration
    }
}

pub trait IntoConfiguredExt {
    fn into_configured(self) -> Schema<Configured>;
}

impl IntoConfiguredExt for Schema<Parsed> {
    fn into_configured(self) -> Schema<Configured> {
        self.refine(Configured {})
    }
}
