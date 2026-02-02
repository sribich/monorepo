use query_core::{Operation, Selection};
use query_structure::PrismaValue;

use super::{Model, WhereInput, merge_fields};
use crate::{
    client::InternalClient,
    query::{
        BatchResult, ModelOperation, ModelWriteOperation, Query, QueryConvert, QueryResult, SelectionQuery, query,
    },
};

pub struct UpdateMany<'db, TModel: Model> {
    client: &'db InternalClient,
    params: UpdateManyParams<TModel>,
}

pub struct UpdateManyParams<TModel: Model> {
    r#where: Vec<TModel::Where>,
    update: Vec<TModel::Set>,
}

impl<'db, TModel: Model> UpdateMany<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, r#where: Vec<TModel::Where>, update: Vec<TModel::Set>) -> Self {
        Self {
            client,
            params: UpdateManyParams { r#where, update },
        }
    }

    pub async fn exec(self) -> QueryResult<i64> {
        query(self).await
    }

    fn to_selection(where_params: Vec<TModel::Where>, set_params: Vec<TModel::Set>) -> Selection {
        Self::selection(
            [
                Some((
                    "data".to_owned(),
                    PrismaValue::Object(merge_fields(set_params.into_iter().map(Into::into).collect())).into(),
                )),
                (!where_params.is_empty()).then(|| {
                    (
                        "where".to_owned(),
                        PrismaValue::Object(merge_fields(
                            where_params
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|s| (s.field, s.value.into()))
                                .collect(),
                        ))
                        .into(),
                    )
                }),
            ]
            .into_iter()
            .flatten(),
            [BatchResult::selection()],
        )
    }
}

impl<TModel: Model> QueryConvert for UpdateMany<'_, TModel> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw.count)
    }
}

impl<'db, TModel: Model> Query<'db> for UpdateMany<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        (
            Operation::Write(Self::to_selection(self.params.r#where, self.params.update)),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for UpdateMany<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::UpdateMany);
}
