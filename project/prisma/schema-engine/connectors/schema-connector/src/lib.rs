#![deny(rust_2018_idioms, unsafe_code, missing_docs)]

//! This crate defines the API exposed by the connectors to the schema engine core. The entry point for this API is the [MigrationConnector](trait.MigrationConnector.html) trait.

mod checksum;
mod connector_host;
mod connector_params;
mod database_schema;
mod destructive_change_checker;
mod diff;
mod error;
mod introspect_sql;
mod introspection_context;
mod introspection_result;
mod migration;
mod migration_persistence;
mod namespaces;
mod schema_connector;

pub mod migrations_directory;
pub mod warnings;

pub use connector_host::ConnectorHost;
pub use connector_host::EmptyHost;
pub use connector_params::ConnectorParams;
pub use database_schema::DatabaseSchema;
pub use destructive_change_checker::DestructiveChangeChecker;
pub use destructive_change_checker::DestructiveChangeDiagnostics;
pub use destructive_change_checker::MigrationWarning;
pub use destructive_change_checker::UnexecutableMigration;
pub use diff::DiffTarget;
pub use error::ConnectorError;
pub use error::ConnectorResult;
pub use introspect_sql::*;
pub use introspection_context::IntrospectionContext;
pub use introspection_result::IntrospectionResult;
pub use introspection_result::ViewDefinition;
pub use migration::Migration;
pub use migration_persistence::MigrationPersistence;
pub use migration_persistence::MigrationRecord;
pub use migration_persistence::PersistenceNotInitializedError;
pub use migration_persistence::Timestamp;
pub use warnings::Warnings;

pub use crate::namespaces::Namespaces;
pub use crate::schema_connector::ExternalShadowDatabase;
pub use crate::schema_connector::SchemaConnector;
pub use crate::schema_connector::SchemaDialect;

/// Alias for a pinned, boxed future, used by the traits.
pub type BoxFuture<'a, O> = std::pin::Pin<Box<dyn std::future::Future<Output = O> + Send + 'a>>;
