use convert_case::{Case, Casing};
use generator_shared::{
    casing::cased_ident,
    extensions::{DmmfInputFieldExt, FieldExtension, ScalarFieldTypeExtension},
};
use proc_macro2::TokenStream;
use psl::parser_database::ScalarFieldType;
use query_structure::{
    FieldArity,
    walkers::{FieldWalker, IndexFieldWalker, ModelWalker, ScalarFieldWalker},
};
use quote::{format_ident, quote};

use crate::{
    args::{Filter, GeneratorArgs},
    rust::module::FieldModule,
};

pub struct Operator {
    pub name: &'static str,
    pub action: &'static str,
    pub list: bool,
}

static OPERATORS: &[Operator] = &[
    Operator {
        name: "Not",
        action: "NOT",
        list: false,
    },
    Operator {
        name: "Or",
        action: "OR",
        list: true,
    },
    Operator {
        name: "And",
        action: "AND",
        list: false,
    },
];

pub fn generate_where_field_module(model: ModelWalker, args: &GeneratorArgs) -> FieldModule {
    let mut variants = vec![];

    variants.extend(OPERATORS.iter().map(|op| {
        let variant_name = cased_ident(op.name, Case::Pascal);
        let op_action = &op.action;

        let value = if op.list {
            quote! {
                ::generator_runtime::model::SerializedWhereValue::List(
                    value
                        .into_iter()
                        .map(::generator_runtime::model::WhereInput::serialize)
                        .map(|p| ::generator_runtime::internal::PrismaValue::Object(vec![p.into()]))
                        .collect()
                )
            }
        } else {
            quote! {
                ::generator_runtime::model::SerializedWhereValue::Object(
                    ::generator_runtime::model::merge_fields(
                        value
                            .into_iter()
                            .map(::generator_runtime::model::WhereInput::serialize)
                            .map(Into::into)
                            .collect()
                    )
                )
            }
        };

        Variant::Base {
            definition: quote!(#variant_name(Vec<WhereParam>)),
            match_arm: quote! {
                Self::#variant_name(value) => (
                    #op_action,
                    #value,
                )
            },
        }
    }));

    let compound_field_accessors = unique_field_combos(model).iter().filter_map(|fields| {
        if fields.len() == 1 {
            let field = *fields.first().unwrap();

            let read_filter = args.read_filter(
                field
            ).unwrap();

            variants.push(Variant::unique(field, read_filter, &quote!()));

            None
        } else {
            let variant_name_string = fields.iter().map(|f| cased_ident(f.name(), Case::Pascal).to_string()).collect::<String>();
            let variant_name = format_ident!("{}Equals", &variant_name_string);

            let variant_data_names = fields.iter().map(|f| cased_ident(f.name(), Case::Snake)).collect::<Vec<_>>();

            let ((field_defs, field_types), (prisma_values, field_names_snake)):
                ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = fields.iter().map(|field| {
                let field_type = match field.ast_field().arity {
                    FieldArity::List | FieldArity::Required => field.to_tokens(&quote!()),
                    FieldArity::Optional => field.scalar_field_type().to_tokens(&quote!(), FieldArity::Required, field.db)
                }.unwrap();


                    let field_name_snake = cased_ident(field.name(), Case::Snake);

                (
                    (quote!(#field_name_snake: #field_type), field_type),
                    (field.scalar_field_type().to_prisma_tokens(&field_name_snake, FieldArity::Required), field_name_snake)
                )
            }).unzip();

            let field_names_joined = fields.iter().map(|f| f.name()).collect::<Vec<_>>().join("_");

            variants.extend([
                Variant::CompoundUnique {
                    field_names_string: variant_name_string.clone(),
                    variant_data_types: field_types,
                    match_arm: quote! {
                    	Self::#variant_name(#(#field_names_snake),*) => (
                    		#field_names_joined,
                     		::generator_runtime::model::SerializedWhereValue::Object(vec![#((#variant_data_names::NAME.to_string(), #prisma_values)),*])
                     	)
                    },               }
            ]);

            let accessor_name = cased_ident(&variant_name_string, Case::Snake);

            Some(quote! {
                pub fn #accessor_name<T: From<UniqueWhereParam>>(#(#field_defs),*) -> T {
                    UniqueWhereParam::#variant_name(#(#field_names_snake),*).into()
                }
            })
        }
    }).collect::<TokenStream>();

    let (result, field_variants): (_, Vec<_>) = model
        .fields()
        .filter(|field| field.ast_field().field_type.as_unsupported().is_none())
        .map(|field| generate_field(field, args))
        .unzip();

    variants.extend(field_variants.into_iter().flatten());

    let variants = generate_variants(&variants);

    FieldModule {
        model_data: quote! {
            #compound_field_accessors
            #variants
        },
        field_data: result,
    }
}

enum Variant {
    Base {
        definition: TokenStream,
        match_arm: TokenStream,
    },
    Unique {
        field_name: String,
        field_required_type: TokenStream,
        read_filter_name: String,
        optional: bool,
        value: TokenStream,
    },
    CompoundUnique {
        field_names_string: String,
        variant_data_types: Vec<TokenStream>,
        match_arm: TokenStream,
    },
}

impl Variant {
    pub fn unique(field: ScalarFieldWalker, read_filter: &Filter, module_path: &TokenStream) -> Self {
        Self::Unique {
            field_name: field.name().to_owned(),
            field_required_type: field
                .scalar_field_type()
                .to_tokens(
                    module_path,
                    match field.ast_field().arity {
                        FieldArity::Optional => FieldArity::Required,
                        a @ (FieldArity::Required | FieldArity::List) => a,
                    },
                    field.db,
                )
                .unwrap(),
            read_filter_name: read_filter.name.clone(),
            optional: field.ast_field().arity.is_optional(),
            value: {
                let value = field
                    .scalar_field_type()
                    .to_prisma_tokens(
                        &format_ident!("value"),
                        match field.ast_field().arity {
                            FieldArity::Optional => FieldArity::Required,
                            a @ (FieldArity::Required | FieldArity::List) => a,
                        },
                    )
                    .unwrap();

                quote!(::generator_runtime::model::SerializedWhereValue::Value(#value))
            },
        }
    }
}

fn unique_field_combos(model: ModelWalker) -> Vec<Vec<ScalarFieldWalker>> {
    let mut combos = model
        .indexes()
        .filter(|field| field.is_unique())
        .map(|unique| {
            unique
                .fields()
                .filter_map(|field| model.scalar_fields().find(|mf| mf.field_id() == field.field_id()))
                .collect()
        })
        .collect::<Vec<_>>();

    if let Some(primary_key) = model.primary_key() {
        let primary_key_is_also_unique = model.indexes().any(|i| {
            primary_key.contains_exactly_fields(
                i.fields()
                    .filter_map(IndexFieldWalker::as_scalar_field)
                    .collect::<Vec<_>>()
                    .into_iter(),
            )
        });

        if !primary_key_is_also_unique {
            combos.push(
                primary_key
                    .fields()
                    .filter_map(|field| model.scalar_fields().find(|mf| mf.field_id() == field.field_id()))
                    .collect(),
            );
        }
    }

    combos
}

fn generate_variants(variants: &[Variant]) -> TokenStream {
    let (where_variants, to_serialized_where): (Vec<_>, Vec<_>) = variants
        .iter()
        .filter_map(|variant| match variant {
            Variant::Base { definition, match_arm } => Some((definition.clone(), Some(match_arm))),
            Variant::Unique { .. } | Variant::CompoundUnique { .. } => None,
        })
        .unzip();

    let (optional_unique_impls, (unique_variants, unique_to_serialized_where)): (Vec<_>, (Vec<_>, Vec<_>)) = variants
        .iter()
        .filter_map(|variant| match variant {
            Variant::Unique {
                field_name,
                field_required_type,
                read_filter_name,
                optional,
                value,
            } => {
                let field_pascal = cased_ident(field_name, Case::Pascal);
                let field_snake = cased_ident(field_name, Case::Snake);

                let variant_name = format_ident!("{}Equals", &field_pascal);
                let filter_enum = format_ident!("{}Filter", &read_filter_name);

                let optional_unique_impls = optional.then(|| {
                    quote! {
                        impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Equals> for WhereParam {
                            type Arg = Option<#field_required_type>;

                            fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                                Self::#field_pascal(super::super::prisma::read_filters::#filter_enum::Equals(arg))
                            }
                        }

                        impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Equals> for UniqueWhereParam {
                            type Arg = #field_required_type;

                            fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                                Self::#variant_name(arg)
                            }
                        }
                    }
                });

                Some((
                    optional_unique_impls,
                    (
                        quote!(#variant_name(#field_required_type)),
                        quote!(UniqueWhereParam::#variant_name(value) => (#field_name, #value)),
                    ),
                ))
            }
            Variant::CompoundUnique {
                field_names_string,
                variant_data_types,
                match_arm,
            } => {
                let variant_name = format_ident!("{}Equals", field_names_string);

                Some((
                    None,
                    (quote!(#variant_name(#(#variant_data_types),*)), quote!(#match_arm)),
                ))
            }
            Variant::Base { .. } => None,
        })
        .unzip();

    quote! {
        #[derive(Clone, Debug)]
        pub enum WhereParam {
            #(#where_variants),*
        }

        impl ::generator_runtime::model::WhereInput for WhereParam {
            fn serialize(self) -> ::generator_runtime::model::SerializedWhereInput {
                let (name, value): (&str, ::generator_runtime::model::SerializedWhereValue) = match self {
                    #(#to_serialized_where),*
                };

                ::generator_runtime::model::SerializedWhereInput::new(
                    name.to_owned(),
                    value,
                )
            }
        }

        #[derive(Clone, Debug)]
        pub enum UniqueWhereParam {
            #(#unique_variants),*
        }

        impl ::generator_runtime::model::WhereInput for UniqueWhereParam {
            fn serialize(self) -> ::generator_runtime::model::SerializedWhereInput {
                let (name, value) = match self {
                    #(#unique_to_serialized_where),*
                };

                ::generator_runtime::model::SerializedWhereInput::new(
                    name.to_owned(),
                    value,
                )
            }
        }

        #(#optional_unique_impls)*

        impl From<::generator_runtime::operator::Operator<Self>> for WhereParam {
            fn from(op: ::generator_runtime::operator::Operator<Self>) -> Self {
                match op {
                    ::generator_runtime::operator::Operator::Not(value) => Self::Not(value),
                    ::generator_runtime::operator::Operator::And(value) => Self::And(value),
                    ::generator_runtime::operator::Operator::Or(value) => Self::Or(value),
                }
            }
        }
    }
}

fn generate_field(field: FieldWalker, args: &GeneratorArgs) -> ((String, TokenStream), Vec<Variant>) {
    let mut variants = vec![];

    let field_name_raw = field.name();
    let field_name_pascal = cased_ident(field_name_raw, Case::Pascal);
    let field_name_snake = cased_ident(field_name_raw, Case::Snake);

    let field_type = field.to_tokens(&quote!());

    let arity = field.ast_field().arity;

    let is_null_variant = format_ident!("{field_name_pascal}IsNull");
    let equals_variant = format_ident!("{field_name_pascal}Equals");

    let data = if let Some(refiner) = field.refine() {
        match refiner {
            query_structure::walkers::RefinedFieldWalker::Scalar(scalar_field) => {
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

                        let read_fns = args.read_filter(scalar_field).map(|read_filter| {
                        let filter_name = format_ident!("{}Filter", &read_filter.name);

                        variants.push(Variant::Base {
						    definition: quote!(#field_name_pascal(super::super::prisma::read_filters::#filter_name)),
						    match_arm: quote! {
							    Self::#field_name_pascal(value) => (
								    #field_name_snake::NAME,
								    value.into()
							    )
						    },
					    });

                        let (names, (names_pascal, types)): (Vec<_>, (Vec<_>, Vec<_>)) =
                            read_filter
                                .fields
                                .iter()
                                .filter_map(|inner_field| {
                                    let name = match inner_field.name.as_str() {
                                        "equals" => return None,
                                        "in" => "inVec",
                                        "notIn" => "notInVec",
                                        n => n,
                                    };

                                    let method_name_snake = cased_ident(name, Case::Snake);
                                    let method_name_pascal = cased_ident(name, Case::Pascal);

                                    let typ = inner_field.to_tokens(&prefix /*quote!()*/);

                                    // Some(quote!(fn #method_name_snake(_: #typ) -> #method_name_pascal;))
                                    Some((method_name_snake, (method_name_pascal, typ)))
                                })
                                .unzip();

                        // Add equals query functions. Unique/Where enum variants are added in unique/primary key sections earlier on.
                        let model = field.model();
					    let equals = match (
    						scalar_field.is_single_pk(),
						    model.indexes().any(|idx| {
							    let mut fields = idx.fields();
							    idx.is_unique() && fields.len() == 1 && fields.next().map(IndexFieldWalker::field_id) == Some(scalar_field.field_id())
						    }),
    						arity.is_optional()
	    				) {
		    				(true, _, _) | (_, true, false) => quote! {
							    pub fn equals<T: From<Equals>>(value: #prefix #field_type) -> T {
							    	Equals(value).into()
							    }

    							impl From<Equals> for UniqueWhereParam {
    								fn from(Equals(v): Equals) -> Self {
    									UniqueWhereParam::#equals_variant(v)
    								}
    							}
    						},
    						(_, true, true) => quote! {
    							pub fn equals<T: ::generator_runtime::TODO::FromOptionalUniqueArg<Equals>>(value: T::Arg) -> T {
    								T::from_arg(value)
    							}
    						},
    						(false, false, _) => quote! {
    							pub fn equals<T: From<Equals>>(value: #prefix #field_type) -> T {
    								Equals(value).into()
    							}
    						}
					    };

                        quote! {
                            pub struct Equals(pub #prefix #field_type);

						    #equals

						    impl From<Equals> for WhereParam {
						    	fn from(Equals(v): Equals) -> Self {
							    	WhereParam::#field_name_pascal(prisma::read_filters::#filter_name::Equals(v))
						    	}
					    	}

                            #(
                                pub fn #names(value: #types) -> WhereParam {
                                    WhereParam::#field_name_pascal(prisma::read_filters::#filter_name::#names_pascal(value))
                                }
                            )*
                        }
                    });

                        quote! {
                            #read_fns
                        }
                    }
                }
            }
            query_structure::walkers::RefinedFieldWalker::Relation(relation_field) => {
                let relation_model_name = cased_ident(relation_field.related_model().name(), Case::Snake);

                if arity == FieldArity::Optional {
                    variants.push(Variant::Base {
                        definition: quote!(#is_null_variant),
                        match_arm: quote! {
                            Self::#is_null_variant => (
                                #field_name_snake::NAME,
                                ::generator_runtime::model::SerializedWhereValue::Value(::generator_runtime::internal::PrismaValue::Null)
                            )
                        },
                    });
                }

                let relation_methods = field.relation_methods().iter().map(|method| {
                    let method_action_string = method.to_case(Case::Camel);
				    let method_name_snake = cased_ident(method, Case::Snake);

                    let variant_name = format_ident!("{}{}", &field_name_pascal, cased_ident(method, Case::Pascal));

				    variants.push(Variant::Base {
					    definition: quote!(#variant_name(Vec<super::#relation_model_name::WhereParam>)),
					    match_arm: quote! {
						    Self::#variant_name(where_params) => (
							    #field_name_snake::NAME,
							    ::generator_runtime::model::SerializedWhereValue::Object(vec![(
								    #method_action_string.to_string(),
								    ::generator_runtime::internal::PrismaValue::Object(
									    where_params
										    .into_iter()
										    .map(::generator_runtime::model::WhereInput::serialize)
    										.map(::generator_runtime::model::SerializedWhereInput::transform_equals)
    										.collect()
								    ),
							    )])
						    )
					    },
				    });

				    quote! {
					    pub fn #method_name_snake(value: Vec<super::super::#relation_model_name::WhereParam>) -> WhereParam {
						    WhereParam::#variant_name(value)
					    }
				    }
                }).collect::<TokenStream>();

                quote! {
                    #relation_methods
                }
            }
        }
    } else {
        quote! {}
    };

    ((field.name().to_owned(), data), variants)
}
