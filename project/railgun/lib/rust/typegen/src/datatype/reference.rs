use std::borrow::Cow;

use super::{DataType, generic::GenericType};
use crate::{
    Generics, NamedType, Type,
    cache::{CachedType, TypeCache},
    id::TypeId,
};

#[derive(Clone, Debug)]
pub struct Reference {
    pub inner: DataType,
    _private: (),
}

impl Reference {
    pub fn new<T: Type + ?Sized>(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
        Reference {
            inner: T::datatype(cache, &Generics::Concrete(generics)),
            _private: (),
        }
    }

    pub fn new_named<T: NamedType>(cache: &mut TypeCache, reference: ReferenceType) -> Reference {
        if !cache.cache.contains_key(&T::ID) {
            cache.cache.entry(T::ID).or_insert(CachedType::InProgress);

            let def = T::named_datatype(cache, &Generics::Impl);

            cache.cache.insert(T::ID, CachedType::Resolved(def));
        }

        Reference {
            inner: DataType::Reference(reference),
            _private: (),
        }
    }
}

impl From<DataType> for Reference {
    fn from(value: DataType) -> Self {
        Reference {
            inner: value,
            _private: (),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReferenceType {
    pub name: Cow<'static, str>,
    pub id: TypeId,
    pub generics: Vec<(GenericType, DataType)>,
}

impl ReferenceType {
    pub fn new<T: Into<String>>(
        name: T,
        generics: Vec<(GenericType, DataType)>,
        id: TypeId,
    ) -> Self {
        Self {
            name: Cow::Owned(name.into()),
            generics,
            id,
        }
    }
}

/*
/// A reference to a [`DataType`] that can be used before a type is resolved in order to
/// support recursive types without causing an infinite loop.
///
/// This works since a child type that references a parent type does not care about the
/// parent's fields, only really its name. Once all of the parent's fields have been
/// resolved will the parent's definition be placed in the type map.
///
// This doesn't account for flattening and inlining recursive types, however, which will
// require a more complex solution since it will require multiple processing stages.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceType {
    pub(crate) name: Cow<'static, str>,
    pub(crate) sid: SpectaID,
    // pub(crate) generics: Vec<(GenericType, DataType)>,
}

impl ReferenceType {
    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }

    pub fn sid(&self) -> SpectaID {
        self.sid
    }

    /*
    pub fn generics(&self) -> &Vec<(GenericType, DataType)> {
        &self.generics
    }
    */
}
*/
