use convert_case::Case;
use dmmf::serialization_ast::{
    DmmfInputField, DmmfInputType, DmmfSchema, DmmfTypeReference, TypeLocation,
};
use proc_macro2::TokenStream;
use psl::parser_database::{ParserDatabase, ScalarFieldType, ScalarType};
use query_structure::{
    FieldArity,
    walkers::{
        FieldWalker, ModelWalker, RefinedFieldWalker, ScalarFieldWalker,
    },
};
use quote::{format_ident, quote};
use syn::Ident;

use super::casing::cased_ident;

pub trait ModelExtension {
    fn field_has_relation(self, field: ScalarFieldWalker) -> bool;
}

impl ModelExtension for ModelWalker<'_> {
    fn field_has_relation(self, field: ScalarFieldWalker) -> bool {
        self.relation_fields().any(|relation_field| {
            relation_field
                .fields()
                .is_some_and(|mut fields| fields.any(|f| f.field_id() == field.field_id()))
        })
    }
}

pub trait FieldExtension {
    fn to_tokens(&self, prefix: &TokenStream) -> Option<TokenStream>;

    fn is_required(&self) -> bool;

    fn relation_methods(&self) -> &'static [&'static str];
}

impl FieldExtension for FieldWalker<'_> {
    fn to_tokens(&self, prefix: &TokenStream) -> Option<TokenStream> {
        let it = self.refine()?;
        match it {
                    RefinedFieldWalker::Scalar(scalar_field) => scalar_field.to_tokens(prefix),
                    RefinedFieldWalker::Relation(relation_field) => {
                        let model_name = cased_ident(relation_field.related_model().name(), Case::Snake);

                        Some(
                            self.ast_field()
                                .arity
                                .to_tokens(&quote! { #prefix::#model_name::Data }),
                        )
                    },
                }
    }

    fn is_required(&self) -> bool {
        self.ast_field().arity.is_required()
            && self
                .refine()
                .map(|it| match it {
                    RefinedFieldWalker::Scalar(field) => field.is_required(),
                    RefinedFieldWalker::Relation(_) => true,
                })
                .unwrap_or(false)
    }

    fn relation_methods(&self) -> &'static [&'static str] {
        if self.ast_field().arity.is_list() {
            &["some", "every", "none"]
        } else {
            &["is", "is_not"]
        }
    }
}

impl FieldExtension for ScalarFieldWalker<'_> {
    fn to_tokens(&self, prefix: &TokenStream) -> Option<TokenStream> {
        self.scalar_field_type()
            .to_tokens(prefix, self.ast_field().arity, self.db)
    }

    fn is_required(&self) -> bool {
        self.ast_field().arity.is_required()
            && !self.is_updated_at()
            && self.default_value().is_none()
    }

    fn relation_methods(&self) -> &'static [&'static str] {
        &[]
    }
}

pub trait FieldArityExtension {
    fn to_tokens(&self, inner_type: &TokenStream) -> TokenStream;

    fn to_prisma_type(&self, var: &Ident, value: TokenStream) -> TokenStream;
}

