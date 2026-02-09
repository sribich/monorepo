use std::borrow::Cow;

use r#enum::EnumType;

use self::generic::GenericType;
use self::list::ListMeta;
use self::primitive::PrimitiveMeta;
use self::reference::ReferenceType;
use self::r#struct::StructType;
use self::tuple::TupleMeta;
use crate::id::TypeId;

pub mod r#enum;
pub mod field;
pub mod generic;
pub mod list;
pub mod primitive;
pub mod reference;
pub mod r#struct;
pub mod tuple;

#[derive(Clone, Debug)]
pub enum DataType {
    Primitive(PrimitiveMeta),

    List(ListMeta),

    Struct(StructType),
    Enum(EnumType),
    Tuple(TupleMeta),

    // Maps
    // Sets
    // Results
    // Options
    Optional(Box<DataType>),
    // ShortCircuit -> Reference,
    Reference(ReferenceType),

    Generic(GenericType),

    Unit,
}

impl DataType {
    pub fn generics(&self) -> Option<&Vec<GenericType>> {
        match self {
            DataType::Struct(item) => Some(&item.generics),
            DataType::Enum(item) => Some(&item.generics),
            DataType::Primitive(_)
            | DataType::List(_)
            | DataType::Tuple(_)
            | DataType::Optional(_)
            | DataType::Reference(_)
            | DataType::Generic(_)
            | DataType::Unit => None,
        }
    }
}

/// Represents a type which carries additional metadata about it, namely
/// its name.
///
/// TODO: Document
#[derive(Clone, Debug)]
pub struct NamedDataType {
    /// The name of the type
    name: Cow<'static, str>,
    /// The id of the type
    id: TypeId,
    /// The datatype
    datatype: DataType,

    doc: String,
    deprecation: Option<crate::internal::Deprecation>,
}

impl NamedDataType {
    pub const fn new(
        name: Cow<'static, str>,
        id: TypeId,
        datatype: DataType,
        doc: String,
        deprecation: Option<crate::internal::Deprecation>,
    ) -> Self {
        Self {
            name,
            id,
            datatype,
            doc,
            deprecation,
        }
    }

    pub fn id(&self) -> &TypeId {
        &self.id
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }

    pub fn datatype(&self) -> &DataType {
        &self.datatype
    }

    pub fn doc(&self) -> &String {
        &self.doc
    }

    pub fn deprecation(&self) -> &Option<crate::internal::Deprecation> {
        &self.deprecation
    }
}
