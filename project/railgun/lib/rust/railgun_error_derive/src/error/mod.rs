use r#enum::impl_enum;
use macro_util::ast::Input;
use proc_macro::TokenStream;
use proc_macro_error2::abort;
use r#struct::impl_struct;

mod r#enum;
mod r#struct;

pub fn derive(input: TokenStream) -> TokenStream {
    inner_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

pub fn inner_derive(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse(input)?;
    let input = Input::from_syn(&input)?;

    Ok(match input {
        Input::Struct(input) => impl_struct(input)?,
        Input::Enum(input) => impl_enum(&input)?,
        Input::Union(input) => abort!(input.node, "Unions are not supported as Error types."),
    })
}
