mod r#enum;
mod r#struct;

use r#enum::impl_enum;
use macro_util::{
    ast::Input,
    generics::{generics_with_ident, generics_with_ident_and_bounds, where_clause_with_bounds},
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, format_ident, quote};
use r#struct::impl_struct;
use syn::{GenericArgument, Ident, PathArguments, Type};

use crate::{attributes::container::ContainerAttributes, util::get_crate_name};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    inner_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn inner_derive(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse(input)?;
    let input = Input::from_syn(&input)?;

    let crate_name = get_crate_name();

    let ident = input.ident();
    let generics = input.generics();

    let type_params = generics_with_ident(generics);
    let trait_bounds = generics_with_ident_and_bounds(generics);
    let where_bounds =
        where_clause_with_bounds(generics, |ident| quote!(#ident: #crate_name::Type));

    let container_attributes = input.attributes::<ContainerAttributes>()?;

    let ident_str = ident.to_string();
    let serialized_container_name = container_attributes
        .rename
        .as_ref()
        .map(|name| format_ident!("{}", name))
        .unwrap_or_else(|| ident.clone())
        .to_string();

    let (definition, reference) = match &input {
        Input::Struct(input) => impl_struct(
            &serialized_container_name,
            &input.resolve_attributes::<ContainerAttributes>()?,
        )?,
        Input::Enum(input) => impl_enum(
            &serialized_container_name,
            &input.resolve_attributes::<ContainerAttributes>()?,
        )?,
        Input::Union(_) => todo!(),
    };

    let definition_generics = generics.type_params().map(|param| {
        let ident = param.ident.to_string();

        quote!(#crate_name::datatype::DataType::Generic(
            #crate_name::datatype::generic::GenericType::from_str(#ident)
        ))
    });

    let comment = container_attributes.docs()?;
    let deprecation = if let Some(deprecation) = container_attributes.deprecation()? {
        let inner_tokens = deprecation.to_token_stream();
        quote!(Some(#inner_tokens))
    } else {
        quote!(None)
    };

    Ok(quote! {
        const _: () = {
            const SOURCE_LOCATION: #crate_name::id::SourceLocation = #crate_name::comptime::source_location(
                concat!(file!(), ":", line!(), ":", column!())
            );

            const DEFINITION_GENERICS: &[#crate_name::datatype::DataType] = &[#(#definition_generics),*];

            #[automatically_derived]
            impl #trait_bounds #crate_name::Type for #ident #type_params #where_bounds {
                fn datatype(
                    cache: &mut #crate_name::cache::TypeCache,
                    generics: &#crate_name::Generics
                ) -> #crate_name::datatype::DataType {
                    let generics = match generics {
                        #crate_name::Generics::Impl => DEFINITION_GENERICS,
                        #crate_name::Generics::Concrete(generics) => *generics,
                    };

                    #definition
                }

                fn reference(
                    cache: &mut #crate_name::cache::TypeCache,
                    generics: &[#crate_name::datatype::DataType]
                ) -> #crate_name::datatype::reference::Reference {
                    #reference
                }
            }

            #[automatically_derived]
            impl #trait_bounds #crate_name::NamedType for #ident #type_params #where_bounds {
                const ID: #crate_name::id::TypeId = #crate_name::comptime::type_id(#ident_str, SOURCE_LOCATION.0);

                fn named_datatype(cache: &mut #crate_name::cache::TypeCache, generics: &#crate_name::Generics) -> #crate_name::datatype::NamedDataType {
                    #crate_name::datatype::NamedDataType::new(
                        #serialized_container_name.into(),
                        Self::ID,
                        <Self as #crate_name::Type>::datatype(cache, generics),
                        #comment.into(),
                        #deprecation,
                    )
                }
            }
        };
    })
}

pub fn datatype(
    ident: &Ident,
    ty: &Type,
    generic_type_idents: &[(usize, &Ident)],
) -> syn::Result<proc_macro2::TokenStream> {
    let crate_name = get_crate_name();

    let path = match ty {
        Type::Array(_) => todo!(),
        Type::BareFn(_) => todo!(),
        Type::Group(_) => todo!(),
        Type::ImplTrait(_) => todo!(),
        Type::Infer(_) => todo!(),
        Type::Macro(_) => todo!(),
        Type::Never(_) => todo!(),
        Type::Paren(_) => todo!(),
        Type::Path(path) => &path.path,
        Type::Ptr(_) => todo!(),
        Type::Reference(_) => todo!(),
        Type::Slice(_) => todo!(),
        Type::TraitObject(_) => todo!(),
        Type::Tuple(tuple) => {
            let elems = tuple
                .elems
                .iter()
                .enumerate()
                .map(|(i, el)| datatype(&format_ident!("{}_{}", ident, i), el, generic_type_idents))
                .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>()?;

            let generic_idents = tuple
                .elems
                .iter()
                .enumerate()
                .map(|(i, _)| format_ident!("{}_{}", &ident, i));

            let generics = quote!(&[#(#generic_idents),*]);

            return Ok(quote! {
                #(#elems)*

                let #ident = <#ty as #crate_name::Type>::reference(cache, #generics).inner;
            });
        },
        Type::Verbatim(_) => todo!(),
        _ => todo!(),
    };

    if let Some(type_ident) = path.get_ident()
        && let Some((i, generic_ident)) = generic_type_idents
            .iter()
            .find(|(_, ident)| ident == &type_ident)
    {
        let type_ident = type_ident.to_string();
        let generics = quote!(&[#crate_name::datatype::DataType::Generic(std::borrow::Cow::Borrowed(#type_ident).into())]);

        // , #generics
        return Ok(quote! {
            let #ident = generics.get(#i).cloned().unwrap_or_else(
                || {
                    <#generic_ident as #crate_name::Type>::reference(cache, #generics).inner
                },
            );
        });
    }

    let generic_args = match &path.segments.last().unwrap().arguments {
        PathArguments::AngleBracketed(args) => args
            .args
            .iter()
            .enumerate()
            .filter_map(|(i, input)| match input {
                GenericArgument::Type(ty) => Some((i, ty)),
                GenericArgument::Lifetime(_)
                | GenericArgument::Const(_)
                | GenericArgument::AssocType(_)
                | GenericArgument::AssocConst(_)
                | GenericArgument::Constraint(_)
                | _ => None,
            })
            .collect(),
        PathArguments::None => vec![],
        PathArguments::Parenthesized(_) => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Only angle bracketed generics are supported!",
            ));
        },
    };
    let generic_vars = generic_args
        .iter()
        .map(|(i, path)| {
            datatype(
                &format_ident!("{}_{}", &ident, i),
                path,
                generic_type_idents,
            )
        })
        .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>()?;
    let generic_var_idents = generic_args
        .iter()
        .map(|(i, _)| format_ident!("{}_{}", &ident, i));
    let generics = quote!(&[#(#generic_var_idents),*]);
    Ok(quote! {
        #(#generic_vars)*

        let #ident = <#ty as #crate_name::Type>::reference(cache, #generics).inner;
        // let #ident = <#ty as typegen::Type>::datatype();
    })
}
