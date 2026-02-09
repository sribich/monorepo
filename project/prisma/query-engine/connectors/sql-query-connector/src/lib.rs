#![allow(clippy::wrong_self_convention)]
#![deny(unsafe_code)]

mod database;
mod error;
mod query_ext;
mod row;
mod ser_raw;
mod value;

pub use database::FromSource;
#[cfg(feature = "mysql-native")]
pub use database::Mysql;
#[cfg(feature = "postgresql-native")]
pub use database::PostgreSql;
#[cfg(feature = "sqlite-native")]
pub use database::Sqlite;
pub use error::SqlError;
use quaint::prelude::Queryable;

use self::query_ext::QueryExt;
use self::row::*;

type Result<T> = std::result::Result<T, error::SqlError>;
