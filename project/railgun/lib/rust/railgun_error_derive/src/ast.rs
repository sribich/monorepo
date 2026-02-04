use darling::FromAttributes;
use quote::{quote, quote_spanned};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Generics, Ident, Index, Member, Result,
    Type, spanned::Spanned,
};

use crate::attributes::{ContainerAttributes, FieldAttributes, VariantAttributes};

#[derive(Debug)]
pub(crate) enum Input<'node> {
    Struct(Struct<'node>),
    Enum(Enum<'node>),
}

#[derive(Debug)]
pub(crate) struct Struct<'node> {
    pub original: &'node syn::DataStruct,
    pub kind: StructKind,

    pub container_attributes: ContainerAttributes,
    pub variant_attributes: VariantAttributes,

    pub has_location: bool,
    pub has_source: bool,
    pub has_external_cause: bool,

    pub location_field: Option<Field<'node>>,
    pub source_field: Option<Field<'node>>,
    pub error_field: Option<Field<'node>>,
}

#[derive(Debug)]
pub(crate) struct Enum<'node> {
    pub ident: Ident,
    pub generics: &'node Generics,
    pub variants: Vec<Variant<'node>>,
    pub container_attributes: ContainerAttributes,
}

#[derive(Debug)]
pub struct Variant<'node> {
    pub original: &'node syn::Variant,
    pub attributes: VariantAttributes,
    pub ident: Ident,
    pub fields: Vec<Field<'node>>,
}

impl<'node> Struct<'node> {
    pub fn from_syn(node: &'node DeriveInput, data: &'node DataStruct) -> Result<Self> {
        let fields = fields
            .into_iter()
            .filter(|field| {
                if let Some(ident) = &field.original.ident {
                    ident != "location" && ident != "source" && ident != "error"
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        Ok(Struct {
            fields,
            container_attributes: ContainerAttributes::from_attributes(&node.attrs)?,
            variant_attributes: VariantAttributes::from_attributes(&node.attrs)?,
            has_location: location_field.is_some(),
            has_source: source_field.is_some(),
            has_external_cause: error_field.is_some(),
            location_field,
            source_field,
            error_field,
        })
    }

    pub(crate) fn to_display_body(&self) -> proc_macro2::TokenStream {
        let name = &self.ident;

        let display = if let Some(attr) = &self.container_attributes.display {
            let str = &attr.fmt;
            let rest = &attr.paths;

            quote!(#str, #(#rest),*)
        } else {
            quote!("{self:?}")
        };

        let fields = &self
            .original
            .fields
            .iter()
            .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
            .collect::<Vec<_>>();

        let span = self.original.struct_token.span();

        match (self.has_location, self.has_source, self.has_external_cause) {
            (true, true, _) => quote_spanned! {span=>
                buf.push(format!("{layer}: {}, at {}", format!(#display), self.location));
                source.display(layer + 1, buf);
            },
            (true, false, true) => quote_spanned! {span=>
                buf.push(format!("{layer}: {}, at {}", format!(#display), self.location));
                buf.push(format!("{}: {:?}", layer + 1, error));
            },
            (true, false, false) => quote_spanned! {span=>
                buf.push(format!("{layer}: {}, at {}", format!(#display), self.location));
            },
            (false, true, _) => quote_spanned! {span=>
                buf.push(format!("{layer}: {}", format!(#display)));
                source.display(layer + 1, buf);
            },
            (false, false, true) => quote_spanned! {span=>
                buf.push(format!("{layer}: {}", format!(#display)));
                buf.push(format!("{}: {:?}", layer + 1, error));
            },
            (false, false, false) => quote_spanned! {span=>
                buf.push(format!("{layer}: {}", format!(#display)));
            },
        }
    }

    pub(crate) fn to_next_body(&self) -> proc_macro2::TokenStream {
        let name = &self.ident;

        let fields = &self
            .original
            .fields
            .iter()
            .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
            .collect::<Vec<_>>();

        let span = self.original.struct_token.span();

        if self.has_source {
            quote_spanned! {span=>
                Some(source)
            }
        } else {
            quote_spanned! {span=>
                None
            }
        }
    }
}

impl<'node> Enum<'node> {
    pub fn from_syn(node: &'node DeriveInput, data: &'node DataEnum) -> Result<Self> {
        let variants = data
            .variants
            .iter()
            .map(|node| {
                let variant = Variant::from_syn(node)?;

                Ok(variant)
            })
            .collect::<Result<_>>()?;

        Ok(Enum {
            ident: node.ident.clone(),
            generics: &node.generics,
            variants,
            container_attributes: ContainerAttributes::from_attributes(&node.attrs)?,
        })
    }
}

impl<'node> Variant<'node> {
    fn from_syn(variant: &'node syn::Variant) -> Result<Self> {
        let fields = Field::multiple_from_syn(&variant.fields)?;

        let [location_field, source_field, error_field] =
            ["location", "source", "error"].map(|name| {
                fields
                    .iter()
                    .find(|field| {
                        if let Some(ident) = &field.original.ident {
                            ident == name
                        } else {
                            false
                        }
                    })
                    .cloned()
            });

        let fields = fields
            .into_iter()
            .filter(|field| {
                if let Some(ident) = &field.original.ident {
                    ident != "location" && ident != "source" && ident != "error"
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        Ok(Variant {
            original: variant,
            attributes: VariantAttributes::from_attributes(&variant.attrs)?,
            ident: variant.ident.clone(),
            fields,
            has_location: location_field.is_some(),
            has_source: source_field.is_some(),
            has_external_cause: error_field.is_some(),
            location_field,
            source_field,
            error_field,
        })
    }

    pub(crate) fn to_display_match_arm(&self) -> proc_macro2::TokenStream {
        let name = &self.ident;

        let display = if let Some(attr) = &self.attributes.display {
            let str = &attr.fmt;
            let rest = &attr.paths;

            quote!(#str, #(#rest),*)
        } else {
            quote!("Missing display")
        };

        let fields = &self
            .original
            .fields
            .iter()
            .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
            .collect::<Vec<_>>();

        let span = self.original.span();

        match (self.has_location, self.has_source, self.has_external_cause) {
            (true, true, _) => quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),*, } => {
                    buf.push(format!("{layer}: {}, at {}", format!(#display), location));
                    source.display(layer + 1, buf);
                }
            },
            (true, false, true) => quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),* } => {
                    buf.push(format!("{layer}: {}, at {}", format!(#display), location));
                    buf.push(format!("{}: {:?}", layer + 1, error));
                }
            },
            (true, false, false) => quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),* } => {
                    buf.push(format!("{layer}: {}, at {}", format!(#display), location));
                }
            },
            (false, true, _) => quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),* } => {
                    buf.push(format!("{layer}: {}", format!(#display)));
                    source.display(layer + 1, buf);
                }
            },
            (false, false, true) => quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),* } => {
                    buf.push(format!("{layer}: {}", format!(#display)));
                    buf.push(format!("{}: {:?}", layer + 1, error));
                }
            },
            (false, false, false) => quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),* } => {
                    buf.push(format!("{layer}: {}", format!(#display)));
                }
            },
        }
    }

    pub(crate) fn to_next_match_arm(&self) -> proc_macro2::TokenStream {
        let name = &self.ident;

        let fields = &self
            .original
            .fields
            .iter()
            .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
            .collect::<Vec<_>>();

        let span = self.original.span();

        if self.has_source {
            quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),* } => {
                    Some(source)
                }
            }
        } else {
            quote_spanned! {span=>
                #[allow(unused_variables)]
                #name { #(#fields),* } => {
                    None
                }
            }
        }
    }
}
