use std::collections::BTreeMap;

use convert_case::Case;
use generator_shared::casing::cased_ident;
use generator_shared::extensions::DmmfInputFieldExt;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Ident;

use super::Module;
use crate::PrismaError;
use crate::args::Filter;
use crate::args::GeneratorArgs;
use crate::error::StringErrorContext;

pub fn write_param_name(filter: &Filter) -> Ident {
    format_ident!("{}Param", &filter.name)
}

pub(crate) fn generate_prisma_module(args: &GeneratorArgs) -> Result<Module, PrismaError> {
    let client = generate_client(args);
    let internal_enums = generate_internal_enums(args)?;

    let read_filters = generate_read_filters(args);
    let write_params = generate_write_params(args);

    Ok(Module::new(
        "_prisma",
        quote! {
            pub use ::generator_runtime::scalar::*;

            #client

            #internal_enums

            #read_filters
            #write_params
        },
    ))
}

fn generate_client(args: &GeneratorArgs) -> TokenStream {
    let model_actions = args
        .schema
        .context()
        .db
        .walk_models()
        .map(|model| {
            let model_name = cased_ident(model.name(), Case::Snake);

            quote! {
                pub fn #model_name(&self) -> super::model::#model_name::Actions {
                    super::model::#model_name::Actions {
                        client: &self.0,
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub struct PrismaClientBuilder {
            url: Option<String>,
        }

        impl PrismaClientBuilder {
            fn new() -> Self {
                Self {
                    url: None
                }
            }

            pub fn with_url(mut self, url: String) -> Self {
                self.url = Some(url);
                self
            }

            // TODO: This should be a ::generator_runtime::Error
            pub async fn build(self) -> Result<PrismaClient, ::generator_runtime::RuntimeError> {
                Ok(PrismaClient(
                    ::generator_runtime::client::InternalClient::new(
                        self.url,
                        super::PRISMA_SCHEMA,
                    ).await?
                ))
            }
        }

        #[derive(Clone)]
        pub struct PrismaClient(::generator_runtime::client::InternalClient);

        impl ::std::fmt::Debug for PrismaClient {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("PrismaClient").finish()
            }
        }

        impl PrismaClient {
            pub fn builder() -> PrismaClientBuilder {
                PrismaClientBuilder::new()
            }

            pub fn _transaction(&self) -> ::generator_runtime::transaction::TransactionBuilder<Self> {
                ::generator_runtime::transaction::TransactionBuilder::new(self, &self.0)
            }

            pub fn into_migrator(&self) -> ::generator_runtime::client::Migrator {
                self.into()
            }

            #(#model_actions)*
        }

        impl ::generator_runtime::client::PrismaClient for PrismaClient {
            fn with_tx_id(&self, tx_id: Option<::generator_runtime::client::TxId>) -> Self {
                Self(self.0.with_tx_id(tx_id))
            }
        }

        impl From<&PrismaClient> for ::generator_runtime::client::Migrator {
            fn from(value: &PrismaClient) -> ::generator_runtime::client::Migrator {
                ::generator_runtime::client::Migrator::new(
                    super::PRISMA_SCHEMA,
                    super::MIGRATIONS_DIR,
                    value.0.url(),
                )
            }
        }
    }
}

fn generate_internal_enums(args: &GeneratorArgs) -> Result<TokenStream, PrismaError> {
    let internal_enums = args
        .dmmf
        .schema
        .enum_types
        .get("prisma")
        .ok_or_else(|| {
            StringErrorContext {
                reason: "Unable to obtain prisma schema enum types",
            }
            .build()
        })?
        .iter()
        .map(|schema_enum| {
            let enum_name = cased_ident(&schema_enum.name, Case::Pascal);

            let enum_variants = schema_enum
                .values
                .iter()
                .map(|prisma_variant| {
                    let rust_variant = cased_ident(prisma_variant, Case::Pascal);

                    quote! {
                        #[serde(rename=#prisma_variant)]
                        #rust_variant
                    }
                })
                .collect::<Vec<_>>();

            let matches = schema_enum
                .values
                .iter()
                .map(|prisma_variant| {
                    let rust_variant = cased_ident(prisma_variant, Case::Pascal);

                    quote! {
                        Self::#rust_variant => #prisma_variant
                    }
                })
                .collect::<Vec<_>>();

            // let isolation_level_impl = (&e.name == "TransactionIsolationLevel").then(|| quote! {
            //     impl ::prisma_client_rust::TransactionIsolationLevel for TransactionIsolationLevel {}
            // });

            quote! {
                #[derive(Clone, Copy, Debug, Eq, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
                pub enum #enum_name {
                    #(#enum_variants),*
                }

                impl std::fmt::Display for #enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let result = match self {
                            #(#matches),*
                        };

                        write!(f, "{result}")
                    }
                }

                // #isolation_level_impl
            }
        });

    Ok(quote! {
        #(#internal_enums)*

        impl From<SortOrder> for ::generator_runtime::internal::PrismaValue {
            fn from(value: SortOrder) -> Self {
                // ::generator_runtime::internal::PrismaValue::String(value.to_string())
                match value {
                    SortOrder::Asc => ::generator_runtime::internal::PrismaValue::String("asc".to_owned()),
                    SortOrder::Desc => ::generator_runtime::internal::PrismaValue::String("desc".to_owned())
                }
            }
        }
    })
}

