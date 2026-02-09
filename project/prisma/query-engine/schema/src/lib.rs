#![deny(rust_2018_idioms, unsafe_code)]

pub mod constants;

mod build;
mod enum_type;
mod identifier_type;
mod input_types;
mod output_types;
mod query_schema;
mod utils;

use std::sync::Arc;

pub use self::build::build;
pub use self::build::build_with_features;
pub use self::build::compound_id_field_name;
pub use self::build::compound_index_field_name;
pub use self::build::itx_isolation_levels;
pub use self::enum_type::DatabaseEnumType;
pub use self::enum_type::EnumType;
use self::identifier_type::IdentifierType;
pub use self::input_types::InputField;
pub use self::input_types::InputObjectType;
use self::input_types::InputObjectTypeConstraints;
pub use self::input_types::InputType;
pub use self::input_types::ObjectTag;
pub use self::output_types::InnerOutputType;
pub use self::output_types::ObjectType;
pub use self::output_types::OutputField;
pub use self::output_types::OutputType;
pub use self::query_schema::Identifier;
pub use self::query_schema::QueryInfo;
pub use self::query_schema::QuerySchema;
pub use self::query_schema::QueryTag;
pub use self::query_schema::ScalarType;
use self::utils::*;

pub type QuerySchemaRef = Arc<QuerySchema>;
