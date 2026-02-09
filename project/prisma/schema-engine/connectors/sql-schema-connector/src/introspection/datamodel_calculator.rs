//! Calculate a PSL data model, together with warnings.

mod context;

pub(crate) use context::DatamodelCalculatorContext;
use psl::PreviewFeature;
use psl::parser_database::ExtensionTypes;
use schema_connector::IntrospectionContext;
use schema_connector::IntrospectionResult;
use sql_schema_describer as sql;

use crate::introspection::rendering;
use crate::introspection::warnings;

/// Calculate datamodels from a database schema.
pub fn calculate(
    schema: &sql::SqlSchema,
    ctx: &IntrospectionContext,
    search_path: &str,
    extension_types: &dyn ExtensionTypes,
) -> IntrospectionResult {
    let introspection_file_name = ctx.introspection_file_path();
    let ctx = DatamodelCalculatorContext::new(ctx, schema, search_path, extension_types);

    let (datamodels, is_empty, views) = rendering::to_psl_string(introspection_file_name, &ctx);

    let views = if ctx
        .config
        .preview_features()
        .contains(PreviewFeature::Views)
    {
        Some(views)
    } else {
        None
    };

    let warnings = warnings::generate(&ctx);
    let warnings = match warnings.is_empty() {
        true => None,
        false => Some(warnings.to_string()),
    };

    IntrospectionResult {
        datamodels,
        is_empty,
        warnings,
        views,
    }
}
