use convert_case::Case;
use generator_shared::casing::cased_ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::GeneratorArgs;

pub fn generate_model_enums(args: &GeneratorArgs) -> TokenStream {
    let enums = args.dmmf.data_model.enums.iter().map(|model_enum| {
        let enum_name = cased_ident(&model_enum.name, Case::Pascal);

        let enum_variants = model_enum
            .values
            .iter()
            .map(|value| {
                let prisma_variant = &value.name;
                let rust_variant = cased_ident(prisma_variant, Case::Pascal);

                let docs = if let Some(documentation) = &value.documentation {
                    quote!(#[doc = #documentation])
                } else {
                    quote!()
                };

                quote! {
                    #docs
                    #[serde(rename=#prisma_variant)]
                    #rust_variant
                }
            })
            .collect::<Vec<_>>();

        let match_arms = model_enum
            .values
            .iter()
            .map(|value| {
                let prisma_name = &value.name;
                let rust_name = cased_ident(prisma_name, Case::Pascal);

                quote! {
                    Self::#rust_name => #prisma_name
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #[derive(Clone, Copy, Debug, Eq, PartialEq, ::typegen::Typegen, ::serde::Serialize, ::serde::Deserialize)]
            pub enum #enum_name {
                #(#enum_variants),*
            }

            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let result = match self {
                        #(#match_arms),*
                    };

                    write!(f, "{result}")
                }
            }
        }
    });

    quote! {
        #(#enums)*
    }
}
