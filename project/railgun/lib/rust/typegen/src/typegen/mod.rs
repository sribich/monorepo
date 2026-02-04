pub mod cache;
pub mod id;
pub mod impls;


use self::{cache::TypeCache, id::TypeId};
use crate::datatype::{reference::Reference, DataType, NamedDataType};

#[derive(Debug)]
pub enum Generics<'a> {
    /// Defines the raw generic type
    Impl,
    /// The types that exist from a concrete instantiation
    /// of a generic type.
    ///
    /// ```
    /// struct Foo<A>(A);
    ///
    /// struct Bar {
    ///     field: Foo<u32>, // ^^^ u32 is the concrete type
    /// }
    /// ```
    Concrete(&'a [DataType]),
}

/// Defines type information that can be used at runtime.
///
/// Types implementing this trait are intended to be fed into
/// a [`Generator`] in order to create type-aware clients for
/// use in other languages.
///
/// This trait should not be implemented directly, but should
/// instead be utilized through the [`Typegen`] macro.
///
/// [`Generator`]: typegen::generator::Generator
/// [`Typegen`]: typegen::Typegen
pub trait Type {
    /// TODO: Document
    fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType;

    /// TODO: Document
    fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
        Reference::new::<Self>(cache, generics)
    }
}

/// A named type represents a concrete type that has been
/// annotated with `#[derive(Typegen)]`.
///
/// These types have special properties.
///
/// TODO: Document
pub trait NamedType: Type {
    const ID: TypeId;

    fn named_datatype(cache: &mut TypeCache, generics: &Generics) -> NamedDataType;
}
