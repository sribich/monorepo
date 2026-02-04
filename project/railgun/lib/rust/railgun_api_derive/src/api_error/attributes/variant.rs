use darling::{FromAttributes, FromMeta};
use macro_util::ast::Attributes;

#[derive(Debug, FromMeta)]
#[darling(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    UnprocessableEntity,
    InternalServerError,
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(api))]
pub struct VariantAttributes {
    pub status: Status,
    pub code: String,
}

impl Attributes for VariantAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        // Self::from_attributes(attrs).map_err(|err| syn::Error::new(err.span(),
        // err.to_string()))
        Ok(Self::from_attributes(attributes).unwrap())
    }
}

impl VariantAttributes {
    pub fn status_code(&self) -> u16 {
        match self.status {
            Status::UnprocessableEntity => 422,
            Status::InternalServerError => 500,
        }
    }
}
