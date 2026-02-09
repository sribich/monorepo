use query_core::Operation;
use query_structure::PrismaValue;

use super::Model;
use super::WhereInput;
use super::merge_fields;
use crate::client::InternalClient;
use crate::query::BatchResult;
use crate::query::ModelOperation;
use crate::query::ModelWriteOperation;
use crate::query::Query;
use crate::query::QueryConvert;
use crate::query::QueryResult;
use crate::query::SelectionQuery;
use crate::query::query;

pub struct DeleteMany<'db, TModel: Model> {
    client: &'db InternalClient,
    params: DeleteManyParams<TModel>,
}

pub struct DeleteManyParams<TModel: Model> {
    r#where: Vec<TModel::Where>,
}

impl<'db, TModel: Model> DeleteMany<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, r#where: Vec<TModel::Where>) -> Self {
        Self {
            client,
            params: DeleteManyParams { r#where },
        }
    }

    pub async fn exec(self) -> QueryResult<i64> {
        query(self).await
    }
}

impl<TModel: Model> QueryConvert for DeleteMany<'_, TModel> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw.count)
    }
}

impl<'db, TModel: Model> Query<'db> for DeleteMany<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        (
            Operation::Write(Self::selection(
                [(!self.params.r#where.is_empty()).then(|| {
                    (
                        "where".to_string(),
                        PrismaValue::Object(merge_fields(
                            self.params
                                .r#where
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|s| (s.field, s.value.into()))
                                .collect(),
                        ))
                        .into(),
                    )
                })]
                .into_iter()
                .flatten(),
                [BatchResult::selection()],
            )),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for DeleteMany<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::DeleteMany);
}
