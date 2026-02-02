use convert_case::Case;
use generator_shared::{casing::cased_ident, extensions::FieldExtension};
use proc_macro2::TokenStream;
use psl::parser_database::ScalarFieldType;
use query_structure::walkers::ModelWalker;
use quote::quote;

use super::get_required_model_fields;

pub fn generate_types(model: ModelWalker) -> TokenStream {
    let create = create(model);
    let create_unchecked = create_unchecked(model);

    quote! {
        #create

        #create_unchecked
    }
}

fn create(model: ModelWalker) -> Option<TokenStream> {
    let model_name = cased_ident(model.name(), Case::Snake);

    let (names, (types, push_wrappers)): (Vec<_>, (Vec<_>, Vec<_>)) = get_required_model_fields(&model)?
        .into_iter()
        .map(|field| {
            (
                cased_ident(field.inner.name(), Case::Snake),
                (field.r#type, field.push_wrapper),
            )
        })
        .unzip();

    Some(quote! {
        #[derive(Clone, Debug)]
        pub struct Create {
            #(pub #names: #types,)*
            pub params: Vec<SetParam>,
        }

        impl Create {
            // pub fn to_query<'db>(self, client: &'db PrismaClient) -> CreateQuery<'db> {
            pub fn to_query(self, client: &PrismaClient) -> CreateQuery {
                client.#model_name().create(#(self.#names,)* self.params)
            }

            pub fn to_params(mut self) -> Vec<SetParam> {
                self.params.extend([
                    #(#names::#push_wrappers(self.#names)),*
                ]);

                self.params
            }
        }

        pub fn create(#(#names: #types,)* params: Vec<SetParam>) -> Create {
            Create {
                #(#names,)*
                params
            }
        }
    })
}

fn create_unchecked(model: ModelWalker) -> Option<TokenStream> {
    let model_name = cased_ident(model.name(), Case::Snake);

    get_required_model_fields(&model)?;

    let (names, types): (Vec<_>, Vec<_>) = model
        .scalar_fields()
        .filter_map(|field| {
            let field_name = cased_ident(field.name(), Case::Snake);

            if !field.is_required() {
                return None;
            }

            Some((
                field_name,
                match field.scalar_field_type() {
                    ScalarFieldType::Enum(_)
                    | ScalarFieldType::Extension(_)
                    | ScalarFieldType::BuiltInScalar(_)
                    | ScalarFieldType::Unsupported(_) => field.to_tokens(&quote!())?,
                },
            ))
        })
        .unzip();

    Some(quote! {
        #[derive(Clone, Debug)]
        pub struct CreateUnchecked {
            #(pub #names: #types,)*
            pub params: Vec<UncheckedSetParam>,
        }

        impl CreateUnchecked {
            // pub fn to_query<'db>(self, client: &'db PrismaClient) -> CreateUncheckedQuery<'db> {
            pub fn to_query(self, client: &PrismaClient) -> CreateUncheckedQuery {
                client.#model_name().create_unchecked(#(self.#names,)* self.params)
            }

            pub fn to_params(mut self) -> Vec<UncheckedSetParam> {
                self.params.extend([
                    #(#names::set(self.#names)),*
                ]);

                self.params
            }
        }

        pub fn create_unchecked(#(#names: #types,)* params: Vec<UncheckedSetParam>) -> CreateUnchecked {
            CreateUnchecked {
                #(#names,)*
                params
            }
        }
    })
}
