use query_core::Operation;
use query_core::Selection;
use query_structure::PrismaValue;
use serde::Deserialize;

use super::Model;
use super::WhereInput;
use super::merge_fields;
use crate::client::InternalClient;
use crate::query::ModelOperation;
use crate::query::ModelReadOperation;
use crate::query::Query;
use crate::query::QueryConvert;
use crate::query::QueryResult;
use crate::query::SelectionQuery;
use crate::query::query;

pub struct Count<'db, TModel: Model> {
    client: &'db InternalClient,
    params: CountParams<TModel>,
}

pub struct CountParams<TModel: Model> {
    r#where: Vec<TModel::Where>,
    order_by: Vec<TModel::OrderBy>,
    cursor: Vec<TModel::Cursor>,
    skip: Option<i64>,
    take: Option<i64>,
}

impl<'db, TModel: Model> Count<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, r#where: Vec<TModel::Where>) -> Self {
        Self {
            client,
            params: CountParams {
                r#where,
                order_by: vec![],
                cursor: vec![],
                skip: None,
                take: None,
            },
        }
    }

    pub fn add_where(&mut self, param: TModel::Where) {
        self.params.r#where.push(param);
    }

    pub fn add_order_by(&mut self, param: TModel::OrderBy) {
        self.params.order_by.push(param);
    }

    pub fn add_cursor(&mut self, param: TModel::Cursor) {
        self.params.cursor.push(param);
    }

    pub fn set_skip(&mut self, param: i64) {
        self.params.skip = Some(param);
    }

    pub fn set_take(&mut self, param: i64) {
        self.params.take = Some(param);
    }

    #[must_use]
    pub fn order_by(mut self, param: TModel::OrderBy) -> Self {
        self.params.order_by.push(param);
        self
    }

    #[must_use]
    pub fn cursor(mut self, param: TModel::Cursor) -> Self {
        self.params.cursor.push(param);
        self
    }

    #[must_use]
    pub fn skip(mut self, skip: i64) -> Self {
        self.params.skip = Some(skip);
        self
    }

    #[must_use]
    pub fn take(mut self, take: i64) -> Self {
        self.params.take = Some(take);
        self
    }

    pub async fn exec(self) -> QueryResult<i64> {
        query(self).await
    }
}

#[derive(Deserialize)]
pub struct CountAggregateResult {
    #[serde(rename = "_count")]
    count: CountResult,
}

#[derive(Deserialize)]
pub struct CountResult {
    #[serde(rename = "_all")]
    all: i64,
}

impl<TModel: Model> QueryConvert for Count<'_, TModel> {
    type RawType = CountAggregateResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw.count.all)
    }
}

impl<'db, TModel: Model> Query<'db> for Count<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let operation = Operation::Read(Self::selection(
            [
                (!self.params.r#where.is_empty()).then(|| {
                    (
                        "where".to_owned(),
                        PrismaValue::Object(merge_fields(
                            self.params
                                .r#where
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|data| (data.field, data.value.into()))
                                .collect(),
                        ))
                        .into(),
                    )
                }),
                (!self.params.order_by.is_empty()).then(|| {
                    (
                        "orderBy".to_owned(),
                        PrismaValue::List(
                            self.params
                                .order_by
                                .into_iter()
                                .map(|param| PrismaValue::Object(vec![param.into()]))
                                .collect(),
                        )
                        .into(),
                    )
                }),
                (!self.params.cursor.is_empty()).then(|| {
                    (
                        "cursor".to_owned(),
                        PrismaValue::Object(
                            self.params
                                .cursor
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|data| (data.field, data.value.into()))
                                .collect(),
                        )
                        .into(),
                    )
                }),
                self.params
                    .skip
                    .map(|skip| ("skip".to_owned(), PrismaValue::Int(skip).into())),
                self.params
                    .take
                    .map(|take| ("take".to_owned(), PrismaValue::Int(take).into())),
            ]
            .into_iter()
            .flatten(),
            [Selection::new(
                "_count",
                None,
                [],
                [Selection::new("_all", None, [], [])],
            )],
        ));

        (operation, self.client)
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for Count<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Read(ModelReadOperation::Count);
}
