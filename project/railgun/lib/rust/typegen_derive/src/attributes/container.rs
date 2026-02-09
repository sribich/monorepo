use convert_case::Case;
use darling::FromAttributes;
use darling::FromMeta;
use macro_util::ast::Attributes;
use proc_macro2::Span;

use super::common::Deprecation;
use super::common::DocAttribute;

#[derive(Debug, FromMeta)]
pub enum SerdeCase {
    #[darling(rename = "lowercase")]
    Lowercase,
    #[darling(rename = "UPPERCASE")]
    Uppercase,
    #[darling(rename = "PascalCase")]
    PascalCase,
    #[darling(rename = "camelCase")]
    CamelCase,
    #[darling(rename = "snake_case")]
    SnakeCase,
    #[darling(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnakeCase,
    #[darling(rename = "kebab-case")]
    KebabCase,
    #[darling(rename = "SCREAMING-KEBAB-CASE")]
    ScreamingKebabCase,
}

impl From<&SerdeCase> for convert_case::Case<'_> {
    fn from(value: &SerdeCase) -> Self {
        match value {
            SerdeCase::Lowercase => Case::Lower,
            SerdeCase::Uppercase => Case::Upper,
            SerdeCase::PascalCase => Case::Pascal,
            SerdeCase::CamelCase => Case::Camel,
            SerdeCase::SnakeCase => Case::Snake,
            SerdeCase::ScreamingSnakeCase => Case::UpperSnake,
            SerdeCase::KebabCase => Case::Kebab,
            SerdeCase::ScreamingKebabCase => Case::UpperKebab,
        }
    }
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(serde, typegen), forward_attrs(doc), allow_unknown_fields)]
pub struct ContainerAttributes {
    /// Exports the container type with the given name instead of
    /// the default Rust name.
    ///
    /// https://serde.rs/container-attrs.html#rename
    pub rename: Option<String>,
    /// Rename all the fields (if this is a struct) or variants (if this is an
    /// enum) according to the given case convention. The possible values are
    /// "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case",
    /// "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE".
    ///
    /// https://serde.rs/container-attrs.html#rename_all
    pub rename_all: Option<SerdeCase>,
    /// #[serde(rename_all_fields = "...")]
    ///
    /// Apply a rename_all on every struct variant of an enum according to the
    /// given case convention. The possible values are "lowercase", "UPPERCASE",
    /// "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE",
    /// "kebab-case", "SCREAMING-KEBAB-CASE".
    ///
    /// https://serde.rs/container-attrs.html#rename_all_fields
    pub rename_all_fields: Option<SerdeCase>,
    /// On an enum: Use the internally tagged enum representation, with the
    /// given tag. See enum representations for details on this representation.
    ///
    /// On a struct with named fields: Serialize the struct's name (or value of
    /// serde(rename)) as a field with the given key, in front of all the real
    /// fields of the struct.
    ///
    /// https://serde.rs/container-attrs.html#tag
    pub tag: Option<String>,
    /// Use the adjacently tagged enum representation for this enum, with the
    /// given field names for the tag and content. See enum representations for
    /// details on this representation.
    ///
    /// https://serde.rs/container-attrs.html#tag--content
    pub content: Option<String>,
    /// Use the untagged enum representation for this enum. See enum
    /// representations for details on this representation.
    ///
    /// https://serde.rs/container-attrs.html#untagged
    pub untagged: Option<bool>,
    /// Serialize and deserialize a newtype struct or a braced struct
    /// with one field exactly the same as if its one field were serialized
    /// and deserialized by itself. Analogous to `#[repr(transparent)]`.
    ///
    /// https://serde.rs/container-attrs.html#transparent
    #[darling(default)]
    pub transparent: bool,
    /// Additional attributes forwarded by darling.
    pub attrs: Vec<syn::Attribute>,
}

impl Attributes for ContainerAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        // Self::from_attributes(attrs).map_err(|err| syn::Error::new(err.span(),
        // err.to_string()))
        Ok(Self::from_attributes(attributes).unwrap())
    }
}

impl ContainerAttributes {
    pub fn docs(&self) -> syn::Result<String> {
        DocAttribute::from_attributes(&self.attrs).map(|it| it.0)
    }

    pub fn deprecation(&self) -> syn::Result<Option<Deprecation>> {
        Deprecation::from_attributes(&self.attrs)
    }

    pub fn tagged(&self) -> syn::Result<Tagged> {
        let span = Span::call_site();

        match (&self.tag, &self.content, self.untagged) {
            (None, None, None) => Ok(Tagged::Externally),
            (None, None, Some(_)) => Ok(Tagged::Untagged),
            (None, Some(_), None) => {
                Err(syn::Error::new(span, "Content cannot be used without tag."))
            }
            (None, Some(_), Some(_)) => Err(syn::Error::new(
                span,
                "Untagged cannot be used with content.",
            )),
            (Some(tag), None, None) => Ok(Tagged::Internally { tag: tag.clone() }),
            (Some(tag), Some(content), None) => Ok(Tagged::Adjacently {
                tag: tag.clone(),
                content: content.clone(),
            }),
            (Some(_), _, Some(_)) => {
                Err(syn::Error::new(span, "Untagged cannot be used with tag."))
            }
        }
    }
}

pub enum Tagged {
    Externally,
    Adjacently { tag: String, content: String },
    Internally { tag: String },
    Untagged,
}

