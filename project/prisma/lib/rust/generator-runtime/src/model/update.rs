use query_core::{Operation, Selection};
use query_structure::PrismaValue;

use super::{Model, WhereInput, merge_fields};
use crate::{
    client::InternalClient,
    query::{ModelOperation, ModelWriteOperation, Query, QueryConvert, QueryResult, SelectionQuery, query},
};

pub struct Update<'db, TModel: Model> {
    client: &'db InternalClient,
    params: UpdateParams<TModel>,
}

pub struct UpdateParams<TModel: Model> {
    r#where: TModel::WhereUnique,
    update: Vec<TModel::Set>,
    with: Vec<TModel::With>,
}

impl<'db, TModel: Model> Update<'db, TModel> {
    #[must_use]
    pub fn new(
        client: &'db InternalClient,
        r#where: TModel::WhereUnique,
        update: Vec<TModel::Set>,
        with: Vec<TModel::With>,
    ) -> Self {
        Self {
            client,
            params: UpdateParams { r#where, update, with },
        }
    }

    pub async fn exec(self) -> QueryResult<TModel::Data> {
        query(self).await
    }

    fn to_selection(
        where_param: TModel::WhereUnique,
        set_params: Vec<TModel::Set>,
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

impl<TModel: Model> QueryConvert for Update<'_, TModel> {
    type RawType = TModel::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for Update<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(self.params.r#where, self.params.update, selections)),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for Update<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Update);
}
