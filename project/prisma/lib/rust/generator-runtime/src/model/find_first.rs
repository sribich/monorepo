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

pub struct FindFirst<'db, TModel: Model> {
    client: &'db InternalClient,
    params: FindFirstParams<TModel>,
}

pub struct FindFirstParams<TModel: Model> {
    r#where: Vec<TModel::Where>,
    with: Vec<TModel::With>,
    order_by: Vec<TModel::OrderBy>,
    cursor: Vec<TModel::Cursor>,
    skip: Option<i64>,
    take: Option<i64>,
}

impl<'db, TModel: Model> FindFirst<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, where_params: Vec<TModel::Where>) -> Self {
        Self {
            client,
            params: FindFirstParams {
                r#where: where_params,
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

    pub async fn exec(self) -> QueryResult<Option<TModel::Data>> {
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

impl<TModel: Model> QueryConvert for FindFirst<'_, TModel> {
    type RawType = Option<TModel::Data>;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for FindFirst<'db, TModel> {
    fn operation(self) -> (query_core::Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Read(Self::to_selection(
                self.params.r#where,
                self.params.order_by,
                self.params.cursor,
                self.params.skip,
                self.params.take,
                selections,
            )),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for FindFirst<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Read(ModelReadOperation::FindFirst);
}
