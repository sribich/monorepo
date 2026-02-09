use convert_case::Casing;
use macro_util::ast::Struct;
use macro_util::ast::Unresolved;
use quote::format_ident;
use quote::quote;
use syn::GenericParam;

use crate::attributes::common::tokenize_deprecation;
use crate::attributes::container::ContainerAttributes;
use crate::attributes::field::FieldAttributes;
use crate::typegen::datatype;
use crate::util::get_crate_name;

pub(super) fn impl_struct(
    container_name: &String,
    input: &Struct<'_, ContainerAttributes, Unresolved>,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let crate_name = get_crate_name();

    let generic_idents = input
        .generics
        .params
        .iter()
        .enumerate()
        .filter_map(|(index, value)| match value {
            GenericParam::Type(ty) => Some((index, &ty.ident)),
            GenericParam::Lifetime(_) | GenericParam::Const(_) => None,
        })
        .collect::<Vec<_>>();

    let type_generics = generic_idents.iter().map(|(_, ident)| {
        let ident = ident.to_string();
        quote!(::std::borrow::Cow::Borrowed(#ident).into())
    });

    let definition = if input.attributes.transparent {
        let field = match input.kind {
            macro_util::ast::StructKind::Named | macro_util::ast::StructKind::Unnamed => {
                let fields = input
                    .fields
                    .iter()
                    .map(macro_util::ast::Field::resolve_attributes::<FieldAttributes>)
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .filter(|field| !field.attributes.skip)
                    .collect::<Vec<_>>();

                if fields.len() != 1 {
                    return Err(syn::Error::new(
                        input.name.span(),
                        "Transparent structs must only have a single field.",
                    ));
                }

                fields.into_iter().next().unwrap()
            }
            macro_util::ast::StructKind::Unit => {
                return Err(syn::Error::new(
                    input.name.span(),
                    "Unit structs cannot be transparent.",
                ));
            }
        };

        let ty = datatype(&format_ident!("ty"), field.ty, &generic_idents)?;

        quote!({
            #ty

            ty
        })
    } else {
        let fields =
            match input.kind {
                macro_util::ast::StructKind::Named => {
                    let fields = input
                        .fields
                        .iter()
                        .map(macro_util::ast::Field::resolve_attributes::<FieldAttributes>)
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .filter(|field| !field.attributes.skip)
                        .map(|field| {
                            let docs = field.attributes.docs()?;
                            let deprecation = tokenize_deprecation(field.attributes.deprecation()?);

                            let field_name = field.attributes.rename.unwrap_or_else(|| {
                                let original_name = field.node.ident.as_ref().unwrap().to_string();

                                if let Some(rename_all) = &input.attributes.rename_all {
                                    original_name.to_case(rename_all.into())
                                } else {
                                    original_name
                                }
                            });

                            let ty = datatype(&format_ident!("ty"), field.ty, &generic_idents)
                                .map(|ty| {
                                    let ty = quote! {
                                        #ty

                                        ty
                                    };
                                    quote! { Some({
                                        #ty
                                    })}
                                })?;

                            let flatten = field.attributes.flatten;

                            Ok(Some(
                                quote!((#field_name.into(), #crate_name::datatype::field::field(
                                    #ty,
                                    #docs.into(),
                                    #deprecation,
                                    #flatten,
                                ))),
                            ))
                        })
                        .collect::<syn::Result<Vec<Option<proc_macro2::TokenStream>>>>()?;

                    quote!(#crate_name::datatype::r#struct::StructFields::new_named(
                        vec![#(#fields),*]
                    ))
                }
                macro_util::ast::StructKind::Unnamed => {
                    let fields = input
                        .fields
                        .iter()
                        .map(macro_util::ast::Field::resolve_attributes::<FieldAttributes>)
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .map(|field| {
                            let docs = field.attributes.docs()?;
                            let deprecation = tokenize_deprecation(field.attributes.deprecation()?);

                            let ty = datatype(&format_ident!("ty"), field.ty, &generic_idents)
                                .map(|ty| {
                                    let ty = quote! {
                                        #ty

                                        ty
                                    };
                                    quote! { Some({
                                        #ty
                                    })}
                                })?;

                            Ok(quote!(#crate_name::datatype::field::field(
                                #ty,
                                #docs.into(),
                                #deprecation,
                                false,
                            )))
                        })
                        .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>()?;

                    quote!(#crate_name::datatype::r#struct::StructFields::new_unnamed(
                        vec![#(#fields),*]
                    ))
                }
                macro_util::ast::StructKind::Unit => {
                    quote!(#crate_name::datatype::r#struct::StructFields::new_unit())
                }
            };

        quote! {
            #crate_name::datatype::DataType::Struct(
                #crate_name::datatype::r#struct::StructType::new(
                    #container_name.into(),
                    <Self as #crate_name::NamedType>::ID,
                    // Self::id().expect("We should have a TypeID here. TODO better error."),
                    #fields,
                    vec![#(#type_generics),*],
                )
            )
        }
    };

    let reference = {
        let reference_generics = generic_idents.iter().map(|(i, ident)| {
            quote! {
                generics
                    .get(#i)
                    .cloned()
                    .unwrap_or_else(|| <#ident as #crate_name::Type>::reference(cache, &[]).inner)
            }
        });

        let generic_datatypes = generic_idents.iter().map(|(i, ident)| {
            let ident = ident.to_string();

            quote! {
                (
                    std::borrow::Cow::Borrowed(#ident).into(),
                    generics[#i].clone(),
                )
            }
        });

        quote! {
            {
                let generics: &[#crate_name::datatype::DataType] = &[#(#reference_generics),*];

                #crate_name::datatype::reference::Reference::new_named::<Self>(
                    cache,
                    #crate_name::datatype::reference::ReferenceType::new(
                        #container_name,
                        vec![#(#generic_datatypes),*],
                        <Self as #crate_name::NamedType>::ID,
                    )
                )
            }
        }
    };

    Ok((definition, reference))
}
