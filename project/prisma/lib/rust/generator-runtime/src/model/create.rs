use query_core::{Operation, Selection};
use query_structure::PrismaValue;

use crate::{
    client::InternalClient,
    query::{ModelOperation, ModelWriteOperation, Query, QueryConvert, QueryResult, SelectionQuery, query},
};

use super::{Model, merge_fields};

pub struct Create<'db, TModel: Model> {
    client: &'db InternalClient,
    params: CreateParams<TModel>,
}

pub struct CreateParams<TModel: Model> {
    set: Vec<TModel::Set>,
    with: Vec<TModel::With>,
}

impl<'db, TModel: Model> Create<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, set: Vec<TModel::Set>) -> Self {
        Self {
            client,
            params: CreateParams { set, with: vec![] },
        }
    }

    pub fn add_set(&mut self, param: TModel::Set) {
        self.params.set.push(param);
    }

    pub fn add_with(&mut self, param: impl Into<TModel::With>) {
        self.params.with.push(param.into());
    }

    #[must_use]
    pub fn with(mut self, param: impl Into<TModel::With>) -> Self {
        self.params.with.push(param.into());
        self
    }

    fn to_selection(set_params: Vec<TModel::Set>, nested_selections: impl IntoIterator<Item = Selection>) -> Selection {
        Self::selection(
            [(
                "data".to_owned(),
                PrismaValue::Object(merge_fields(set_params.into_iter().map(Into::into).collect())).into(),
            )],
            nested_selections,
        )
    }

    pub async fn exec(self) -> QueryResult<TModel::Data> {
        query(self).await
    }
}

impl<TModel: Model> QueryConvert for Create<'_, TModel> {
    type RawType = TModel::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for Create<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(self.params.set, selections)),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for Create<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Create);
}
