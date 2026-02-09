#![deny(unsafe_code, rust_2018_idioms)]

#[macro_use]
extern crate tracing;

pub mod constants;
pub mod executor;
pub mod protocol;
pub mod query_document;
pub mod query_graph_builder;
pub mod relation_load_strategy;
pub mod response_ir;

pub use connector::Connector;
pub use connector::error::ConnectorError;
pub use connector::error::ErrorKind as ConnectorErrorKind;

pub use self::error::CoreError;
pub use self::error::ExtendedUserFacingError;
pub use self::error::FieldConversionError;
pub use self::executor::QueryExecutor;
pub use self::executor::TransactionOptions;
pub use self::executor::with_sync_unevaluated_request_context;
pub use self::interactive_transactions::TransactionError;
pub use self::interactive_transactions::TxId;
pub use self::query_ast::*;
pub use self::query_document::*;
pub use self::query_graph::*;
pub use self::query_graph_builder::*;

mod error;
mod interactive_transactions;
mod interpreter;
mod metrics;
mod query_ast;
mod query_graph;
mod result_ast;

use self::executor::*;
use self::interactive_transactions::*;
use self::interpreter::Env;
use self::interpreter::ExpressionResult;
use self::interpreter::Expressionista;
use self::interpreter::InterpreterError;
use self::interpreter::QueryInterpreter;
use self::response_ir::IrSerializer;
use self::response_ir::ResponseData;
use self::result_ast::*;

/// Result type tying all sub-result type hierarchies of the core together.
pub type Result<T> = std::result::Result<T, CoreError>;

// Re-exports
pub use schema;
