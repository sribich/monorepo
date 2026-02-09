use convert_case::Casing;
use macro_util::ast::Enum;
use macro_util::ast::Unresolved;
use macro_util::ast::variant::Variant;
use quote::format_ident;
use quote::quote;
use syn::Fields;
use syn::GenericParam;

use crate::attributes::common::tokenize_deprecation;
use crate::attributes::container::ContainerAttributes;
use crate::attributes::container::Tagged;
use crate::attributes::field::FieldAttributes;
use crate::attributes::variant::VariantAttributes;
use crate::typegen::datatype;
use crate::util::get_crate_name;

pub(super) fn impl_enum(
    container_name: &String,
    input: &Enum<'_, ContainerAttributes, Unresolved>,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let crate_name = get_crate_name();

    if input.attributes.transparent {
        return Err(syn::Error::new(
            input.name.span(),
            "Enums may not be transparent.",
        ));
    }

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

    let variant_types =
        input.variants
            .iter()
            .map(Variant::resolve_attributes::<VariantAttributes>)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|variant| {
                let variant_name = variant.attributes.rename.clone().unwrap_or_else(|| {
                    let original_name = variant.name.to_string();

                    if let Some(rename_all) = &input.attributes.rename_all {
                        original_name.to_case(rename_all.into())
                    } else {
                        original_name
                    }
                });

                let generic_idents = generic_idents.clone();

                let inner = match &variant.node.fields {
                    Fields::Unit => {
                        quote!(#crate_name::datatype::r#enum::EnumVariantFields::new_unit())
                    },
                    Fields::Unnamed(_) => {
                        let fields = variant
                            .fields
                            .iter()
                            .map(macro_util::ast::Field::resolve_attributes::<FieldAttributes>)
                            .collect::<Result<Vec<_>, _>>()?
                            .iter()
                            .map(|field| {
                                let deprecation = tokenize_deprecation(field.attributes.deprecation()?);
                                let docs = field.attributes.docs()?;

                                let ty = if variant.attributes.skip || field.attributes.skip { Ok(quote!(None)) } else { {
                                        datatype(&format_ident!("fld"), field.ty, &generic_idents).map(|fld| {
                                            let ty = quote! {
                                                #fld
                                                fld
                                            };
                                            quote! { Some({
                                                #ty
                                            })}
                                        })
                                    } }?;

                                Ok(quote! {
                                    #crate_name::datatype::field::field(
                                        #ty,
                                        #docs.into(),
                                        #deprecation,
                                        false,
                                    )
                                })
                            })
                            .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>()?;

                        quote!(#crate_name::datatype::r#enum::EnumVariantFields::new_unnamed(
                            vec![#(#fields),*],
                        ))
                    },
                    Fields::Named(_) => {
                        let fields = variant
                            .fields
                            .iter()
                            .map(macro_util::ast::Field::resolve_attributes::<FieldAttributes>)
                            .collect::<Result<Vec<_>, _>>()?
                            .iter()
                            .map(|field| {
                                let deprecation = tokenize_deprecation(field.attributes.deprecation()?);
                                let docs = field.attributes.docs()?;

                                let field_name = field.attributes.rename.clone().unwrap_or_else(|| {
                                    let original_name = field.node.ident.as_ref().unwrap().to_string();

                                    if let Some(rename_all_fields) = &input.attributes.rename_all_fields {
                                        original_name.to_case(rename_all_fields.into())
                                    } else {
                                        original_name
                                    }
                                });

                                let ty = if variant.attributes.skip || field.attributes.skip { Ok(quote!(None)) } else { {
                                        datatype(&format_ident!("fld"), field.ty, &generic_idents).map(|fld| {
                                            let ty = quote! {
                                                #fld
                                                fld
                                            };
                                            quote! { Some({
                                                #ty
                                            })}
                                        })
                                    } }?;

                                Ok(quote!{(
                                    #field_name.into(),
                                    #crate_name::datatype::field::field(
                                        #ty,
                                        #docs.into(),
                                        #deprecation,
                                        false,
                                    )
                                )})
                            })
                            .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>()?;

                        quote! {
                            #crate_name::datatype::r#enum::EnumVariantFields::new_named(
                                vec![#(#fields),*],
                            )
                        }
                    },
                };

                let deprecated = variant.attributes.deprecation()?.map(|it| { let ty = it.as_tokens(); quote!(Some(#ty)) }).unwrap_or(quote!(None));
                let skip = variant.attributes.skip;
                let doc = variant.attributes.docs()?;

                Ok(quote!((#variant_name.into(), #crate_name::datatype::r#enum::EnumVariant::new(#skip, #doc.into(), #deprecated, #inner))))
            })
            .collect::<syn::Result<Vec<_>>>()?;

    let repr = input.attributes.tagged()?;
    let repr = match repr {
        Tagged::Externally => quote!(#crate_name::datatype::r#enum::EnumRepr::External),
        Tagged::Adjacently { tag, content } => {
            quote!(#crate_name::datatype::r#enum::EnumRepr::Adjacent { tag: #tag.into(), content: #content.into() })
        }
        Tagged::Internally { tag } => {
            quote!(#crate_name::datatype::r#enum::EnumRepr::Internal { tag: #tag.into() })
        }
        Tagged::Untagged => quote!(#crate_name::datatype::r#enum::EnumRepr::Untagged),
    };

    let definition = quote! {
        #crate_name::datatype::DataType::Enum(
                #crate_name::datatype::r#enum::EnumType::new(
                    #container_name.into(),
                    <Self as #crate_name::NamedType>::ID,
                    #repr,
                    vec![#(#type_generics),*],
                    vec![#(#variant_types),*],
                )
            )
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
        let generic_len = generic_idents.len();

        quote! {
            {
                let generics: &[#crate_name::datatype::DataType] = &[#(#reference_generics),*];

                assert!(generics.len() >= #generic_len);

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
