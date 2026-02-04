use darling::FromAttributes;
use macro_util::attribute::{parse_attributes, Attribute, AttributeValue};
use quote::{quote, ToTokens};
use syn::Result;

use crate::util::get_crate_name;

pub struct DocAttribute(pub String);

impl DocAttribute {
    pub fn from_attributes(attributes: &[syn::Attribute]) -> Result<Self> {
        let mut result = String::new();

        for attribute in parse_attributes(attributes)?
            .iter()
            .filter(|attribute| attribute.key == "doc")
        {
            match &attribute.value {
                Some(AttributeValue::Lit(_)) => {
                    let doc = attribute.parse_string()?;

                    if !result.is_empty() {
                        result.push('\n');
                    }

                    result.push_str(&doc);
                },
                _ => {
                    return Err(syn::Error::new(
                        attribute.key.span(),
                        r#"Invalid doc attribute. Expected format `#[doc = "value"]`"#,
                    ));
                },
            }
        }

        Ok(Self(result))
    }
}

#[derive(Clone)]
pub enum Deprecation {
    /// A raw deprecated annotation with no additional information.
    ///
    /// `#[deprecated]`
    Deprecated,
    /// A deprecated annotation with a message and optional `since` version.
    ///
    /// `#[deprecated("Use `foobar()` instead")]
    /// `#[deprecated(since = "1.0.0", message = "Use `foobar` instead")]`
    DeprecatedWithMeta(DeprecationAttributes),
}

#[derive(Clone, Debug, FromAttributes)]
#[darling(attributes(deprecated))]
pub struct DeprecationAttributes {
    note: Option<String>,
    since: Option<String>,
}

impl Deprecation {
    pub fn from_attributes(attributes: &[syn::Attribute]) -> syn::Result<Option<Self>> {
        if let Some(attribute) = parse_attributes(attributes)?
            .iter()
            .find(|attribute| attribute.key == "deprecated")
        {
            match &attribute.value {
                Some(AttributeValue::Lit(_)) => {
                    return Ok(Some(Deprecation::DeprecatedWithMeta(
                        DeprecationAttributes {
                            note: Some(attribute.parse_string()?),
                            since: None,
                        },
                    )));
                },
                Some(AttributeValue::Nested { attributes, .. }) => {
                    let note = attributes
                        .iter()
                        .find(|it| it.key == "note")
                        .map(Attribute::parse_string)
                        .transpose()?;
                    let since = attributes
                        .iter()
                        .find(|it| it.key == "since")
                        .map(Attribute::parse_string)
                        .transpose()?;

                    return Ok(Some(Deprecation::DeprecatedWithMeta(
                        DeprecationAttributes { note, since },
                    )));
                },
                None => return Ok(Some(Deprecation::Deprecated)),
                _ => {
                    return Err(syn::Error::new(
                        attribute.key.span(),
                        "Invalid deprecation attribute",
                    ));
                },
            }
        }

        Ok(None)
    }

    pub fn as_tokens(&self) -> proc_macro2::TokenStream {
        let crate_name = get_crate_name();

        match &self {
            Deprecation::Deprecated => quote! {
                #crate_name::internal::Deprecation::Deprecated
            },
            Deprecation::DeprecatedWithMeta(DeprecationAttributes { note, since }) => {
                let since_tokens = if let Some(cow) = since {
                    quote!(Some(std::borrow::Cow::Borrowed(#cow)))
                } else {
                    quote!(None)
                };

                quote! {
                    #crate_name::internal::Deprecation::DeprecatedWithMeta(
                        #crate_name::internal::DeprecationAttributes {
                            message: std::borrow::Cow::Borrowed(#note),
                            since: #since_tokens,
                        }
                    )
                }
            },
        }
    }
}

impl ToTokens for Deprecation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.as_tokens());
    }
}

pub fn tokenize_deprecation(deprecation: Option<Deprecation>) -> proc_macro2::TokenStream {
    if let Some(inner) = deprecation {
        let inner = inner.as_tokens();

        quote!(Some(#inner))
    } else {
        quote!(None)
    }
}
