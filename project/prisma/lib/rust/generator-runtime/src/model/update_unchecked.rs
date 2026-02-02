use query_core::{Operation, Selection};
use query_structure::PrismaValue;

use super::{Model, WhereInput, merge_fields};
use crate::{
    client::InternalClient,
    query::{ModelOperation, ModelWriteOperation, Query, QueryConvert, QueryResult, SelectionQuery, query},
};

pub struct UpdateUnchecked<'db, TModel: Model> {
    client: &'db InternalClient,
    params: UpdateUncheckedParams<TModel>,
}

pub struct UpdateUncheckedParams<TModel: Model> {
    r#where: TModel::WhereUnique,
    update: Vec<TModel::UncheckedSet>,
    with: Vec<TModel::With>,
}

impl<'db, TModel: Model> UpdateUnchecked<'db, TModel> {
    #[must_use]
    pub fn new(
        client: &'db InternalClient,
        r#where: TModel::WhereUnique,
        update: Vec<TModel::UncheckedSet>,
        with: Vec<TModel::With>,
    ) -> Self {
        Self {
            client,
            params: UpdateUncheckedParams { r#where, update, with },
        }
    }

    pub async fn exec(self) -> QueryResult<TModel::Data> {
        query(self).await
    }

    fn to_selection(
        where_param: TModel::WhereUnique,
        set_params: Vec<TModel::UncheckedSet>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::selection(
            [
                (
                    "where".to_owned(),
                    PrismaValue::Object(vec![where_param.serialize().transform_equals()]).into(),
                ),
                (
                    "data".to_owned(),
                    PrismaValue::Object(merge_fields(set_params.into_iter().map(Into::into).collect())).into(),
                ),
            ],
            nested_selections,
        )
    }
}

impl<TModel: Model> QueryConvert for UpdateUnchecked<'_, TModel> {
    type RawType = TModel::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for UpdateUnchecked<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(self.params.r#where, self.params.update, selections)),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for UpdateUnchecked<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Update);
}
