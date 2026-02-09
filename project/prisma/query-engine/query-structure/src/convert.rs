use std::sync::Arc;

use crate::InternalDataModel;

pub fn convert(schema: Arc<psl::ValidatedSchema>) -> InternalDataModel {
    InternalDataModel { schema }
}
