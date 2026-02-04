mod settings;

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro_error2::proc_macro_error;
use proc_macro2::Span;
use syn::{Ident, Path, parse_quote};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn settings(args: TokenStream, item: TokenStream) -> TokenStream {
    settings::expand(args, item)
}

/// Returns the path required to access the crate from the generated code.
///
/// This will likely be `railgun`, but if it is an internal package then
/// it might be `railgun_settings`.
pub(crate) fn get_crate_path(span: Span) -> syn::Result<Path> {
    let railgun_crate = crate_name("railgun");

    if let Ok(found) = railgun_crate {
        return Ok(match found {
            FoundCrate::Itself => parse_quote!(::railgun::settings),
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                parse_quote!(::#ident::settings)
            },
        });
    }

    let railgun_settings_crate = crate_name("railgun_settings");

    if let Ok(found) = railgun_settings_crate {
        return Ok(match found {
            FoundCrate::Itself => parse_quote!(::railgun_settings),
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                parse_quote!(::#ident)
            },
        });
    }

    Err(syn::Error::new(
        span,
        "Could not find a suitable crate import when expanding the macro. \
Pleasure ensure that either:

    1. \"railgun\" or \"railgun_settings\" is imported.
    2. \"#[settings(crate_path = \"path_to_error_export\")]\" is used.
",
    ))
}
