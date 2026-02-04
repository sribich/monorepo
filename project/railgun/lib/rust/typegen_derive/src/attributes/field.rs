use darling::FromAttributes;
use macro_util::{ast::Attributes, attribute::parse_attributes};
use proc_macro2::Span;

use super::common::{Deprecation, DocAttribute};

#[derive(Clone, Default, FromAttributes)]
#[darling(
    attributes(serde, typegen),
    forward_attrs(deprecated, doc),
    allow_unknown_fields
)]
pub struct FieldAttributes {
    /// Flatten the contents of this field into the container it is defined in.
    ///
    /// This removes one level of structure between the serialized
    /// representation and the Rust data structure representation. It can be
    /// used for factoring common keys into a shared structure, or for capturing
    /// remaining fields into a map with arbitrary string keys. The struct
    /// flattening page provides some examples.
    ///
    /// Note: this attribute is not supported in combination with structs that
    /// use deny_unknown_fields. Neither the outer nor inner flattened struct
    /// should use that attribute.
    ///
    /// https://serde.rs/field-attrs.html#flatten
    #[darling(default)]
    pub flatten: bool,
    /// Serialize and deserialize this field with the given name instead of
    /// its Rust name. This is useful for serializing fields as camelCase or
    /// serializing fields with names that are reserved Rust keywords.
    ///
    /// https://serde.rs/field-attrs.html#rename
    pub rename: Option<String>,
    /// Skip this field: do not serialize or deserialize it.
    ///
    /// When deserializing, Serde will use Default::default() or the function
    /// given by default = "..." to get a default value for this field.
    ///
    /// https://serde.rs/field-attrs.html#skip
    #[darling(default)]
    pub skip: bool,
    /// Additional deprecated & doc attributes
    pub attrs: Vec<syn::Attribute>,
}

impl Attributes for FieldAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        Self::check_for_invalid_skip_attributes(attributes)?;

        // Self::from_attributes(attrs).map_err(|err| syn::Error::new(err.span(),
        // err.to_string()))
        Ok(Self::from_attributes(attributes).unwrap())
    }
}

impl FieldAttributes {
    pub fn docs(&self) -> syn::Result<String> {
        DocAttribute::from_attributes(&self.attrs).map(|it| it.0)
    }

    pub fn deprecation(&self) -> syn::Result<Option<Deprecation>> {
        Deprecation::from_attributes(&self.attrs)
    }

    fn check_for_invalid_skip_attributes(attrs: &[syn::Attribute]) -> syn::Result<()> {
        let invalid_attributes = [
            "skip_serializing",
            "skip_deserializing",
            "skip_serializing_if",
        ];

        if parse_attributes(attrs)?
            .iter()
            .map(|it| &it.key)
            .any(|it| invalid_attributes.contains(&it.to_string().as_str()))
        {
            return Err(syn::Error::new(
                Span::call_site(),
                "Conditional serializers are not supported when using typegen as type safety cannot be guaranteed.",
            ));
        }

        Ok(())
    }
}
