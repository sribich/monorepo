use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

pub(crate) struct Module {
    pub name: String,
    pub content: TokenStream,
    pub submodules: Vec<Module>,
}

impl Module {
    pub fn new<T: Into<String>>(name: T, content: TokenStream) -> Self {
        Self {
            name: name.into(),
            content,
            submodules: vec![],
        }
    }

    pub fn add_submodule(&mut self, submodule: Self) {
        self.submodules.push(submodule);
    }

    pub fn flatten(&self) -> TokenStream {
        let content = &self.content;

        let submodule_content = self
            .submodules
            .iter()
            .map(|submodule| {
                let name = format_ident!("{}", submodule.name.to_case(Case::Snake));
                let submodule_content = &submodule.flatten();

                quote!(
                    pub mod #name {
                        #submodule_content
                    }
                )
            })
            .collect::<Vec<_>>();

        quote!(
            #content

            #(#submodule_content)*
        )
    }
}

/// A [`FieldModule`] contains generated data for a given field within a model.
///
/// TODO: I don't think this needs to be pub at all.
pub(crate) struct FieldModule {
    /// Data that should be appended to the model itself.
    pub model_data: TokenStream,
    /// Individual field data that should be moved into a
    /// field-specific submodule.
    pub field_data: HashMap<String, TokenStream>,
}

impl FieldModule {
    pub fn merge(modules: Vec<Self>) -> (TokenStream, Vec<Module>) {
        let (model_data, field_data): (Vec<_>, Vec<_>) = modules
            .into_iter()
            .map(|module| (module.model_data, module.field_data))
            .unzip();

        // let mut field_module = Module::new("field", quote! {});

        let modules = field_data
            .into_iter()
            .flat_map(std::iter::IntoIterator::into_iter)
            .fold(HashMap::new(), |mut acc, (k, v)| {
                let item: &mut Vec<_> = acc.entry(k).or_default();
                item.push(v);
                acc
            })
            .into_iter()
            .map(|(field_name, data)| {
                Module::new(
                    &field_name,
                    quote! {
                        use super::{
                            SetParam,
                            UncheckedSetParam,
                            UniqueWhereParam,
                            WhereParam,
                            WithParam
                        };
                        use super::super::super::prisma::{
                            self,
                            *
                        };

                        // use super::super::{*, super::*, super::super::*};
                        // use super::super::{_prisma::*, *};
                        // use super::{WhereParam, UniqueWhereParam, WithParam, SetParam, UncheckedSetParam};

                        pub const NAME: &str = #field_name;

                        #(#data)*
                    },
                )
            })
            .collect();
        // .for_each(|module| field_module.add_submodule(module));

        (quote!(#(#model_data)*), modules)
    }
}
