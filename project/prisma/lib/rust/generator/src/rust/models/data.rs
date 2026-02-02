use convert_case::Case;
use generator_shared::{casing::cased_ident, extensions::FieldExtension};
use proc_macro2::TokenStream;
use psl::parser_database::ScalarFieldType;
use query_structure::{
    FieldArity,
    walkers::{ModelWalker, RefinedFieldWalker},
};
use quote::quote;

use crate::rust::module::FieldModule;

pub fn generate_data_field_module(model: ModelWalker) -> FieldModule {
    let field_data = model
        .fields()
        .filter_map(|field| {
            let field_name = field.name().to_owned();

            let (raw_type, recursive_type) = match field.refine_known() {
                RefinedFieldWalker::Scalar(scalar_field) => match scalar_field.scalar_field_type() {
                    ScalarFieldType::Enum(_)
                    | ScalarFieldType::Extension(_)
                    | ScalarFieldType::BuiltInScalar(_)
                    | ScalarFieldType::Unsupported(_) => (field.to_tokens(&quote!())?, field.to_tokens(&quote!())?),
                },
                RefinedFieldWalker::Relation(relation_field) => {
                    let related_model_name = cased_ident(relation_field.related_model().name(), Case::Snake);
                    let related_model_data = quote!(super::super::#related_model_name::Data);

                    match field.ast_field().arity {
                        FieldArity::Required => (quote!(#related_model_data), quote!(Box<#related_model_data>)),
                        FieldArity::Optional => (
                            quote!(Option<#related_model_data>),
                            quote!(Option<Box<#related_model_data>>),
                        ),
                        FieldArity::List => (quote!(Vec<#related_model_data>), quote!(Vec<#related_model_data>)),
                    }
                }
            };

            Some((
                field_name,
                quote! {
                    pub type Type = #raw_type;
                    pub type RecursiveSafeType = #recursive_type;
                },
            ))
        })
        .collect();

    let data_struct = generate_data_struct(model);

    FieldModule {
        model_data: quote! {
            #data_struct
        },
        field_data,
    }
}

fn generate_data_struct(model: ModelWalker) -> TokenStream {
    let struct_fields = model
        .fields()
        .filter(|field| field.ast_field().field_type.as_unsupported().is_none())
        .map(|field| match field.refine_known() {
            RefinedFieldWalker::Relation(relation_field) => {
                let field_name_prisma = relation_field.name();
                let field_name_rust = cased_ident(field_name_prisma, Case::Snake);

                let attributes = match relation_field.ast_field().arity {
                    FieldArity::Optional => quote! {
                        #[serde(
                            rename = #field_name_prisma,
                            default,
                            skip_serializing_if = "Option::is_none"
                            // with = "prisma_client_rust::serde::double_option"
                        )]
                    },
                    FieldArity::Required | FieldArity::List => quote! {
                        #[serde(rename = #field_name_prisma)]
                    },
                };

                // let specta_attrs = cfg!(feature = "specta").then(|| quote!(#[specta(skip)]));

                quote! {
                    #attributes
                    // #specta_attrs
                    pub #field_name_rust: Option<#field_name_rust::RecursiveSafeType>
                }
            }
            RefinedFieldWalker::Scalar(scalar_field) => {
                let field_name_prisma = scalar_field.name();
                let field_name_rust = cased_ident(field_name_prisma, Case::Snake);

                quote! {
                    #[serde(rename = #field_name_prisma)]
                    pub #field_name_rust: #field_name_rust::Type
                }
            }
        });

    let relation_accessors = model.fields().filter_map(|field| match field.refine_known() {
        RefinedFieldWalker::Relation(relation) => {
            let field_name = cased_ident(field.name(), Case::Snake);
            let model_name = cased_ident(relation.related_model().name(), Case::Snake);

            let (r#type, map) = match field.ast_field().arity {
                FieldArity::Required => (
                    quote! { &super::#model_name::Data },
                    Some(quote! { .map(std::convert::AsRef::as_ref) }), //|v| v.as_ref()
                ),
                FieldArity::Optional => (
                    quote! { Option<&super::#model_name::Data> },
                    Some(quote! { .map(|v| v.as_ref().map(std::convert::AsRef::as_ref)) }), //|v| v.as_ref()
                ),
                FieldArity::List => (quote! { &#field_name::Type }, None),
            };

            let error = quote! {
                ::generator_runtime::RuntimeError::Generic(stringify!(#field_name).to_owned())
            };

            Some(quote! {
                pub fn #field_name(&self) -> Result<#r#type, ::generator_runtime::RuntimeError> {
                    self.#field_name.as_ref().ok_or(#error) #map
                }
            })
        }
        RefinedFieldWalker::Scalar(_) => None,
    });

    let typegen_derive = {
        let model_name = cased_ident(model.name(), Case::Pascal).to_string();

        quote! {
            #[derive(::typegen::Typegen)]
            #[typegen(rename = #model_name)]
        }
    };

    quote! {
        #[derive(Clone, Debug, ::serde::Deserialize, ::serde::Serialize)]
        #typegen_derive
        pub struct Data {
            #(#struct_fields),*
        }

        impl Data {
            #(#relation_accessors)*
        }
    }
}