impl FieldArityExtension for FieldArity {
    fn to_tokens(&self, inner_type: &TokenStream) -> TokenStream {
        match self {
            Self::Required => quote! { #inner_type },
            Self::Optional => quote! { Option<#inner_type> },
            Self::List => quote! { Vec<#inner_type> },
        }
    }

    fn to_prisma_type(&self, var: &Ident, value: TokenStream) -> TokenStream {
        match self {
            Self::Required => value,
            Self::Optional => quote! {
                #var.map(|#var| #value).unwrap_or_else(|| ::generator_runtime::internal::PrismaValue::Null)
            },
            Self::List => quote! {
                ::generator_runtime::internal::PrismaValue::List(
                    #var.into_iter().map(|#var| #value).collect()
                )
            },
        }
    }
}

pub trait ScalarFieldTypeExtension {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream>;

    fn to_prisma_tokens(&self, var: &Ident, arity: FieldArity) -> Option<TokenStream>;
}

impl ScalarFieldTypeExtension for ScalarFieldType {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream> {
        let base = match *self {
            Self::Enum(id) => {
                let name = cased_ident(db.walk(id).name(), Case::Pascal);

                quote! { #prefix #name }
            },
            Self::BuiltInScalar(r#type) => r#type.to_tokens(),
            Self::Extension(_) | Self::Unsupported(_) => return None,
        };

        Some(arity.to_tokens(&base))
    }

    fn to_prisma_tokens(&self, var: &Ident, arity: FieldArity) -> Option<TokenStream> {
        let scalar_converter = match self {
            Self::BuiltInScalar(typ) => typ.to_prisma(var),
            Self::Enum(_) => {
                quote!(::generator_runtime::internal::PrismaValue::Enum(#var.to_string()))
            },
            Self::Extension(_) | Self::Unsupported(_) => return None,
        };

        Some(arity.to_prisma_type(var, scalar_converter))
    }
}

pub trait ScalarTypeExtension {
    fn to_dmmf_string(&self) -> String;
    fn to_prisma(&self, ident: &Ident) -> TokenStream;
    fn to_tokens(&self) -> TokenStream;
}

impl ScalarTypeExtension for ScalarType {
    fn to_dmmf_string(&self) -> String {
        match self {
            Self::Boolean => "Bool".to_owned(),
            Self::Int
            | Self::BigInt
            | Self::Float
            | Self::String
            | Self::DateTime
            | Self::Json
            | Self::Bytes
            | Self::Decimal => self.as_str().to_owned(),
        }
    }

    fn to_prisma(&self, ident: &Ident) -> TokenStream {
        let qualifier = quote!(::generator_runtime::internal::PrismaValue);

        match self {
            Self::Int => quote!(#qualifier::Int(i64::from(#ident))), // #ident as i64
            Self::BigInt => quote!(#qualifier::BigInt(#ident)),
            // Self::Float => quote!(#qualifier::Float(#ident)),
            Self::Float => quote!(
                #qualifier::Float(
                    <::generator_runtime::internal::bigdecimal::BigDecimal as ::generator_runtime::internal::bigdecimal::FromPrimitive>
                        ::from_f64(#ident)
                            .unwrap()
                            .normalized()
                )
            ),
            Self::Boolean => quote!(#qualifier::Boolean(#ident)),
            Self::String => quote!(#qualifier::String(#ident)),
            Self::DateTime => quote!(#qualifier::DateTime(#ident)),
            // #qualifier::Json(::generator_runtime::internal::serde_json::to_string(&#ident).
            // unwrap())
            Self::Json => quote!(todo!("Not implemented 2")),
            Self::Bytes => quote!(#qualifier::Bytes(#ident)),
            Self::Decimal => quote!(#qualifier::String(#ident.to_string())),
        }
    }

    fn to_tokens(&self) -> TokenStream {
        let ident = format_ident!("{}", self.as_str());

        quote!(#ident)
    }
}

pub trait DmmfSchemaExtension {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType>;
}

impl DmmfSchemaExtension for DmmfSchema {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType> {
        self.input_object_types
            .get("prisma")
            .and_then(|types| types.iter().find(|item| item.name == name))
    }
}

pub trait DmmfInputFieldExt {
    fn arity(&self) -> FieldArity;

    fn to_tokens(&self, prefix: &TokenStream) -> TokenStream;
    fn to_prisma_tokens(&self, var: &Ident) -> TokenStream;
}

impl DmmfInputFieldExt for DmmfInputField {
    fn arity(&self) -> FieldArity {
        let input_type = self
            .input_types
            .iter()
            .find(|typ| !matches!(typ.location, TypeLocation::Scalar if typ.typ == "Null"))
            .expect(&format!("No type found for field {}", self.name));

        if input_type.is_list {
            FieldArity::List
        } else if self.is_nullable {
            FieldArity::Optional
        } else {
            FieldArity::Required
        }
    }

    fn to_tokens(&self, prefix: &TokenStream) -> TokenStream {
        let input_type = self
            .input_types
            .iter()
            .find(|typ| !matches!(typ.location, TypeLocation::Scalar if typ.typ == "Null"))
            .expect(&format!("No type found for field {}", self.name));

        let arity = self.arity();

        match input_type.location {
            TypeLocation::Scalar => arity.to_tokens(
                &ScalarType::try_from_str(&input_type.typ, true)
                    .unwrap()
                    .to_tokens(),
            ),
            TypeLocation::EnumTypes => {
                let typ: TokenStream = input_type.typ.parse().unwrap();
                arity.to_tokens(&quote!(#prefix #typ))
            },
            TypeLocation::InputObjectTypes => {
                let typ: TokenStream = input_type.typ.parse().unwrap();
                quote!(Vec<#prefix #typ>)
            },
            TypeLocation::OutputObjectTypes | TypeLocation::FieldRefTypes => todo!(),
        }
    }

    // TODO: We should probably rename this to PrismaAction or something along
    //       those lines. Not sure.
    fn to_prisma_tokens(&self, var: &Ident) -> TokenStream {
        let input_type = self
            .input_types
            .iter()
            .find(|typ| !matches!(typ.location, TypeLocation::Scalar if typ.typ == "Null"))
            .expect(&format!("No type found for field {}", self.name));

        let arity = self.arity();

        match input_type.location {
            TypeLocation::Scalar => arity.to_prisma_type(
                var,
                ScalarType::try_from_str(&input_type.typ, true)
                    .unwrap()
                    .to_prisma(var),
            ),
            TypeLocation::EnumTypes => arity.to_prisma_type(
                var,
                quote!(::generator_runtime::internal::PrismaValue::Enum(#var.to_string())),
            ),
            TypeLocation::InputObjectTypes => {
                quote!(::generator_runtime::internal::PrismaValue::Object(#var.into_iter().map(Into::into).collect()))
            },
            TypeLocation::OutputObjectTypes | TypeLocation::FieldRefTypes => todo!(),
        }
    }
}

pub trait DmmfTypeReferenceExt {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream>;
}

impl DmmfTypeReferenceExt for DmmfTypeReference {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream> {
        Some(match self.location {
            TypeLocation::Scalar => {
                ScalarFieldType::BuiltInScalar(ScalarType::try_from_str(&self.typ, true).unwrap())
                    .to_tokens(prefix, arity, db)?
            },
            TypeLocation::EnumTypes => {
                let enum_name_pascal = cased_ident(&self.typ, Case::Pascal);
                quote!(#prefix #enum_name_pascal)
            },
            TypeLocation::InputObjectTypes => {
                let typ = match &self.typ {
                    t if t.ends_with("OrderByWithRelationInput") => {
                        let model_name = t.replace("OrderByWithRelationInput", "");
                        let model_name_snake = cased_ident(&model_name, Case::Snake);

                        quote!(#model_name_snake::OrderByWithRelationParam)
                    },
                    t if t.ends_with("OrderByRelationAggregateInput") => {
                        let model_name = t.replace("OrderByRelationAggregateInput", "");
                        let model_name_snake = cased_ident(&model_name, Case::Snake);

                        quote!(#model_name_snake::OrderByRelationAggregateParam)
                    },
                    t if t.ends_with("OrderByInput") => {
                        let model_name = t.replace("OrderByInput", "");
                        let model_name_snake = cased_ident(&model_name, Case::Snake);

                        quote!(#model_name_snake::OrderByParam)
                    },
                    _ => return None,
                };

                quote!(Vec<#prefix #typ>)
            },
            TypeLocation::OutputObjectTypes | TypeLocation::FieldRefTypes => return None,
        })
    }
}
