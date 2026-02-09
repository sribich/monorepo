use query_core::Operation;
use query_core::Selection;
use query_structure::PrismaValue;

use super::Model;
use super::WhereInput;
use crate::client::InternalClient;
use crate::query::ModelOperation;
use crate::query::ModelWriteOperation;
use crate::query::Query;
use crate::query::QueryConvert;
use crate::query::QueryResult;
use crate::query::SelectionQuery;
use crate::query::query;

pub struct Delete<'db, TModel: Model> {
    client: &'db InternalClient,
    params: DeleteParams<TModel>,
}

pub struct DeleteParams<TModel: Model> {
    r#where: TModel::WhereUnique,
    r#with: Vec<TModel::With>,
}

impl<'db, TModel: Model> Delete<'db, TModel> {
    #[must_use]
    pub fn new(
        client: &'db InternalClient,
        r#where: TModel::WhereUnique,
        r#with: Vec<TModel::With>,
    ) -> Self {
        Self {
            client,
            params: DeleteParams { r#where, r#with },
        }
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

    pub async fn exec(self) -> QueryResult<TModel::Data> {
        query(self).await
    }
}

impl<TModel: Model> QueryConvert for Delete<'_, TModel> {
    type RawType = TModel::Data;
    type ReturnValue = TModel::Data;

    fn convert(raw: Self::RawType) -> QueryResult<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'db, TModel: Model> Query<'db> for Delete<'db, TModel> {
    fn operation(self) -> (Operation, &'db InternalClient) {
        let mut selections = TModel::scalar_selections();

        selections.extend(self.params.with.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(self.params.r#where, selections)),
            self.client,
        )
    }
}

impl<'db, TModel: Model> SelectionQuery<'db> for Delete<'db, TModel> {
    type MODEL = TModel;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Delete);
}
