use darling::FromAttributes;
use darling::FromMeta;
use macro_util::ast::Attributes;

#[derive(Debug, FromMeta)]
#[darling(rename_all = "snake_case")]
#[derive(Default)]
pub enum Format {
    #[default]
    Json,
}


#[derive(Debug, FromAttributes)]
#[darling(attributes(api))]
pub struct ContainerAttributes {
    #[darling(default)]
    pub format: Format,
}

impl Attributes for ContainerAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        // Self::from_attributes(attrs).map_err(|err| syn::Error::new(err.span(),
        // err.to_string()))
        Ok(Self::from_attributes(attributes).unwrap())
    }
}
