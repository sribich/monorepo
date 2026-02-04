mod component;
mod take;

use proc_macro::TokenStream;

#[proc_macro_derive(Component, attributes(component, inject))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    component::derive(input)
}

#[proc_macro_derive(Take)]
pub fn derive_take(input: TokenStream) -> TokenStream {
    take::derive(input)
}
