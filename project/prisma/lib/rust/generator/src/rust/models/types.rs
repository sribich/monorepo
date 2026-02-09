use convert_case::Case;
use generator_shared::casing::cased_ident;
use generator_shared::extensions::FieldExtension;
use proc_macro2::TokenStream;
use psl::parser_database::ScalarFieldType;
use query_structure::walkers::ModelWalker;
use quote::quote;

pub fn generate_types(model: ModelWalker) -> TokenStream {
    let selections = model.scalar_fields().filter_map(|field| {
        let field_name = cased_ident(field.name(), Case::Snake);

        Some(match field.scalar_field_type() {
            ScalarFieldType::Enum(_)
            | ScalarFieldType::Extension(_)
            | ScalarFieldType::BuiltInScalar(_)
            | ScalarFieldType::Unsupported(_) => {
                field.to_tokens(&quote!())?;

                quote! {
                    ::generator_runtime::sel(#field_name::NAME)
                }
            }
        })
    });

    quote! {
        #[derive(Clone, Debug)]
        pub struct Types;

        impl ::generator_runtime::model::Model for Types {
            type Data = Data;
            type Where = WhereParam;
            type WhereUnique = UniqueWhereParam;
            type UncheckedSet = UncheckedSetParam;
            type Set = SetParam;
            type With = WithParam;
            type OrderBy = OrderByWithRelationParam;
            type Cursor = UniqueWhereParam;

            const NAME: &'static str = NAME;

            fn scalar_selections() -> Vec<::generator_runtime::internal::Selection> {
                vec![#(#selections),*]
            }
        }
    }
}
