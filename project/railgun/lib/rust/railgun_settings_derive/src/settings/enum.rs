use macro_util::ast::Enum;
use quote::quote;

use super::options::Options;

pub fn impl_enum(options: Options, input: Enum) -> syn::Result<proc_macro2::TokenStream> {
    _ = (options, input);

    Ok(quote! {})
}
