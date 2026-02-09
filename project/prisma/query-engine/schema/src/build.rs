//! Query schema builder. Root for query schema building.

mod enum_types;
mod input_types;
mod mutations;
mod output_types;
mod utils;

pub(crate) use output_types::mutation_type;
pub(crate) use output_types::query_type;
use psl::PreviewFeatures;
use psl::datamodel_connector::ConnectorCapability;
use query_structure::Field as ModelField;
use query_structure::Model;
use query_structure::RelationFieldRef;
use query_structure::TypeIdentifier;

pub use self::enum_types::itx_isolation_levels;
use self::enum_types::*;
pub use self::utils::compound_id_field_name;
pub use self::utils::compound_index_field_name;
use self::utils::*;
use crate::*;

pub fn build(schema: Arc<psl::ValidatedSchema>, enable_raw_queries: bool) -> QuerySchema {
    let preview_features = schema.configuration.preview_features();

    build_with_features(schema, preview_features, enable_raw_queries)
}

pub fn build_with_features(
    schema: Arc<psl::ValidatedSchema>,
    preview_features: PreviewFeatures,
    enable_raw_queries: bool,
) -> QuerySchema {
    let connector = schema.connector;
    let internal_data_model = query_structure::convert(schema);

    QuerySchema::new(
        enable_raw_queries,
        connector,
        preview_features,
        internal_data_model,
    )
}
