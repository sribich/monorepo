use std::future::Future;

use futures_util::FutureExt;
use query_core::Operation;
use query_core::Selection;
use query_core::SelectionArgument;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde::de::IntoDeserializer;
use thiserror::Error;

use super::client::InternalClient;
use super::model::Model;

pub trait QueryData: DeserializeOwned + 'static {}

impl<T: DeserializeOwned + 'static> QueryData for T {}

/// Result of createMany
#[derive(Deserialize)]
pub struct BatchResult {
    pub count: i64,
}

impl BatchResult {
    pub fn selection() -> Selection {
        Selection::new("count", None, [], [])
    }
}

pub trait QueryConvert {
    type RawType: QueryData;
    type ReturnValue: QueryData;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue>;
}

pub trait Query<'db>: QueryConvert {
    fn operation(self) -> (Operation, &'db InternalClient);
}

pub trait SelectionQuery<'db>: Query<'db> {
    type MODEL: Model;

    const TYPE: ModelOperation;

    fn selection(
        arguments: impl IntoIterator<Item = SelectionArgument>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Selection::new(
            format!("{}{}", Self::TYPE.name(), Self::MODEL::NAME),
            None,
            arguments.into_iter().collect::<Vec<_>>(),
            nested_selections.into_iter().collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, Error, Serialize)]
pub enum QueryError {
    #[error("Error executing query: {} - {}", .0.as_known().map_or("Unknown".to_owned(), |k| k.error_code.to_string()), .0.message())]
    Execute(user_facing_errors::Error),

    // #[error("Error deserializing value")]
    // Deserialize(color_eyre::eyre::Error),
    #[error("Error deserializing value")]
    Deserialize(String),
}

pub type QueryResult<T> = std::result::Result<T, QueryError>;

pub fn query<'db, TQuery: Query<'db> + 'db>(
    query: TQuery,
) -> impl Future<Output = QueryResult<TQuery::ReturnValue>> + 'db {
    let (operation, client) = query.operation();

    client.execute(operation).map(|value| {
        let value = value?;

        Ok(match client.engine() {
            super::client::ExecutionEngine::Real { .. } => {
                TQuery::RawType::deserialize(value.into_deserializer())
                    .map_err(|e| e.to_string())
                    .map_err(QueryError::Deserialize)
                    .and_then(TQuery::convert)?
            }
        })
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelReadOperation {
    FindUnique,
    FindFirst,
    FindMany,
    Count,
}

impl ModelReadOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FindUnique => "findUnique",
            Self::FindFirst => "findFirst",
            Self::FindMany => "findMany",
            Self::Count => "aggregate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelWriteOperation {
    Create,
    CreateMany,
    Update,
    UpdateMany,
    Delete,
    DeleteMany,
    Upsert,
}

impl ModelWriteOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "createOne",
            Self::CreateMany => "createMany",
            Self::Update => "updateOne",
            Self::UpdateMany => "updateMany",
            Self::Delete => "deleteOne",
            Self::DeleteMany => "deleteMany",
            Self::Upsert => "upsertOne",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelOperation {
    Read(ModelReadOperation),
    Write(ModelWriteOperation),
}

impl ModelOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Read(q) => q.name(),
            Self::Write(q) => q.name(),
        }
    }
}
