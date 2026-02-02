use query_core::{Operation, Selection};
use query_structure::PrismaValue;

use super::{Model, WhereInput};
use crate::{
    client::InternalClient,
    query::{ModelOperation, ModelReadOperation, Query, QueryConvert, QueryResult, SelectionQuery, query},
};

pub struct FindUnique<'db, TModel: Model> {
    client: &'db InternalClient,
    params: FindUniqueParams<TModel>,
}

pub struct FindUniqueParams<TModel: Model> {
    r#where: TModel::WhereUnique,
    with: Vec<TModel::With>,
}

impl<'db, TModel: Model> FindUnique<'db, TModel> {
    pub fn new(client: &'db InternalClient, r#where: TModel::WhereUnique) -> Self {
        Self {
            client,
            params: FindUniqueParams { r#where, with: vec![] },
        }
    }

    #[must_use]
    pub fn with(mut self, param: impl Into<TModel::With>) -> Self {
        self.params.with.push(param.into());
        self
    }

    pub async fn exec(self) -> QueryResult<Option<TModel::Data>> {
        query(self).await
    }

    fn to_selection(
        where_param: TModel::WhereUnique,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::selection(
            [(
                "where".to_owned(),
                PrismaValue::Object(vec![where_param.serialize().transform_equals()]).into(),
            )],
            nested_selections,
        )
    }
}

impl<TModel: Model> QueryConvert for FindUnique<'_, TModel> {
    type RawType = Option<TModel::Data>;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for FindUnique<'db, TModel> {
    fn operation(self) -> (query_core::Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Read(Self::to_selection(self.params.r#where, selections)),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for FindUnique<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Read(ModelReadOperation::FindUnique);
}

impl<TModel: Model> FindUniqueParams<TModel> {
    #[must_use]
    pub fn new(r#where: TModel::WhereUnique) -> Self {
        Self { r#where, with: vec![] }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FindUniqueArgs<TModel: Model> {
    pub with_params: Vec<TModel::With>,
}

impl<TModel: Model> FindUniqueArgs<TModel> {
    pub fn new() -> Self {
        Self { with_params: vec![] }
    }

    pub fn with(mut self, with: impl Into<TModel::With>) -> Self {
        self.with_params.push(with.into());
        self
    }
}
