use std::collections::HashMap;

use convert_case::Case;
use dmmf::serialization_ast::TypeLocation;
use generator_shared::{
    casing::cased_ident,
    extensions::{DmmfSchemaExtension, DmmfTypeReferenceExt},
};
use proc_macro2::TokenStream;
use query_structure::{FieldArity, walkers::ModelWalker};
use quote::quote;
use syn::Ident;

use crate::{args::GeneratorArgs, rust::module::FieldModule};

/// TODO: This probably doesn't need to be here. We can surely move it into the
/// consumer.
pub fn fetch_builder_fn(model_name_snake: &Ident) -> TokenStream {
    quote! {
        pub fn order_by(mut self, param: super::super::#model_name_snake::OrderByWithRelationParam) -> Self {
            self.0 = self.0.order_by(param);
            self
        }
    }
}

pub fn generate_module(model: ModelWalker, args: &GeneratorArgs) -> FieldModule {
    let (order_by_relation_aggregate_param, aggregate_field_data) = args
        .dmmf
        .schema
        .find_input_type(&format!("{}OrderByRelationAggregateInput", model.name()))
        .map(|input_type| {
            let ((variants, match_arms), field_data): ((Vec<_>, Vec<_>), Vec<_>) = input_type
                .fields
                .iter()
                .filter_map(|field| {
                    let field_name = &field.name;
                    let field_name_pascal = cased_ident(&field.name, Case::Pascal);

                    let field_type = field.input_types.first().unwrap().to_tokens(
                        &quote!(super::),
                        FieldArity::Required,
                        &args.schema.context().db,
                    )?;

                    Some((
                        (
                            quote!(#field_name_pascal(#field_type)),
                            quote! {
                                OrderByRelationAggregateParam::#field_name_pascal(param) => (
                                    #field_name,
                                    param.into()
                                )
                            },
                        ),
                        (
                            field_name,
                            (
                                field_type,
                                quote! {
                                    // TODO: Import OrderByRelationAggregateParam in field module, remove super call
                                    impl From<Order> for super::OrderByRelationAggregateParam {
                                        fn from(Order(v): Order) -> Self {
                                            Self::#field_name_pascal(v)
                                        }
                                    }
                                },
                            ),
                        ),
                    ))
                })
                .unzip();

            (
                quote! {
                    #[derive(Debug, Clone)]
                    pub enum OrderByRelationAggregateParam {
                        #(#variants),*
                    }

                    impl From<OrderByRelationAggregateParam> for (String, ::generator_runtime::internal::PrismaValue) {
                        fn from(value: OrderByRelationAggregateParam) -> Self {
                            let (k, v) = match value {
                                #(#match_arms),*
                            };

                            (k.to_owned(), v)
                        }
                    }
                },
                field_data,
            )
        })
        .unwrap_or_default();

    let (order_by_with_relation_param, relation_field_data) = args
        .dmmf
        .schema
        .find_input_type(&format!("{}OrderByWithRelationInput", model.name()))
        .map(|input_type| {
            let ((variants, match_arms), field_data): ((Vec<_>, Vec<_>), Vec<_>) = input_type
                .fields
                .iter()
                .filter_map(|field| {
                    let field_name = &field.name;
                    let field_name_pascal = cased_ident(&field.name, Case::Pascal);

                    let field_type_ref = field.input_types.first().unwrap();
                    let field_type =
                        field_type_ref.to_tokens(&quote!(super::), FieldArity::Required, &args.schema.context().db)?;

                    let pv = match field_type_ref.location {
                        TypeLocation::EnumTypes | TypeLocation::Scalar => quote!(param.into()),
                        TypeLocation::InputObjectTypes => quote! {
                            ::generator_runtime::internal::PrismaValue::Object(
                                param.into_iter().map(Into::into).collect()
                            )
                        },
                        TypeLocation::OutputObjectTypes | TypeLocation::FieldRefTypes => return None,
                    };

                    Some((
                        (
                            quote!(#field_name_pascal(#field_type)),
                            quote! {
                                OrderByWithRelationParam::#field_name_pascal(param) => (
                                    #field_name,
                                    #pv
                                )
                            },
                        ),
                        (
                            field_name,
                            (
                                field_type_ref.to_tokens(
                                    &quote!(super::super::),
                                    FieldArity::Required,
                                    &args.schema.context().db,
                                )?,
                                quote! {
                                    // TODO: Import OrderByWithRelationParams in field module, remove super call
                                    impl From<Order> for super::OrderByWithRelationParam {
                                        fn from(Order(v): Order) -> Self {
                                            Self::#field_name_pascal(v)
                                        }
                                    }
                                },
                            ),
                        ),
                    ))
                })
                .unzip();

            (
                quote! {
                    #[derive(Debug, Clone)]
                    pub enum OrderByWithRelationParam {
                       #(#variants),*
                    }

                    impl From<OrderByWithRelationParam> for (String, ::generator_runtime::internal::PrismaValue) {
                        fn from(value: OrderByWithRelationParam) -> Self {
                            let (k, v) = match value {
                                #(#match_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }
                },
                field_data,
            )
        })
        .unwrap_or_default();

    FieldModule {
        model_data: quote! {
            #order_by_with_relation_param
            #order_by_relation_aggregate_param
        },
        field_data: aggregate_field_data
            .into_iter()
            .chain(relation_field_data)
            .fold(HashMap::new(), |mut acc, (name, data)| {
                let entry: &mut Vec<_> = acc.entry(name.clone()).or_default();
                entry.push(data);
                acc
            })
            .into_iter()
            .map(|(name, data)| {
                let Some(typ) = data
                    .iter()
                    .find_map(|(typ, _)| (typ.to_string() == data.first().unwrap().0.to_string()).then_some(typ))
                else {
                    panic!();
                };

                let impls = data.iter().map(|(_, impls)| impls);

                (
                    name,
                    quote! {
                        pub struct Order(#typ);

                        pub fn order<T: From<Order>>(v: #typ) -> T {
                            Order(v).into()
                        }

                        #(#impls)*
                    },
                )
            })
            .collect(),
    }
}
