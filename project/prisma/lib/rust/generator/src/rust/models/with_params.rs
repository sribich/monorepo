use convert_case::Case;
use generator_shared::casing::cased_ident;
use proc_macro2::TokenStream;
use query_structure::{
    FieldArity,
    walkers::{ModelWalker, RelationFieldWalker},
};
use quote::quote;

use super::{order_by, pagination};
use crate::{args::GeneratorArgs, rust::module::FieldModule};

pub fn generate_module(model: ModelWalker, _args: &GeneratorArgs) -> FieldModule {
    let variants = model.relation_fields().map(enum_variant);
    let into_selection_arms = model.relation_fields().map(into_selection_arm);

    let field_data = model
        .relation_fields()
        .map(|field| {
            let field_name_pascal = cased_ident(field.name(), Case::Pascal);

            let relation_model_name_snake = cased_ident(field.related_model().name(), Case::Snake);

            let order_by_fn = order_by::fetch_builder_fn(&relation_model_name_snake);
            let pagination_fns = pagination::fetch_builder_fns(&relation_model_name_snake);
            let with_fn = builder_fn(field);

            let body = match field.referential_arity() {
                FieldArity::List => {
                    quote! {
                        pub struct Fetch(pub super::super::#relation_model_name_snake::ManyArgs);

                        impl Fetch {
                            #with_fn

                            #order_by_fn

                            #pagination_fns
                        }

                        impl From<Fetch> for WithParam {
                            fn from(Fetch(v): Fetch) -> Self {
                                WithParam::#field_name_pascal(v)
                            }
                        }

                        pub fn fetch(params: Vec<super::super::#relation_model_name_snake::WhereParam>) -> Fetch {
                            Fetch(super::super::#relation_model_name_snake::ManyArgs::new(params))
                        }
                    }
                }
                FieldArity::Required | FieldArity::Optional => {
                    quote! {
                        pub struct Fetch(pub super::super::#relation_model_name_snake::UniqueArgs);

                        impl Fetch {
                            #with_fn
                        }

                        impl From<Fetch> for WithParam {
                            fn from(Fetch(v): Fetch) -> Self {
                                WithParam::#field_name_pascal(v)
                            }
                        }

                        pub fn fetch() -> Fetch {
                            Fetch(super::super::#relation_model_name_snake::UniqueArgs::new())
                        }
                    }
                }
            };

            (field.name().to_string(), body)
        })
        .collect();

    FieldModule {
        model_data: quote! {
            #[derive(Debug, Clone)]
            pub enum WithParam {
                #(#variants),*
            }

            impl From<WithParam> for ::generator_runtime::internal::Selection {
                fn from(value: WithParam) -> Self {
                    match value {
                        #(#into_selection_arms),*
                    }
                }
            }
        },
        field_data,
    }
}

pub fn builder_fn(field: RelationFieldWalker) -> TokenStream {
    let relation_model_name_snake = cased_ident(field.related_model().name(), Case::Snake);

    quote! {
        pub fn with(mut self, params: impl Into<super::super::#relation_model_name_snake::WithParam>) -> Self {
            self.0 = self.0.with(params.into());
            self
        }
    }
}

fn enum_variant(field: RelationFieldWalker) -> TokenStream {
    let field_name_pascal = cased_ident(field.name(), Case::Pascal);
    let relation_model_name_snake = cased_ident(field.related_model().name(), Case::Snake);

    let args = match field.ast_field().arity {
        FieldArity::List => quote!(ManyArgs),
        FieldArity::Required | FieldArity::Optional => quote!(UniqueArgs),
    };

    quote!(#field_name_pascal(super::#relation_model_name_snake::#args))
}

fn into_selection_arm(field: RelationFieldWalker) -> TokenStream {
    let field_name_snake = cased_ident(field.name(), Case::Snake);
    let field_name_pascal = cased_ident(field.name(), Case::Pascal);
    let relation_model_name_snake = cased_ident(field.related_model().name(), Case::Snake);

    let body = match field.ast_field().arity {
        FieldArity::List => quote! {
            let (arguments, mut nested_selections) = args.to_query_meta();
            nested_selections.extend(<super::#relation_model_name_snake::Types as ::generator_runtime::model::Model>::scalar_selections());

            ::generator_runtime::internal::Selection::new(
                #field_name_snake::NAME,
                None,
                arguments,
                nested_selections
            )
        },
        FieldArity::Required | FieldArity::Optional => quote! {
            let mut selections = <super::#relation_model_name_snake::Types as ::generator_runtime::model::Model>::scalar_selections();
            selections.extend(args.with_params.into_iter().map(Into::<::generator_runtime::internal::Selection>::into));

            ::generator_runtime::internal::Selection::new(
                #field_name_snake::NAME,
                None,
                [],
                selections
            )
        },
    };

    quote! {
        WithParam::#field_name_pascal(args) => {
            #body
        }
    }
}
