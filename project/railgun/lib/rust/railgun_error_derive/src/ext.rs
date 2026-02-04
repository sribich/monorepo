use macro_util::ast::variant::Variant;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{attributes::VariantAttributes, meta::VariantMeta};

pub trait VariantExt {
    fn generic_types(&self) -> TokenStream;

    fn generic_where(&self) -> TokenStream;

    fn name_with_generics(&self, name: Option<Ident>) -> TokenStream;
}

impl<'syn> VariantExt for Variant<'syn, VariantAttributes, VariantMeta<'syn>> {
    fn generic_types(&self) -> TokenStream {
        let values = self
            .meta
            .other_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let generic = format_ident!("__T{}", idx);

                quote!(#generic)
            })
            .collect::<Vec<_>>();

        quote!(<#(#values,)*>)
    }

    fn generic_where(&self) -> TokenStream {
        let fields = self
            .meta
            .other_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let ty = field.ty;
                let generic = format_ident!("__T{}", idx);

                quote!(#generic: ::core::convert::Into<#ty>)
            })
            .collect::<Vec<_>>();

        quote!(#(#fields),*)
    }

    fn name_with_generics(&self, name: Option<Ident>) -> TokenStream {
        let name = name.unwrap_or(self.name.clone());
        let generics = self.generic_types();

        quote!(#name #generics)
    }
}
