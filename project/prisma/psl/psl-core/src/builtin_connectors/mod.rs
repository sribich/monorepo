mod capabilities_support;
pub mod completions;

#[cfg(feature = "mysql")]
mod mysql;
mod native_type_definition;
#[cfg(feature = "postgresql")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

mod utils;

pub use capabilities_support::can_have_capability;
pub use capabilities_support::can_support_relation_load_strategy;
pub use capabilities_support::has_capability;
#[cfg(feature = "mysql")]
pub use mysql::MySqlType;
#[cfg(feature = "postgresql")]
pub use postgres::KnownPostgresType;
#[cfg(feature = "postgresql")]
pub use postgres::PostgresDatasourceProperties;
#[cfg(feature = "postgresql")]
pub use postgres::PostgresType;
#[cfg(feature = "postgresql")]
pub use postgres::SequenceFunction;

use crate::ConnectorRegistry;

#[cfg(feature = "postgresql")]
pub const POSTGRES: &'static dyn crate::datamodel_connector::Connector =
    &postgres::PostgresDatamodelConnector;
#[cfg(feature = "mysql")]
pub const MYSQL: &'static dyn crate::datamodel_connector::Connector =
    &mysql::MySqlDatamodelConnector;
#[cfg(feature = "sqlite")]
pub const SQLITE: &'static dyn crate::datamodel_connector::Connector =
    &sqlite::SqliteDatamodelConnector;

pub static BUILTIN_CONNECTORS: ConnectorRegistry<'static> = &[
    #[cfg(feature = "postgresql")]
    POSTGRES,
    #[cfg(feature = "mysql")]
    MYSQL,
    #[cfg(feature = "sqlite")]
    SQLITE,
];
