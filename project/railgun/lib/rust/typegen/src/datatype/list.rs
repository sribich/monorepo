use super::DataType;

#[derive(Clone, Debug)]
pub struct ListMeta {
    /// The type of elements in the list.
    pub inner_type: Box<DataType>,
    /// The length for predefined `[type; size]` arrays.
    pub length: Option<usize>,
    /// Whether the container maintains unique items. Set/HashSet/BTreeSet.
    pub unique: bool,
    // TODO: Do we care about keeping track of the container?
    // pub(crate) container: ListContainer
}

/*
use std::borrow::Cow;

use super::{
    field::{Field, NamedFields, UnnamedFields},
    TypeId,
};

#[derive(Clone, Debug)]
pub struct StructMeta {
    pub name: Cow<'static, str>,
    pub id: TypeId,
    pub fields: StructType,
}

#[derive(Clone, Debug)]
pub enum StructType {
    Unit,
    Unnamed(UnnamedFields),
    Named(NamedFields),
}

pub const fn struct_meta(name: Cow<'static, str>, id: TypeId, inner: StructType) -> StructMeta {
    StructMeta {
        name,
        id,
        fields: inner,
    }
}

pub const fn struct_named(fields: Vec<(Cow<'static, str>, Field)>) -> StructType {
    StructType::Named(NamedFields { fields })
}
*/
