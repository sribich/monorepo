use std::collections::HashMap;

use convert_case::Case;
use generator_shared::{
    casing::cased_ident,
    extensions::{DmmfInputFieldExt, FieldExtension},
};
use proc_macro2::TokenStream;
use psl::parser_database::ScalarFieldType;
use query_structure::{
    FieldArity,
    walkers::{FieldWalker, ModelWalker, RefinedFieldWalker, RelationFieldWalker},
};
use quote::{format_ident, quote};

use crate::{
    args::GeneratorArgs,
    rust::{module::FieldModule, prisma::write_param_name},
};

pub fn generate_field_module(model: ModelWalker, args: &GeneratorArgs) -> FieldModule {
    let (variants, arms, field_data) = model
        .fields()
        .filter_map(|field| generate_field_params(field, args))
        .fold(
            (vec![], vec![], HashMap::default()),
            |(mut a, mut b, mut c), (d, e, f)| {
                a.extend(d);
                b.extend(e);
                c.insert(f.0, f.1);

                (a, b, c)
            },
        );

    let (unchecked_enum, unchecked_fields) = {
        let ((unchecked_variants, into_pv_arms), field_stuff): ((Vec<_>, Vec<_>), Vec<_>) = model
            .scalar_fields()
            .filter_map(|field| {
                let field_name_raw = field.name();
                let field_name_pascal = cased_ident(field_name_raw, Case::Pascal);

                Some(match field.scalar_field_type() {
                    ScalarFieldType::Unsupported(_) => return None,
                    ScalarFieldType::Extension(_) | ScalarFieldType::Enum(_) | ScalarFieldType::BuiltInScalar(_) => {
                        args.write_param(field).map(|write_param| {
                            // let param_enum = write_params::enum_name(write_param);
                            let param_enum = write_param_name(write_param);
                            let param_enum_path = quote!(prisma::write_params::#param_enum);

                            (
                                (
                                    quote!(#field_name_pascal(super::super::prisma::write_params::#param_enum)),
                                    quote! {
                                        UncheckedSetParam::#field_name_pascal(value) => (
                                            #field_name_raw,
                                            value.into()
                                        )
                                    },
                                ),
                                (
                                    field_name_raw.to_owned(),
                                    quote! {
                                        impl From<Set> for UncheckedSetParam {
                                            fn from(Set(v): Set) -> Self {
                                                Self::#field_name_pascal(#param_enum_path::Set(v))
                                            }
                                        }

                                        impl From<UpdateOperation> for UncheckedSetParam {
                                            fn from(UpdateOperation(v): UpdateOperation) -> Self {
                                                Self::#field_name_pascal(v)
                                            }
                                        }
                                    },
                                ),
                            )
                        })?
                    }
                })
            })
            .unzip();

        (
            quote! {
                #[derive(Debug, Clone)]
                pub enum UncheckedSetParam {
                      #(#unchecked_variants),*
                }

                impl From<UncheckedSetParam> for (String, ::generator_runtime::internal::PrismaValue) {
                    fn from(value: UncheckedSetParam) -> Self {
                        let (k, v) = match value {
                            #(#into_pv_arms),*
                        };

                        (k.to_string(), v)
                    }
                }
            },
            field_stuff,
        )
    };

    FieldModule {
        model_data: quote! {
            #[derive(Clone, Debug)]
            pub enum SetParam {
                #(#variants),*
            }

            impl From<SetParam> for (String, ::generator_runtime::internal::PrismaValue) {
                fn from(value: SetParam) -> Self {
                    let (k, v) = match value {
                        #(#arms),*
                    };

                    (k.to_string(), v)
                }
            }

            #unchecked_enum
        },
        field_data: unchecked_fields.into_iter().fold(field_data, |mut acc, (k, v)| {
            let entry = acc.entry(k).or_insert_with(|| quote!());
            entry.extend(v);
            acc
        }),
    }
}

fn generate_field_params(
    field: FieldWalker,
    args: &GeneratorArgs,
) -> Option<(Vec<TokenStream>, Vec<TokenStream>, (String, TokenStream))> {
    let field_name_pascal = cased_ident(field.name(), Case::Pascal);
    let field_name_snake = cased_ident(field.name(), Case::Snake);
    let field_type = field.to_tokens(&quote!());

    let arity = field.ast_field().arity;

    let mut variants = vec![];
    let mut functions = vec![];

    let data = match field.refine_known() {
        RefinedFieldWalker::Scalar(scalar_field) => {
            match scalar_field.scalar_field_type() {
                ScalarFieldType::Enum(_)
                | ScalarFieldType::Extension(_)
                | ScalarFieldType::BuiltInScalar(_)
                | ScalarFieldType::Unsupported(_) => {
                    // Enums exist in the global scope, let's use super
                    let prefix = if let ScalarFieldType::Enum(_) = scalar_field.scalar_field_type() {
                        quote!(super::super::super::)
                    } else {
                        quote!()
                    };

                    if let Some(write_param) = args.write_param(scalar_field) {
                        let param_enum = write_param_name(write_param);

                        let param_enum_path = quote!(prisma::write_params::#param_enum);

                        let mutation_fns = write_param
                            .fields
                            .iter()
                            .filter_map(|inner_field| {
                                if inner_field.name == "set" {
                                    return None;
                                }

                                let method_name_snake = cased_ident(&inner_field.name, Case::Snake);
                                let method_name_pascal = cased_ident(&inner_field.name, Case::Pascal);

                                let typ = inner_field.to_tokens(&quote!());

                                Some(quote! {
                                    pub fn #method_name_snake<T: From<UpdateOperation>>(value: #typ) -> T {
                                        UpdateOperation(#param_enum_path::#method_name_pascal(value)).into()
                                    }
                                })
                            })
                            .collect::<TokenStream>();

                        variants.push(quote!(#field_name_pascal(super::super::prisma::write_params::#param_enum)));
                        functions.push(quote! {
                            SetParam::#field_name_pascal(value) => (
                                #field_name_snake::NAME,
                                value.into()
                            )
                        });

                        (
                            field.name().to_owned(),
                            quote! {
                                pub struct Set(pub #prefix #field_type);

                                impl From<Set> for SetParam {
                                    fn from(Set(v): Set) -> Self {
                                        Self::#field_name_pascal(#param_enum_path::Set(v))
                                    }
                                }

                                pub fn set<T: From<Set>>(value: #prefix #field_type) -> T {
                                    Set(value).into()
                                }

                                pub struct UpdateOperation(pub #param_enum_path);

                                impl From<UpdateOperation> for SetParam {
                                    fn from(UpdateOperation(v): UpdateOperation) -> Self {
                                        Self::#field_name_pascal(v)
                                    }
                                }

                                #mutation_fns
                            },
                        )
                    } else {
                        return None;
                    }
                }
            }
        }
        RefinedFieldWalker::Relation(relation_field) => {
            let (v, f): (Vec<_>, Vec<_>) = relation_field_set_params(relation_field)
                .iter()
                .map(|param| {
                    let action = param.action;
                    let relation_model_name_snake = cased_ident(relation_field.related_model().name(), Case::Snake);
                    let variant_name = format_ident!("{}{}", cased_ident(action, Case::Pascal), &field_name_pascal);

                    match param.typ {
                    RelationSetParamType::Many => {
                        (
                        	quote!(#variant_name(Vec<super::#relation_model_name_snake::UniqueWhereParam>)),
                            quote! {
                                SetParam::#variant_name(where_params) => (
                                    #field_name_snake::NAME,
                                    ::generator_runtime::internal::PrismaValue::Object(
                                        vec![(
                                            #action.to_string(),
                                            ::generator_runtime::internal::PrismaValue::List(
                                                where_params
                                                    .into_iter()
                                                .map(::generator_runtime::model::WhereInput::serialize)
                                                .map(::generator_runtime::model::SerializedWhereInput::transform_equals)
                                                .map(|v| ::generator_runtime::internal::PrismaValue::Object(vec![v]))
                                                    .collect()
                                            )
                                        )]
                                    )
                                )
                            }
                        )
                    }
                    RelationSetParamType::Single => {
                        (quote!(#variant_name(super::#relation_model_name_snake::UniqueWhereParam)),
                            quote! {
                                SetParam::#variant_name(where_param) => (
                                    #field_name_snake::NAME,
                                    ::generator_runtime::internal::PrismaValue::Object(
                                        vec![(
                                            #action.to_string(),
                                            ::generator_runtime::internal::PrismaValue::Object(
                                                std::iter::once(where_param)
                                                    .map(::generator_runtime::model::WhereInput::serialize)
                                                    .map(::generator_runtime::model::SerializedWhereInput::transform_equals)
                                                    .collect()
                                            )
                                        )]
                                    )
                                )
                            }
                        )
                    }
                    RelationSetParamType::True => {
                        (quote!(#variant_name),
                            quote! {
                                SetParam::#variant_name => (
                                    #field_name_snake::NAME,
                                    ::generator_runtime::internal::PrismaValue::Object(
                                        vec![(
                                            #action.to_string(),
                                            ::generator_runtime::internal::PrismaValue::Boolean(true)
                                        )]
                                    )
                                )
                            }
                        )
                    }
                }
                }).unzip();

            let relation_model_name_snake = cased_ident(relation_field.related_model().name(), Case::Snake);

            let connect_variant = format_ident!("Connect{field_name_pascal}");
            let disconnect_variant = format_ident!("Disconnect{field_name_pascal}");
            let set_variant = format_ident!("Set{field_name_pascal}");
            let is_null_variant = format_ident!("{field_name_pascal}IsNull");

            let base = match arity {
                FieldArity::List => {
                    quote! {
                        pub struct Connect(pub Vec<super::super::#relation_model_name_snake::UniqueWhereParam>);

                        impl From<Connect> for SetParam {
                            fn from(Connect(v): Connect) -> Self {
                                Self::#connect_variant(v)
                            }
                        }

                        pub fn connect<T: From<Connect>>(params: Vec<super::super::#relation_model_name_snake::UniqueWhereParam>) -> T {
                            Connect(params).into()
                        }

                        pub fn disconnect(params: Vec<super::super::#relation_model_name_snake::UniqueWhereParam>) -> SetParam {
                            SetParam::#disconnect_variant(params)
                        }

                        pub fn set(params: Vec<super::super::#relation_model_name_snake::UniqueWhereParam>) -> SetParam {
                            SetParam::#set_variant(params)
                        }
                    }
                }
                FieldArity::Required | FieldArity::Optional => {
                    let optional_fns = arity.is_optional().then(|| {
                        quote! {
                            pub fn disconnect() -> SetParam {
                                SetParam::#disconnect_variant
                            }

                            pub fn is_null() -> WhereParam {
                                WhereParam::#is_null_variant
                            }
                        }
                    });

                    quote! {
                        pub struct Connect(super::super::#relation_model_name_snake::UniqueWhereParam);

                        impl From<Connect> for SetParam {
                            fn from(Connect(v): Connect) -> Self {
                                Self::#connect_variant(v)
                            }
                        }

                        pub fn connect<T: From<Connect>>(value: super::super::#relation_model_name_snake::UniqueWhereParam) -> T {
                            Connect(value).into()
                        }

                        #optional_fns
                    }
                }
            };

            variants.extend(v);
            functions.extend(f);

            (field.name().to_owned(), base)
        }
    };

    Some((variants, functions, data))
}

pub struct RelationSetParamConfig {
    pub action: &'static str,
    pub typ: RelationSetParamType,
}

pub enum RelationSetParamType {
    /// Arguments are Vec of UniqueWhereParams
    Many,
    /// Arguments is a single WhereParam
    Single,
    /// No arguments, value is Boolean(true)
    True,
}

fn relation_field_set_params(field: RelationFieldWalker) -> Vec<RelationSetParamConfig> {
    let arity = field.ast_field().arity;

    if arity.is_list() {
        ["connect", "disconnect", "set"]
            .iter()
            .map(|action| RelationSetParamConfig {
                action,
                typ: RelationSetParamType::Many,
            })
            .collect()
    } else {
        let mut params = vec![RelationSetParamConfig {
            action: "connect",
            typ: RelationSetParamType::Single,
        }];

        if arity.is_optional() {
            params.push(RelationSetParamConfig {
                action: "disconnect",
                typ: RelationSetParamType::True,
            });
        }

        params
    }
}
