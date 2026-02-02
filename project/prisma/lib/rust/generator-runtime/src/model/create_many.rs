use query_core::{Operation, Selection};
use query_structure::PrismaValue;

use super::{Model, merge_fields};
use crate::{
    client::InternalClient,
    query::{
        BatchResult, ModelOperation, ModelWriteOperation, Query, QueryConvert, QueryResult, SelectionQuery, query,
    },
};

pub struct CreateMany<'db, TModel: Model> {
    client: &'db InternalClient,
    params: CreateManyParams<TModel>,
}

pub struct CreateManyParams<TModel: Model> {
    set: Vec<Vec<TModel::UncheckedSet>>,
    skip_duplicates: bool,
}

impl<'db, TModel: Model> CreateMany<'db, TModel> {
    #[must_use]
    pub fn new(client: &'db InternalClient, set: Vec<Vec<TModel::UncheckedSet>>) -> Self {
        Self {
            client,
            params: CreateManyParams {
                set,
                skip_duplicates: false,
            },
        }
    }

    #[must_use]
    pub fn skip_duplicates(mut self) -> Self {
        self.params.skip_duplicates = true;
        self
    }

    fn to_selection(
        set_params: Vec<Vec<TModel::UncheckedSet>>,
        skip_duplicates: bool,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::selection(
            [
                Some((
                    "data".to_owned(),
                    PrismaValue::List(
                        set_params
                            .into_iter()
                            .map(|entries| {
                                PrismaValue::Object(merge_fields(entries.into_iter().map(Into::into).collect()))
                            })
                            .collect(),
                    )
                    .into(),
                )),
                skip_duplicates.then(|| {
                    (
                        "skipDuplicates".to_owned(),
                        PrismaValue::Boolean(skip_duplicates).into(),
                    )
                }),
            ]
            .into_iter()
            .flatten(),
            nested_selections,
        )
    }

    pub async fn exec(self) -> QueryResult<i64> {
        query(self).await
    }
}

impl<TModel: Model> QueryConvert for CreateMany<'_, TModel> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw.count)
    }
}

impl<'db, TModel: Model> Query<'db> for CreateMany<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        (
            Operation::Write(Self::to_selection(
                self.params.set,
                self.params.skip_duplicates,
                [BatchResult::selection()],
            )),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for CreateMany<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::CreateMany);
}
