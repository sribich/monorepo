mod attributes;
mod error;
mod ext;
mod meta;

use proc_macro::TokenStream;
use proc_macro_crate::FoundCrate;
use proc_macro_crate::crate_name;
use proc_macro_error2::proc_macro_error;
use proc_macro2::Span;
use syn::Ident;
use syn::Path;
use syn::parse_quote;

#[proc_macro_error]
#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    error::derive(input)
}

/// Returns the path required to access the crate from the generated code.
///
/// This will likely be `railgun`, but if it is an internal package then
/// it might be `railgun_error`.
pub(crate) fn get_crate_path(span: Span) -> syn::Result<Path> {
    let railgun_crate = crate_name("railgun");

    if let Ok(found) = railgun_crate {
        return Ok(match found {
            FoundCrate::Itself => parse_quote!(::railgun::error),
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                parse_quote!(::#ident::error)
            }
        });
    }

    let railgun_error_crate = crate_name("railgun_error");

    if let Ok(found) = railgun_error_crate {
        return Ok(match found {
            FoundCrate::Itself => parse_quote!(::railgun_error),
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                parse_quote!(::#ident)
            }
        });
    }

    Err(syn::Error::new(
        span,
        "Could not find a suitable crate import when expanding the macro. \
Pleasure ensure that either:

    1. \"railgun\" or \"railgun_error\" is imported.
    2. \"#[error(crate_path = \"path_to_error_export\")]\" is used.
",
    ))
}
