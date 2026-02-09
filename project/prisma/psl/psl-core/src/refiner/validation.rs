use std::sync::Arc;

use diagnostics::Diagnostics;
use parser_database::NoExtensionTypes;
use parser_database::ParserDatabase;
use psl_ast::SourceFile;
use psl_schema::Schema;
use psl_schema::SchemaRefiner;

use super::configuration::Configured;
use crate::Configuration;
use crate::datamodel_connector;

pub struct Validated {}

#[derive(Clone)]
pub struct ValidatedSchema {
    pub configuration: Configuration,
    pub db: parser_database::ParserDatabase,
    pub connector: &'static dyn datamodel_connector::Connector,
    pub diagnostics: Diagnostics,
    pub relation_mode: datamodel_connector::RelationMode,
}

#[derive(Clone, Debug)]
pub struct ValidatedFile {}

impl SchemaRefiner for Validated {
    type FileContext = ValidatedFile;
    type From = Configured;
    type SchemaContext = ValidatedSchema;

    fn refine_context(&self, from: &psl_schema::Schema<Self::From>) -> Self::SchemaContext {
        let diagnostics = from.diagnostics().borrow().clone();

        crate::validate_multi_file(
            &from
                .schema_files()
                .iter()
                .map(|it| (it.path().to_owned(), it.content().into()))
                .collect::<Vec<_>>()[..],
            &NoExtensionTypes,
        )
    }

    fn refine_file(
        &self,
        from: &psl_schema::Schema<Self::From>,
        context: &mut Self::SchemaContext,
        file: &psl_schema::SchemaFile<<Self::From as SchemaRefiner>::FileContext>,
    ) -> Self::FileContext {
        ValidatedFile {}
    }
}

pub trait ValidatedExt {}

impl ValidatedExt for Schema<Validated> {}

pub trait IntoValidatedExt {
    fn into_validated(self) -> Schema<Validated>;
}

impl IntoValidatedExt for Schema<Configured> {
    fn into_validated(self) -> Schema<Validated> {
        self.refine(Validated {})
    }
}
