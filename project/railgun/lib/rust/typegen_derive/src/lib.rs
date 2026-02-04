//! This crate contains the macros required to associate type data with
//! their associated types so that we can export them during runtime to
//! provide cross-language type safety.
//!
//! This is done by representing the type in an "intermediary" type that
//! we can refer to at runtime in order to build the required output for
//! the target language.
//!
//! A majority of the logic is in the sibling crate `typegen,` including
//! the intermediary type we will be building in this crate.
mod attributes;
mod typegen;
mod util;

use proc_macro::TokenStream;

/// Implements [`Type`] and [`NamedType`] on a struct or enum.
///
/// # Example
///
/// ```rust
/// use typegen::Typegen;
///
/// #[derive(Typegen)]
/// pub struct Person {
///     name: String,
///     age: u32,
/// }
///
/// #[derive(Typegen)]
/// pub struct Count(u64);
///
/// #[derive(Typegen)]
/// pub struct SomeTuple(u32, String);
///
/// #[derive(Typegen)]
/// pub enum Enum {
///     Unit,
///     Unnamed(String),
///     Named { name: String },
/// }
/// ```
#[proc_macro_derive(Typegen, attributes(serde, typegen))]
pub fn derive_typegen(input: TokenStream) -> TokenStream {
    typegen::derive(input)
}
