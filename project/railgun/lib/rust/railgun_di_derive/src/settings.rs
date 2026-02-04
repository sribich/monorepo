use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    spanned::Spanned,
    Attribute, Expr, ExprLit, Field, Fields, Ident, Item, ItemEnum, ItemStruct, Lit, LitStr, Meta,
    MetaNameValue, Path,
};

use crate::parse_meta_list;

#[derive(FromMeta)]
struct Options {
    #[darling(default = "Options::default_impl_default")]
    impl_default: bool,
    #[darling(default = "Options::default_impl_debug")]
    impl_debug: bool,
    #[darling(default = "Options::default_crate_path")]
    crate_path: Path,
}

impl Options {
    fn default_impl_default() -> bool {
        true
    }

    fn default_impl_debug() -> bool {
        cfg!(debug_assertions)
    }

    fn default_crate_path() -> Path {
        parse_quote!(::railgun)
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            impl_default: Self::default_impl_default(),
            impl_debug: Self::default_impl_debug(),
            crate_path: Self::default_crate_path(),
        }
    }
}

impl Parse for Options {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let options = if input.is_empty() {
            Self::default()
        } else {
            let meta_list = parse_meta_list(&input)?;
            Self::from_list(&meta_list)?
        };

        Ok(options)
    }
}

pub(crate) fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(args as Options);
    let item = parse_macro_input!(item as Item);

    expand_from_parsed(options, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand_from_parsed(
    options: Options,
    mut item: Item,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match item {
        Item::Enum(ref mut item) => expand_enum(options, item),
        Item::Struct(ref mut item) => {
            if matches!(item.fields, Fields::Unnamed(_)) {
                expand_unnamed_struct(options, item)
            } else {
                expand_struct(options, item)
            }
        },
        Item::Const(_)
        | Item::ExternCrate(_)
        | Item::Fn(_)
        | Item::ForeignMod(_)
        | Item::Impl(_)
        | Item::Macro(_)
        | Item::Mod(_)
        | Item::Static(_)
        | Item::Trait(_)
        | Item::TraitAlias(_)
        | Item::Type(_)
        | Item::Union(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | _ => Err(syn::Error::new(
            item.span(),
            "Settings should be either a struct or enum",
        )),
    }
}

fn expand_enum(
    options: Options,
    item: &mut ItemEnum,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    todo!();
}

fn expand_unnamed_struct(
    options: Options,
    item: &mut ItemStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    todo!();
}

fn expand_struct(
    options: Options,
    item: &mut ItemStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let impl_settings = impl_settings_trait(&options, item)?;

    Ok(quote! {
        #item

        #impl_settings
    })
}

fn impl_settings_trait(
    options: &Options,
    item: &ItemStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = item.ident.clone();
    let crate_path = &options.crate_path;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let mut doc_comments_impl = quote! {};

    for field in &item.fields {
        if let Some(name) = &field.ident {
            let impl_for_field = impl_settings_trait_for_field(options, field, name);

            doc_comments_impl.append_all(impl_for_field);
        }
    }

    Ok(quote! {
        impl #impl_generics #crate_path::settings::Settings for #ident #ty_generics #where_clause {
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

fn impl_settings_trait_for_field(
    options: &Options,
    field: &Field,
    name: &Ident,
) -> proc_macro2::TokenStream {
    let crate_path = &options.crate_path;
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

        #crate_path::settings::Settings::add_docs(&self.#name, &key, docs);
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
