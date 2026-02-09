use quote::quote;
use syn::ConstParam;
use syn::GenericParam;
use syn::Generics;
use syn::Ident;
use syn::LifetimeParam;
use syn::TypeParam;
use syn::WhereClause;
use syn::parse_quote;

pub fn generics_with_ident(generics: &Generics) -> Option<proc_macro2::TokenStream> {
    if generics.params.is_empty() {
        return None;
    }

    let generics = generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => quote!(#lifetime),
        GenericParam::Type(TypeParam { ident, .. })
        | GenericParam::Const(ConstParam { ident, .. }) => quote!(#ident),
    });

    Some(quote!(<#(#generics),*>))
}

pub fn generics_with_ident_and_bounds(generics: &Generics) -> Option<proc_macro2::TokenStream> {
    if generics.params.is_empty() {
        return None;
    }

    let generics = generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(LifetimeParam {
            lifetime,
            colon_token,
            bounds,
            ..
        }) => quote!(#lifetime #colon_token #bounds),
        GenericParam::Type(TypeParam {
            ident,
            colon_token,
            bounds,
            ..
        }) => quote!(#ident #colon_token #bounds),
        GenericParam::Const(ConstParam {
            const_token,
            ident,
            colon_token,
            ty,
            ..
        }) => quote!(#const_token #ident #colon_token #ty),
    });

    Some(quote!(<#(#generics),*>))
}

pub fn where_clause_with_bounds<F>(generics: &Generics, bound_fn: F) -> Option<WhereClause>
where
    F: Fn(Ident) -> proc_macro2::TokenStream,
{
    let generic_types = generics
        .params
        .iter()
        .filter_map(|generic| match generic {
            GenericParam::Type(ty) => Some(ty.ident.clone()),
            GenericParam::Lifetime(_) | GenericParam::Const(_) => None,
        })
        .map(bound_fn)
        .collect::<Vec<_>>();

    if generic_types.is_empty() {
        return generics.where_clause.clone();
    }

    generics.where_clause.as_ref().map_or_else(
        || {
            Some(parse_quote! {
                where #(#generic_types),*
            })
        },
        |existing| {
            let existing_bounds = existing.predicates.iter();

            Some(parse_quote! {
                where #(#existing_bounds,)* #(#generic_types),*
            })
        },
    )
}

pub fn where_clause_with_type_bound(generics: &Generics) -> Option<WhereClause> {
    if generics
        .params
        .iter()
        .find_map(|generic| match generic {
            GenericParam::Type(ty) => Some(ty.ident.clone()),
            GenericParam::Lifetime(_) | GenericParam::Const(_) => None,
        })
        .is_none()
    {
        return generics.where_clause.clone();
    }

    generics.where_clause.as_ref().map(|existing| {
        let existing_bounds = existing.predicates.iter();

        parse_quote! {
            where #(#existing_bounds,)*
        }
    })
}
