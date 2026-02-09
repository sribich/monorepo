use std::ops::Not;

use query_core::ArgumentValue;
use query_core::Operation;
use query_core::Selection;
use query_structure::PrismaValue;

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

pub struct FindMany<'db, TModel: Model> {
    client: &'db InternalClient,
    params: FindManyParams<TModel>,
}

/// TODO: Let's not have all of these be pub
#[derive(Clone, Debug)]
pub struct FindManyParams<TModel: Model> {
    pub r#where: Vec<TModel::Where>,
    pub with: Vec<TModel::With>,
    pub order_by: Vec<TModel::OrderBy>,
    pub cursor: Vec<TModel::Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<'db, TModel: Model> FindMany<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, r#where: Vec<TModel::Where>) -> Self {
        Self {
            client,
            params: FindManyParams {
                r#where,
                with: vec![],
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

    pub fn add_with(&mut self, param: impl Into<TModel::With>) {
        self.params.with.push(param.into());
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
    pub fn with(mut self, param: impl Into<TModel::With>) -> Self {
        self.params.with.push(param.into());
        self
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

    pub async fn exec(self) -> QueryResult<Vec<TModel::Data>> {
        query(self).await
    }

    fn to_selection(
        where_params: Vec<TModel::Where>,
        order_by_params: Vec<TModel::OrderBy>,
        cursor_params: Vec<TModel::Cursor>,
        skip: Option<i64>,
        take: Option<i64>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::selection(
            [
                (!where_params.is_empty()).then(|| {
                    (
                        "where".to_owned(),
                        PrismaValue::Object(merge_fields(
                            where_params
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|data| (data.field, data.value.into()))
                                .collect(),
                        ))
                        .into(),
                    )
                }),
                (!order_by_params.is_empty()).then(|| {
                    (
                        "orderBy".to_owned(),
                        PrismaValue::List(
                            order_by_params
                                .into_iter()
                                .map(|param| PrismaValue::Object(vec![param.into()]))
                                .collect(),
                        )
                        .into(),
                    )
                }),
                (!cursor_params.is_empty()).then(|| {
                    (
                        "cursor".to_owned(),
                        PrismaValue::Object(
                            cursor_params
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|data| (data.field, data.value.into()))
                                .collect(),
                        )
                        .into(),
                    )
                }),
                skip.map(|skip| ("skip".to_owned(), PrismaValue::Int(skip).into())),
                take.map(|take| ("take".to_owned(), PrismaValue::Int(take).into())),
            ]
            .into_iter()
            .flatten(),
            nested_selections,
        )
    }
}

impl<TModel: Model> FindManyParams<TModel> {
    #[must_use]
    pub fn new(r#where: Vec<TModel::Where>) -> Self {
        Self {
            r#where,
            with: vec![],
            order_by: vec![],
            cursor: vec![],
            skip: None,
            take: None,
        }
    }

    #[must_use]
    pub fn with(mut self, param: impl Into<TModel::With>) -> Self {
        self.with.push(param.into());
        self
    }

    #[must_use]
    pub fn order_by(mut self, param: TModel::OrderBy) -> Self {
        self.order_by.push(param);
        self
    }

    #[must_use]
    pub fn cursor(mut self, param: TModel::Cursor) -> Self {
        self.cursor.push(param);
        self
    }

    #[must_use]
    pub fn skip(mut self, skip: i64) -> Self {
        self.skip = Some(skip);
        self
    }

    #[must_use]
    pub fn take(mut self, take: i64) -> Self {
        self.take = Some(take);
        self
    }

    #[must_use]
    pub fn to_query_meta(self) -> (Vec<(String, ArgumentValue)>, Vec<Selection>) {
        let arguments = [
            (!self.r#where.is_empty()).then(|| {
                (
                    "where".to_owned(),
                    PrismaValue::Object(
                        self.r#where
                            .into_iter()
                            .map(WhereInput::serialize)
                            .map(Into::into)
                            .collect(),
                    )
                    .into(),
                )
            }),
            (!self.order_by.is_empty()).then(|| {
                (
                    "orderBy".to_owned(),
                    PrismaValue::List(
                        self.order_by
                            .into_iter()
                            .map(|p| PrismaValue::Object(vec![p.into()]))
                            .collect(),
                    )
                    .into(),
                )
            }),
            (!self.cursor.is_empty()).then(|| {
                (
                    "cursor".to_owned(),
                    PrismaValue::Object(
                        self.cursor
                            .into_iter()
                            .map(WhereInput::serialize)
                            .map(|s| (s.field, s.value.into()))
                            .collect(),
                    )
                    .into(),
                )
            }),
            self.skip
                .map(|skip| ("skip".to_owned(), PrismaValue::Int(skip).into())),
            self.take
                .map(|take| ("take".to_owned(), PrismaValue::Int(take).into())),
        ]
        .into_iter()
        .flatten()
        .collect();

        let nested_selections = if self.with.is_empty().not() {
            self.with.into_iter().map(Into::into).collect()
        } else {
            Default::default()
        };

        (arguments, nested_selections)
    }
}

impl<'db, TModel: Model> Query<'db> for FindMany<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let mut nested_selections = TModel::scalar_selections();

        nested_selections.extend(self.params.with.into_iter().map(Into::into));

        let operation = Operation::Read(Self::to_selection(
            self.params.r#where,
            self.params.order_by,
            self.params.cursor,
            self.params.skip,
            self.params.take,
            nested_selections,
        ));

        (operation, self.client)
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for FindMany<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Read(ModelReadOperation::FindMany);
}

impl<TModel: Model> QueryConvert for FindMany<'_, TModel> {
    type RawType = Vec<TModel::Data>;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> crate::query::QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}
