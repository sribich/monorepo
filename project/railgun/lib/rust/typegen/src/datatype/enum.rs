use std::borrow::Cow;

use super::field::Field;
use super::field::NamedFields;
use super::field::UnnamedFields;
use super::generic::GenericType;
use crate::id::TypeId;
use crate::internal::Deprecation;

#[derive(Clone, Debug)]
pub struct EnumType {
    pub name: Cow<'static, str>,
    pub id: TypeId,
    pub repr: EnumRepr,
    pub generics: Vec<GenericType>,
    pub variants: Vec<(String, EnumVariant)>,
}

impl EnumType {
    pub const fn new(
        name: Cow<'static, str>,
        id: TypeId,
        repr: EnumRepr,
        generics: Vec<GenericType>,
        variants: Vec<(String, EnumVariant)>,
    ) -> Self {
        Self {
            name,
            id,
            repr,
            generics,
            variants,
        }
    }
}

#[derive(Clone, Debug)]
pub enum EnumRepr {
    External,
    Adjacent { tag: String, content: String },
    Internal { tag: String },
    Untagged,
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub skip: bool,
    pub docs: String,
    pub deprecated: Option<Deprecation>,
    pub inner: EnumVariantFields,
}

impl EnumVariant {
    pub const fn new(
        skip: bool,
        docs: String,
        deprecated: Option<Deprecation>,
        inner: EnumVariantFields,
    ) -> Self {
        Self {
            skip,
            docs,
            deprecated,
            inner,
        }
    }
}

#[derive(Clone, Debug)]
pub enum EnumVariantFields {
    Unit,
    Named(NamedFields),
    Unnamed(UnnamedFields),
}

impl EnumVariantFields {
    pub const fn new_named(fields: Vec<(Cow<'static, str>, Field)>) -> EnumVariantFields {
        EnumVariantFields::Named(NamedFields { fields })
    }

    pub const fn new_unnamed(fields: Vec<Field>) -> EnumVariantFields {
        EnumVariantFields::Unnamed(UnnamedFields { fields })
    }

    pub const fn new_unit() -> EnumVariantFields {
        EnumVariantFields::Unit
    }
}
