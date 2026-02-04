use std::borrow::Cow;

use super::{
    field::{Field, NamedFields, UnnamedFields},
    generic::GenericType,
};
use crate::id::TypeId;

#[derive(Clone, Debug)]
pub struct StructType {
    pub name: Cow<'static, str>,
    pub id: TypeId,
    pub fields: StructFields,
    pub generics: Vec<GenericType>,
    _private: (),
}

#[derive(Clone, Debug)]
pub enum StructFields {
    Unit,
    Unnamed(UnnamedFields),
    Named(NamedFields),
}

impl StructType {
    pub const fn new(
        name: Cow<'static, str>,
        id: TypeId,
        fields: StructFields,
        generics: Vec<GenericType>,
    ) -> Self {
        Self {
            name,
            id,
            fields,
            generics,
            _private: (),
        }
    }
}

impl StructFields {
    pub const fn new_named(fields: Vec<(Cow<'static, str>, Field)>) -> StructFields {
        StructFields::Named(NamedFields { fields })
    }

    pub const fn new_unnamed(fields: Vec<Field>) -> StructFields {
        StructFields::Unnamed(UnnamedFields { fields })
    }

    pub const fn new_unit() -> StructFields {
        StructFields::Unit
    }
}
