use std::collections::HashMap;

pub use count::*;
pub use create::*;
pub use create_many::*;
pub use create_unchecked::*;
pub use delete::*;
pub use delete_many::*;
pub use find_first::*;
pub use find_many::*;
pub use find_unique::*;
use query_core::Selection;
use query_structure::PrismaValue;
use serde::de::DeserializeOwned;
pub use update::*;
pub use update_many::*;
pub use update_unchecked::*;
pub use upsert::*;

mod count;
mod create;
mod create_many;
mod create_unchecked;
mod delete;
mod delete_many;
mod find_first;
mod find_many;
mod find_unique;
mod update;
mod update_many;
mod update_unchecked;
mod upsert;

pub trait Model {
    const NAME: &'static str;

    type Where: WhereInput + Send;
    type WhereUnique: WhereInput + Send;

    type Cursor: WhereInput + Send;
    type OrderBy: Into<(String, PrismaValue)> + Send;

    type Data: Data + Send;
    type Set: Into<(String, PrismaValue)> + Send;
    type UncheckedSet: Into<(String, PrismaValue)> + Send;
    type With: Into<Selection> + Send;

    fn scalar_selections() -> Vec<Selection>;
}

pub trait Data: DeserializeOwned + 'static {}

impl<T: DeserializeOwned + 'static> Data for T {}

pub trait WhereInput {
    fn serialize(self) -> SerializedWhereInput;
}

pub enum SerializedWhereValue {
    Object(Vec<(String, query_structure::PrismaValue)>),
    List(Vec<query_structure::PrismaValue>),
    Value(query_structure::PrismaValue),
}

impl From<SerializedWhereValue> for PrismaValue {
    fn from(value: SerializedWhereValue) -> Self {
        match value {
            SerializedWhereValue::Object(v) => Self::Object(v),
            SerializedWhereValue::List(v) => Self::List(v),
            SerializedWhereValue::Value(v) => v,
        }
    }
}

pub struct SerializedWhereInput {
    field: String,
    value: SerializedWhereValue,
}

impl SerializedWhereInput {
    #[must_use]
    pub fn new(field: String, value: SerializedWhereValue) -> Self {
        Self { field, value }
    }

    /// If the parameter is an 'equals' parameter, collapses the value provided
    /// directly into the where clause. This is necessary for unique queries
    /// that have no filters, only direct value comparisons.
    #[must_use]
    pub fn transform_equals(self) -> (String, PrismaValue) {
        let Self { field, value } = self;

        (
            field,
            match value {
                SerializedWhereValue::Object(mut params) => match params
                    .iter()
                    .position(|(key, _)| key == "equals")
                    .map(|i| params.swap_remove(i))
                {
                    Some((_, inner_value)) => inner_value,
                    None => PrismaValue::Object(params),
                },
                SerializedWhereValue::List(values) => PrismaValue::List(values),
                SerializedWhereValue::Value(inner_value) => inner_value,
            },
        )
    }
}

impl From<SerializedWhereInput> for (String, PrismaValue) {
    fn from(from_value: SerializedWhereInput) -> Self {
        let SerializedWhereInput { field, value } = from_value;

        (field, value.into())
    }
}

pub fn merge_fields(fields: Vec<(String, PrismaValue)>) -> Vec<(String, PrismaValue)> {
    let mut merged = HashMap::new();

    for field in fields {
        match (merged.get_mut(&field.0), field.1) {
            (Some(PrismaValue::Object(existing)), PrismaValue::Object(incoming)) => {
                existing.extend(incoming);
            },
            (None, value) => {
                merged.insert(field.0, value);
            },
            (Some(_), _) => {
                panic!("Cannot merge fields that are not objects");
            },
        }
    }

    merged.into_iter().collect()
}