///     equals
///     not
///     in
///     notIn
///     lt
///     lte
///     gt
///     gte
///     contains
///     search
///     mode
///     startsWith
///     endsWith
///     AND
///     OR
///     NOT
///
/// some every none is isNot
///
/// https://www.prisma.io/docs/orm/reference/prisma-client-reference#filter-conditions-and-operators
fn generate_read_filters(args: &GeneratorArgs) -> TokenStream {
    let read_filters = args.read_params.iter().map(|filter| {
        let filter_name = format_ident!("{}Filter", filter.name);

        let (variants, matches): (Vec<_>, Vec<_>) = filter
            .fields
            .iter()
            .map(|field| {
                let action = &field.name;
                // Rename the prisma "in" filter, which is a rust keyword, to
                // a safe variant that we can use. "notIn" is changed to keep
                // the API the same between the two variants.
                let action_safe = match &*field.name {
                    "in" => "inVec",
                    "notIn" => "notInVec",
                    n => n,
                };

                let variant_name = cased_ident(action_safe, Case::Pascal);
                // TODO(enums): We probably need super::super:: here when adding
                //              enum support. Conflicts with super:: needed for local enums.
                let variant_type = field.to_tokens(&quote!(super::));

                let match_binding_ident = format_ident!("inner_value");
                let prisma_action = field.to_prisma_tokens(&match_binding_ident);

                /*
                // https://github.com/Brendonovich/prisma-client-rust/issues/297
                if filter.name == "JsonNullable" && field.name == "equals" {
                    Some((
                        quote!(#variant_name(Option<#typ>)),
                        quote! {
                            Self::#variant_name(#value_ident) =>
                                ::prisma_client_rust::SerializedWhereValue::Object(
                                    vec![(
                                        #action_str.to_string(),
                                        #value_ident.map(|#value_ident| #value_as_prisma_value)
                                            .unwrap_or(::prisma_client_rust::PrismaValue::Null)
                                    )]
                                )
                        },
                    ))
                } else {
                */
                (
                    quote!(#variant_name(#variant_type)),
                    quote! {
                        #filter_name::#variant_name(#match_binding_ident) =>
                            ::generator_runtime::model::SerializedWhereValue::Object(
                                vec![(
                                    #action.to_owned(),
                                    #prisma_action
                                )]
                            )
                    },
                )
            })
            .unzip();

        quote! {
            #[derive(Clone, Debug)]
            pub enum #filter_name {
                #(#variants),*
            }

            impl From<#filter_name> for ::generator_runtime::model::SerializedWhereValue {
                fn from(value: #filter_name) -> Self {
                    match value {
                        #(#matches),*
                    }
                }
            }
        }
    });

    quote! {
        pub mod read_filters {
            pub use ::generator_runtime::scalar::*;
            // pub use super::*;

            #(#read_filters)*
        }
    }
}

fn generate_write_params(args: &GeneratorArgs) -> TokenStream {
    // Collect into a [`BTreeMap`] to make the code-generation
    // deterministic.
    let write_params = args
        .write_params
        .iter()
        .map(|write_param| (&write_param.name, write_param))
        .collect::<BTreeMap<_, _>>();

    let write_params = write_params
        .values()
        .map(|param| {
            let param_name = write_param_name(param);

            let (variants, matches): (Vec<_>, Vec<_>) = {
                if param.name == "Json" {
                    todo!("Unimplemented");
                    /*
                    let var = format_ident!("value");
                    let prisma_value = ScalarType::Json.to_prisma_value(&var);

                    (
                        vec![quote!(Set(::prisma_client_rust::serde_json::Value))],
                        vec![quote!(Self::Set(#var) => #prisma_value)],
                    )
                    */
                } else {
                    let match_binding_ident = format_ident!("inner_value");

                    param
                        .fields
                        .iter()
                        .map(|field| {
                            let field_name = &field.name;
                            let field_name_pascal = cased_ident(field_name, Case::Pascal);

                            // TODO(enums): We probably need super::super:: here when adding
                            //              enum support.
                            let field_type = field.to_tokens(&quote!());

                            let prisma_action = {
                                let prisma_value = field.to_prisma_tokens(&match_binding_ident);

                                // Set is not wrapped in an action and instead returns the
                                // prisma value directly.
                                if field_name == "set" {
                                    prisma_value
                                } else {
                                    quote! {
                                        ::generator_runtime::internal::PrismaValue::Object(vec![(
                                            #field_name.to_owned(),
                                            #prisma_value
                                        )])
                                    }
                                }
                            };

                            (
                                quote!(#field_name_pascal(#field_type)),
                                quote!(#param_name::#field_name_pascal(#match_binding_ident) => #prisma_action),
                            )
                        })
                        .unzip()
                }
            };

            quote! {
                #[derive(Clone, Debug)]
                pub enum #param_name {
                    #(#variants),*
                }

                impl From<#param_name> for ::generator_runtime::internal::PrismaValue {
                    fn from(value: #param_name) -> Self {
                        match value {
                            #(#matches),*
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub mod write_params {
            pub use ::generator_runtime::scalar::*;

            #(#write_params)*
        }
    }
}