/*
#[derive(Clone, Default)]
pub struct ContainerAttributes {
    pub common: CommonAttributes,
    /// Overcomes the rust orphan rule by delegating type serialization
    /// to an owned representation of an external type.
    ///
    /// https://serde.rs/remote-derive.html
    pub remote: Option<TokenStream>,

    /// Renames all fields or variants with a given case convention.
    /// Useful when serializing fields to other languages that have
    /// a different "idiomatic" format, like camelCase in javascript.
    ///
    /// https://serde.rs/container-attrs.html#rename_all
    pub rename_all: Option<Case>,
}

impl ContainerAttributes {
    pub fn from_attributes(attributes: &mut Vec<Attribute>) -> Result<Self> {
        let mut result = Self {
            common: CommonAttributes::from_attributes(attributes)?,
            ..Self::default()
        };

        Self::try_from_attrs("doc", attributes, &mut result)?;
        Self::try_from_attrs("serde", attributes, &mut result)?;
        Self::try_from_attrs("typegen", attributes, &mut result)?;

        if let Some(attribute) = attributes.iter().find(|attr| attr.key == "typegen") {
            match &attribute.value {
                Some(AttributeValue::Nested { attributes, .. }) => {
                    if let Some(inner_attribute) = attributes.first() {
                        return Err(syn::Error::new(
                            inner_attribute.key.span(),
                            format!(
                                "Found unsupported container attribute '{}'",
                                inner_attribute.key
                            ),
                        ));
                    }
                },
                _ => {
                    return Err(syn::Error::new(
                        attribute.key.span(),
                        "Invalid typegen attribute format",
                    ));
                },
            }
        }

        Ok(result)
    }
}

impl TryFrom<Vec<Attribute>> for ContainerAttributes {
    type Error = syn::Error;

    fn try_from(mut value: Vec<Attribute>) -> std::prelude::v1::Result<Self, Self::Error> {
        ContainerAttributes::from_attributes(&mut value)
    }
}

attribute_parser! {
    ContainerAttributes(attr, out) {
        "remote" => {
            if out.remote.is_some() {
                return Err(syn::Error::new(attr.key.span(), "Duplicate attribute detected."))
            }

            out.remote = Some(attr.parse_path()?.to_token_stream());
        },
        // TODO: Write test cases
        //        a. Conflicting values (both with serde only, typegen only, and typegen and serde attributes)
        //        b. Same values
        //        c. Make sure when using same values that the value does get overwritten properly.
        "rename" => {
            let value = attr.parse_string()?.to_token_stream();

            if let Some(existing) = &out.rename && existing.to_string() != value.to_string() {
                todo!("Rename has conflicting values.")
            } else {
                out.rename = Some(value);
            }
        },
        "rename_all" => {
            let value = attr.parse_case()?;

            if let Some(existing) = &out.rename_all && *existing == value {
                return Err(syn::Error::new(attr.key.span(), "Duplicate attribute detected"));
            } else {
                out.rename_all = Some(value);
            }
        },

    }
}

/*
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Result, Type};

use super::{attribute_parser, common::CommonAttributes, Attribute};

#[derive(Clone, Default)]
pub struct FieldAttributes {
    pub rename: Option<TokenStream>,
    pub retype: Option<Type>,
    pub skip: bool,
    pub optional: bool,
    pub common: CommonAttributes,
}

impl TryFrom<&mut Vec<Attribute>> for FieldAttributes {
    type Error = syn::Error;

    fn try_from(value: &mut Vec<Attribute>) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut result = Self::default();

        result.common = CommonAttributes::from_attributes(value);

        Self::try_from_attrs("doc", value, &mut result)?;
        Self::try_from_attrs("typegen", value, &mut result)?;
        Self::try_from_attrs("serde", value, &mut result)?;

        Ok(result)
    }
}

attribute_parser! {
    FieldAttributes(attr, out) {
        // TODO: Write test cases
        //        a. Conflicting values (both with serde only, typegen only, and typegen and serde attributes)
        //        b. Same values
        //        c. Make sure when using same values that the value does get overwritten properly.
        "rename" => {
            let value = attr.parse_string()?.to_token_stream();

            if let Some(existing) = &out.rename && existing.to_string() != value.to_string() {
                todo!("Rename has conflicting values.")
            } else {
                out.rename = Some(value)
            }
        },

        "optional" => out.optional = attr.parse_bool().unwrap_or(true),
    }
}

pub(crate) fn parse_field_attributes(field: &Field) -> syn::Result<FieldAttributes> {
    let mut attributes = parse_attributes(&field.attrs)?;
    let field_attributes: FieldAttributes = (&mut attributes).try_into()?;

    println!("here?");

    let mut unused_attributes = attributes
        .iter()
        .filter_map(|attribute| {
            if attribute.key != "typegen" {
                return None;
            }

            match &attribute.value {
                AttributeValue::Nested(inner_attributes) => {
                    if let Some(attr) = inner_attributes.first() {
                        Some(syn::Error::new(
                            attr.key.span(),
                            format!("Unsupported typegen attribute field '{}'", attr.key),
                        ))
                    } else {
                        None
                    }
                }
                _ => Some(syn::Error::new(
                    attribute.key.span(),
                    "Invalid typegen attribute format",
                )),
            }
        })
        .collect::<Vec<_>>();

    if let Some(error) = unused_attributes.pop() {
        return Err(error);
    }

    Ok(field_attributes)
}

#[cfg(test)]
mod test {
    use quote::quote;
    use syn::Attribute;

    #[test]
    fn test() {
        let result = quote!(#[foo(bar = 1, baz = "wee")]);
        let result: Attribute = syn::parse_quote!(#result);
        println!("{:#?}", result);

        assert!(1 == 2);
    }
}

///
/// ```compile_fail ergerg
/// #[derive(typegen_derive::Typegen)]
/// struct Foo {
///   #[typegen = "foobar"]
///   field: String,
/// }
/// ```

*/
*/
