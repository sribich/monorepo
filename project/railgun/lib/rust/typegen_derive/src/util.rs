use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::Ident;

pub(crate) fn get_crate_name() -> proc_macro2::TokenStream {
    let railgun_crate = crate_name("railgun");
    let typegen_crate = crate_name("typegen");

    match typegen_crate {
        Ok(FoundCrate::Itself) => return quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            return quote!(::#ident);
        },
        Err(_) => (),
    }

    match railgun_crate {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident::typegen)
        },
        Err(_) => quote!(typegen),
    }
}
