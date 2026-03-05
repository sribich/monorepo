use std::collections::HashMap;
use std::marker::PhantomData;

use query_core::Operation;
use query_core::Selection;
use query_structure::PrismaValue;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::Model;
use super::WhereInput;
use super::merge_fields;
use crate::client::InternalClient;
use crate::query::ModelOperation;
use crate::query::ModelReadOperation;
use crate::query::Query;
use crate::query::QueryConvert;
use crate::query::QueryError;
use crate::query::QueryResult;
use crate::query::SelectionQuery;
use crate::query::query;
use crate::raw::Raw;
use crate::raw::RawOperationData;
use crate::raw::RawPrismaValue;

pub struct QueryRaw<'db, TData>
where
    TData: DeserializeOwned,
{
    client: &'db InternalClient,
    sql: String,
    params: Vec<Value>,
    _marker: PhantomData<TData>,
}

impl<'db, TData> QueryRaw<'db, TData>
where
    TData: DeserializeOwned + 'static,
{
    #[must_use]
    pub fn new(client: &'db InternalClient, query: Raw, database: &'static str) -> Self {
        let (sql, params) = query.convert(database);

        Self {
            client,
            sql,
            params,
            _marker: PhantomData,
        }
    }

    pub(crate) fn convert(raw: RawOperationData) -> QueryResult<Vec<TData>> {
        raw
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(column_name, cell)| (column_name, cell.into()))
                    .collect::<HashMap<String, RawPrismaValue>>()
            })
            .map(|row| {
                let v = serde_value::to_value(&row)
                    .map_err(|e| e.to_string())
                    .map_err(QueryError::Deserialize)?;

                v.deserialize_into::<TData>()
                    .map_err(|e| e.to_string())
                    .map_err(QueryError::Deserialize)
            })
            .collect::<Result<_, _>>()
    }

    pub async fn exec(self) -> QueryResult<Vec<TData>> {
        query(self).await
    }
}

impl<TData> QueryConvert for QueryRaw<'_, TData>
where
    TData: DeserializeOwned + 'static,
{
    type RawType = RawOperationData;
    type ReturnValue = Vec<TData>;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Self::convert(raw)
    }
}

impl<'a, TData> Query<'a> for QueryRaw<'a, TData>
where
    TData: DeserializeOwned + 'static,
{
    fn operation(self) -> (Operation, &'a InternalClient) {
        (
            Operation::Write(Selection::new(
                "queryRaw".to_owned(),
                None,
                [
                    ("query".to_owned(), PrismaValue::String(self.sql).into()),
                    (
                        "parameters".to_owned(),
                        PrismaValue::String(serde_json::to_string(&self.params).unwrap()).into(),
                    ),
                ],
                [],
            )),
            self.client,
        )
    }
}
