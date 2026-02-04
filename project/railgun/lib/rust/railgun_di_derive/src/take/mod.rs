use macro_util::{
    ast::{Input, Struct, StructKind},
    generics::{generics_with_ident, generics_with_ident_and_bounds, where_clause_with_type_bound},
};
use proc_macro::TokenStream;
use proc_macro_error2::abort;
use quote::{format_ident, quote};

pub fn derive(input: TokenStream) -> TokenStream {
    inner_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn inner_derive(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse(input)?;
    let input = Input::from_syn(&input)?;

    Ok(match input {
        Input::Struct(input) => impl_struct(&input),
        Input::Enum(input) => abort!(input.node, "Enums are not valid Take targets.",),
        Input::Union(input) => abort!(input.node, "Unions are not valid Take targets.",),
    })
}

fn impl_struct(input: &Struct) -> proc_macro2::TokenStream {
    match input.kind {
        StructKind::Named => impl_named_struct(input),
        StructKind::Unnamed => todo!(),
        StructKind::Unit => todo!(),
    }
}

fn impl_named_struct(input: &Struct) -> proc_macro2::TokenStream {
    let ty = &input.name;

    let newtype_name = format_ident!("Taken{}", quote!(#ty).to_string());

    let trait_bounds = generics_with_ident_and_bounds(input.generics);
    let where_bounds = where_clause_with_type_bound(input.generics);
    let type_params = generics_with_ident(input.generics);

    let (fields, mappers) = input
        .fields
        .iter()
        .map(|field| {
            let ident = field
                .node
                .ident
                .as_ref()
                .expect("Guaranteed to be a named struct");
            let ty = field.ty;

            (quote!(pub #ident: #ty), quote!(#ident: self.#ident))
        })
        .collect::<(Vec<_>, Vec<_>)>();

    quote! {
        #[allow(dead_code)]
        pub struct #newtype_name #type_params #where_bounds {
            #(#fields),*
        }

        impl #trait_bounds #ty #type_params #where_bounds {
            pub fn take(self) -> #newtype_name #type_params {
                #newtype_name {
                    #(#mappers),*
                }
            }
        }
    }
}
