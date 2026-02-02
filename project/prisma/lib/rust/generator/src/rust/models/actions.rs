use convert_case::Case;
use generator_shared::{casing::cased_ident, extensions::FieldExtension};
use proc_macro2::TokenStream;
use psl::{datamodel_connector::ConnectorCapability, parser_database::ScalarFieldType};
use query_structure::walkers::ModelWalker;
use quote::quote;

use super::{RequiredField, get_required_model_fields};
use crate::args::GeneratorArgs;

pub(super) fn generate_model_actions(model: ModelWalker, args: &GeneratorArgs) -> TokenStream {
    let create = generate_create(model);
    let create_unchecked = generate_create_unchecked(model);
    let create_many = args
        .schema
        .context()
        .connector
        .capabilities()
        .contains(ConnectorCapability::CreateMany)
        .then(|| generate_create_many(model))
        .flatten();

    let upsert = generate_upsert(model);

    quote! {
        #[derive(Clone)]
        pub struct Actions<'db> {
            pub client: &'db ::generator_runtime::client::InternalClient,
        }

        impl<'db> Actions<'db> {
            #create

            #create_unchecked

            #create_many

            #upsert

            pub fn find_unique(self, r#where: UniqueWhereParam) -> FindUniqueQuery<'db> {
                FindUniqueQuery::new(
                    self.client,
                    r#where
                )
            }

            pub fn find_first(self, r#where: Vec<WhereParam>) -> FindFirstQuery<'db> {
                FindFirstQuery::new(
                    self.client,
                    r#where
                )
            }

            pub fn find_many(self, r#where: Vec<WhereParam>) -> FindManyQuery<'db> {
                FindManyQuery::new(
                    self.client,
                    r#where
                )
            }

            pub fn update(self, r#where: UniqueWhereParam, params: Vec<SetParam>) -> UpdateQuery<'db> {
                UpdateQuery::new(
                    self.client,
                    r#where,
                    params,
                    vec![]
                )
            }

            pub fn update_unchecked(self, r#where: UniqueWhereParam, params: Vec<UncheckedSetParam>) -> UpdateUncheckedQuery<'db> {
                UpdateUncheckedQuery::new(
                    self.client,
                    r#where,
                    params.into_iter().map(Into::into).collect(),
                    vec![]
                )
            }

            pub fn update_many(self, r#where: Vec<WhereParam>, params: Vec<SetParam>) -> UpdateManyQuery<'db> {
                UpdateManyQuery::new(
                    self.client,
                    r#where,
                    params,
                )
            }

            pub fn delete(self, r#where: UniqueWhereParam) -> DeleteQuery<'db> {
                DeleteQuery::new(
                    self.client,
                    r#where,
                    vec![]
                )
            }

            pub fn delete_many(self, r#where: Vec<WhereParam>) -> DeleteManyQuery<'db> {
                DeleteManyQuery::new(
                    self.client,
                    r#where
                )
            }

            pub fn count(self, r#where: Vec<WhereParam>) -> CountQuery<'db> {
                CountQuery::new(
                    self.client,
                    r#where
                )
            }
        }
    }
}

fn generate_create(model: ModelWalker) -> Option<TokenStream> {
    let (names, (types, wrappers)): (Vec<_>, (Vec<_>, Vec<_>)) = get_required_model_fields(&model)?
        .into_iter()
        .map(|field| {
            (
                cased_ident(field.inner.name(), Case::Snake),
                (field.r#type, field.push_wrapper),
            )
        })
        .unzip();

    Some(quote! {
        pub fn create(self, #(#names: #types,)* mut params: Vec<SetParam>) -> CreateQuery<'db> {
            params.extend([
                #(#names::#wrappers(#names)),*
            ]);

            CreateQuery::new(self.client, params)
        }
    })
}

fn generate_create_unchecked(model: ModelWalker) -> Option<TokenStream> {
    let _: Vec<RequiredField<'_>> = get_required_model_fields(&model)?;

    let (names, types): (Vec<_>, Vec<_>) = model
        .scalar_fields()
        .filter_map(|field| {
            let field_name_rust = cased_ident(field.name(), Case::Snake);

            if !field.is_required() {
                return None;
            }

            Some((
                field_name_rust,
                match field.scalar_field_type() {
                    ScalarFieldType::Enum(_)
                    | ScalarFieldType::Extension(_)
                    | ScalarFieldType::BuiltInScalar(_)
                    | ScalarFieldType::Unsupported(_) => field.to_tokens(&quote!(super::))?,
                },
            ))
        })
        .unzip();

    Some(quote! {
        pub fn create_unchecked(self, #(#names: #types,)* mut params: Vec<UncheckedSetParam>) -> CreateUncheckedQuery<'db> {
            params.extend([
                #(#names::set(#names)),*
            ]);

            CreateUncheckedQuery::new(
                self.client,
                params.into_iter().map(Into::into).collect()
            )
        }
    })
}

fn generate_create_many(model: ModelWalker) -> Option<TokenStream> {
    model
        .scalar_fields()
        .all(|field| !field.is_required() || !field.is_unsupported())
        .then(|| {
            quote! {
                pub fn create_many(self, data: Vec<CreateUnchecked>) -> CreateManyQuery<'db> {
                    let data = data.into_iter().map(CreateUnchecked::to_params).collect();

                    CreateManyQuery::new(
                        self.client,
                        data,
                    )
                }
            }
        })
}

fn generate_upsert(model: ModelWalker) -> Option<TokenStream> {
    let _: Vec<RequiredField<'_>> = get_required_model_fields(&model)?;

    Some(quote! {
        pub fn upsert(
            self,
            r#where: UniqueWhereParam,
            create: Create,
            update: Vec<SetParam>
        ) -> UpsertQuery<'db> {
            UpsertQuery::new(self.client, r#where, create.to_params(), update)
        }
    })
}
