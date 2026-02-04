use std::borrow::Cow;

use super::DataType;
use crate::internal::Deprecation;

#[derive(Clone, Debug)]
pub struct Field {
    pub opt: Option<DataType>,
    pub docs: String,
    pub deprecation: Option<Deprecation>,
    pub flatten: bool,
}

#[derive(Clone, Debug)]
pub struct NamedFields {
    pub fields: Vec<(Cow<'static, str>, Field)>,
}

#[derive(Clone, Debug)]
pub struct UnnamedFields {
    pub fields: Vec<Field>,
}

pub const fn field(
    opt: Option<DataType>,
    docs: String,
    deprecation: Option<Deprecation>,
    flatten: bool,
) -> Field {
    Field {
        opt,
        docs,
        deprecation,
        flatten,
    }
}
