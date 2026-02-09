use darling::ast::NestedMeta;
use proc_macro_error2::abort;
use quote::TokenStreamExt;
use quote::quote;
use quote::quote_spanned;
use syn::Attribute;
use syn::Expr;
use syn::ExprLit;
use syn::Field;
use syn::Fields;
use syn::Ident;
use syn::ItemStruct;
use syn::Lit;
use syn::LitStr;
use syn::Meta;
use syn::MetaNameValue;
use syn::Path;
use syn::parse_quote;
use syn::spanned::Spanned;

use super::options::Options;
use crate::get_crate_path;

pub fn impl_struct(
    options: Options,
    mut input: ItemStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let crate_path = match &options.crate_path {
        Some(path) => path,
        None => &get_crate_path(input.span())?,
    };

    Ok(match input.fields {
        Fields::Named(_) => {
            add_default_attrs(&options, &crate_path, &mut input.attrs);

            // Use the default values when the configuration file
            // is missing values.
            input.attrs.push(parse_quote!(#[serde(default)]));

            let impl_settings = impl_settings(&options, &crate_path, &input)?;

            let impl_default = if options.impl_default {
                impl_serde_aware_default(&input)
            } else {
                quote!()
            };

            quote! {
                #input

                #impl_settings
                #impl_default
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                #input
            }
        }
        Fields::Unit => abort!(input, "Unit structs are not valid settings objects"),
    })
}

fn add_default_attrs(options: &Options, crate_path: &Path, attrs: &mut Vec<Attribute>) {
    let serde_path = quote!(#crate_path::_internal_for_macros_::serde).to_string();

    attrs.push(parse_quote!(
        #[derive(
            Clone,
            #crate_path::_internal_for_macros_::serde::Serialize,
            #crate_path::_internal_for_macros_::serde::Deserialize,
        )]
    ));

    if options.impl_debug {
        attrs.push(parse_quote!(#[derive(Debug)]));
    }

    attrs.push(parse_quote!(#[serde(crate = #serde_path)]));
}

fn impl_settings(
    options: &Options,
    crate_path: &Path,
    item: &ItemStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let ident = item.ident.clone();

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let mut doc_comments_impl = quote! {};

    for field in &item.fields {
        if let Some(name) = &field.ident {
            let impl_for_field = impl_settings_for_field(options, crate_path, field, name);

            doc_comments_impl.append_all(impl_for_field);
        }
    }

    Ok(quote! {
        impl #impl_generics #crate_path::Settings for #ident #ty_generics #where_clause {
            fn add_docs(
                &self,
                parent_key: &[String],
                docs: &mut ::std::collections::HashMap<Vec<String>, &'static [&'static str]>)
            {
                #doc_comments_impl
            }
        }
    })
}

fn impl_settings_for_field(
    options: &Options,
    crate_path: &Path,
    field: &Field,
    name: &Ident,
) -> proc_macro2::TokenStream {
    let span = field.ty.span();
    let name_str = name.to_string();
    let docs = extract_doc_comments(&field.attrs);

    let mut impl_for_field = quote! {};

    let cfg_attrs = field
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("cfg"))
        .collect::<Vec<_>>();

    impl_for_field.append_all(quote_spanned! { span=>
        let mut key = parent_key.to_vec();

        key.push(#name_str.into());

        #crate_path::Settings::add_docs(&self.#name, &key, docs);
    });

    if !docs.is_empty() {
        impl_for_field.append_all(quote! {
            docs.insert(key, &[#(#docs,)*][..]);
        });
    }

    if !cfg_attrs.is_empty() {
        impl_for_field = quote! {
            #(#cfg_attrs)*
            {
                #impl_for_field
            }
        }
    }

    impl_for_field
}

fn extract_doc_comments(attrs: &[Attribute]) -> Vec<LitStr> {
    let mut comments = vec![];

    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }

        if let Meta::NameValue(MetaNameValue {
            value: Expr::Lit(ExprLit {
                lit: Lit::Str(lit), ..
            }),
            ..
        }) = &attr.meta
        {
            comments.push(lit.clone());
        }
    }

    comments
}

fn impl_serde_aware_default(item: &ItemStruct) -> proc_macro2::TokenStream {
    let name = &item.ident;

    let initializers = item.fields.iter().map(|field| {
        let name = field
            .ident
            .as_ref()
            .expect("should not generate field docs for tuple struct");

        let span = field.ty.span();

        let cfg_attrs = field
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("cfg"));

        let function_path = get_field_default_fn(field)
            .unwrap_or_else(|| quote_spanned! { span=> Default::default });

        quote_spanned! { span=> #(#cfg_attrs)* #name: #function_path() }
    });

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self { #(#initializers,)* }
            }
        }
    }
}

fn get_field_default_fn(field: &Field) -> Option<proc_macro2::TokenStream> {
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let Meta::List(list) = &attr.meta else {
            continue;
        };

        let Ok(nested_meta_list) = NestedMeta::parse_meta_list(list.tokens.clone()) else {
            continue;
        };

        for meta in nested_meta_list {
            let NestedMeta::Meta(Meta::NameValue(mnv)) = meta else {
                continue;
            };

            if !mnv.path.is_ident("default") {
                continue;
            }

            let Expr::Lit(ExprLit {
                lit: Lit::Str(val), ..
            }) = mnv.value
            else {
                continue;
            };

            if let Ok(tokens) = val.parse() {
                return Some(tokens);
            }
        }
    }

    None
}
