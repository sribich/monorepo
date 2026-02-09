use query_core::Operation;
use query_core::Selection;
use query_structure::PrismaValue;

use super::Model;
use super::merge_fields;
use crate::client::InternalClient;
use crate::query::ModelOperation;
use crate::query::ModelWriteOperation;
use crate::query::Query;
use crate::query::QueryConvert;
use crate::query::QueryResult;
use crate::query::SelectionQuery;
use crate::query::query;

pub struct CreateUnchecked<'db, TModel: Model> {
    client: &'db InternalClient,
    params: CreateUncheckedParams<TModel>,
}

pub struct CreateUncheckedParams<TModel: Model> {
    set: Vec<TModel::UncheckedSet>,
    with: Vec<TModel::With>,
}

impl<'db, TModel: Model> CreateUnchecked<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, set: Vec<TModel::UncheckedSet>) -> Self {
        Self {
            client,
            params: CreateUncheckedParams { set, with: vec![] },
        }
    }

    pub fn add_set(&mut self, param: TModel::UncheckedSet) {
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

    fn to_selection(
        set_params: Vec<TModel::UncheckedSet>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::selection(
            [(
                "data".to_owned(),
                PrismaValue::Object(merge_fields(
                    set_params.into_iter().map(Into::into).collect(),
                ))
                .into(),
            )],
            nested_selections,
        )
    }

    pub async fn exec(self) -> QueryResult<TModel::Data> {
        query(self).await
    }
}

impl<TModel: Model> QueryConvert for CreateUnchecked<'_, TModel> {
    type RawType = TModel::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for CreateUnchecked<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(self.params.set, selections)),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for CreateUnchecked<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Create);
}
