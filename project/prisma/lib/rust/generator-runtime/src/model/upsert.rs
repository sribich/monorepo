use query_core::{Operation, Selection};
use query_structure::PrismaValue;

use super::{Model, WhereInput};
use crate::{
    client::InternalClient,
    query::{ModelOperation, ModelWriteOperation, Query, QueryConvert, QueryResult, SelectionQuery, query},
};

pub struct Upsert<'db, TModel: Model> {
    client: &'db InternalClient,
    params: UpsertParams<TModel>,
}

pub struct UpsertParams<TModel: Model> {
    r#where: TModel::WhereUnique,
    create: Vec<TModel::Set>,
    update: Vec<TModel::Set>,
    with: Vec<TModel::With>,
}

impl<'db, TModel: Model> Upsert<'db, TModel> {
    #[must_use]
    pub fn new(
        client: &'db InternalClient,
        r#where: TModel::WhereUnique,
        create: Vec<TModel::Set>,
        update: Vec<TModel::Set>,
    ) -> Self {
        Self {
            client,
            params: UpsertParams {
                r#where,
                create,
                update,
                with: vec![],
            },
        }
    }

    pub async fn exec(self) -> QueryResult<TModel::Data> {
        query(self).await
    }

    fn to_selection(
        where_param: TModel::WhereUnique,
        create_params: Vec<TModel::Set>,
        update_params: Vec<TModel::Set>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::selection(
            [
                (
                    "where".to_owned(),
                    PrismaValue::Object(vec![where_param.serialize().transform_equals()]).into(),
                ),
                (
                    "create".to_owned(),
                    PrismaValue::Object(create_params.into_iter().map(Into::into).collect()).into(),
                ),
                (
                    "update".to_owned(),
                    PrismaValue::Object(update_params.into_iter().map(Into::into).collect()).into(),
                ),
            ],
            nested_selections,
        )
    }
}

impl<TModel: Model> QueryConvert for Upsert<'_, TModel> {
    type RawType = TModel::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for Upsert<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(
                self.params.r#where,
                self.params.create,
                self.params.update,
                selections,
            )),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for Upsert<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Upsert);
}
