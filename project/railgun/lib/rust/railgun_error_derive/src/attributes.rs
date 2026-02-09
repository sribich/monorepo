use darling::Error;
use darling::FromAttributes;
use darling::FromMeta;
use darling::Result;
use darling::ast::NestedMeta;
use macro_util::ast::Attributes;
use proc_macro2::Span;
use syn::Ident;
use syn::Lit;
use syn::LitStr;
use syn::Meta;
use syn::Path;
use syn::Type;
use syn::TypePath;

#[derive(Debug, FromAttributes)]
#[darling(attributes(error))]
pub struct ContainerAttributes {
    pub crate_path: Option<Path>,
    pub display: Option<DisplayAttr>,

    pub module: Option<ContainerModuleAttribute>,
    #[darling(default)]
    pub explicit: bool,
}

impl Attributes for ContainerAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        Ok(Self::from_attributes(attributes).unwrap())
    }
}

#[derive(Debug, FromMeta)]
pub enum ContainerModuleAttribute {
    #[darling(word)]
    Default,
    WithName(String),
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(error))]
pub struct VariantAttributes {
    pub display: Option<DisplayAttr>,
    #[expect(dead_code, reason = "Used")]
    pub transparent: Option<bool>,

    #[darling(default)]
    pub explicit: bool,
    pub location: Option<String>,
}

impl Attributes for VariantAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        Ok(Self::from_attributes(attributes).unwrap())
    }
}

impl VariantAttributes {
    pub fn location_field_name(&self) -> &str {
        self.location.as_deref().unwrap_or("location")
    }

    pub fn location_field_ident(&self) -> Ident {
        Ident::new(self.location_field_name(), Span::call_site())
    }
}

#[derive(Debug)]
pub struct DisplayAttr {
    pub fmt: LitStr,
    pub paths: Vec<Path>,
}

impl FromMeta for DisplayAttr {
    fn from_list(items: &[NestedMeta]) -> Result<Self> {
        let fmt = if let Some(attr) = items.first() {
            if let NestedMeta::Lit(Lit::Str(value)) = attr {
                value.clone()
            } else {
                return Err(Error::unexpected_type("non string"));
            }
        } else {
            return Err(Error::unexpected_type("none"));
        };

        let rest_attrs = items.iter().skip(1);

        let mut paths = Vec::with_capacity(rest_attrs.len().saturating_sub(1));

        for nmi in rest_attrs {
            if let NestedMeta::Meta(Meta::Path(path)) = nmi {
                paths.push(path.clone());
            } else {
                return Err(Error::unexpected_type("non-word").with_span(nmi));
            }
        }

        Ok(Self { fmt, paths })
    }
}

/*
source	Marks a field as the source error (even if not called source)
source(from(type, transform))	As above, plus converting from type to the field type by calling transform
source(false)	Marks a field that is named source as a regular field
backtrace	Marks a field as backtrace (even if not called backtrace)
backtrace(false)	Marks a field that is named backtrace as a regular field
implicit	Marks a field as implicit (Type needs to implement GenerateImplicitData)
provide	Marks a field as providing a reference to the type
*/
#[derive(Clone, Debug, FromAttributes)]
#[darling(attributes(error))]
pub struct FieldAttributes {
    pub from: Option<FromAttr>,
    #[darling(default = "Default::default")]
    pub impl_from: bool,
}

impl Attributes for FieldAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        Ok(Self::from_attributes(attributes).unwrap())
    }
}

#[derive(Clone, Debug)]
pub struct FromAttr {
    pub ty: Type,
    pub wrapper: Path,
}

impl FromMeta for FromAttr {
    fn from_list(items: &[NestedMeta]) -> Result<Self> {
        if items.len() < 2 {
            return Err(Error::too_few_items(2));
        }

        if items.len() > 2 {
            return Err(Error::too_many_items(2));
        }

        let ty = if let Some(NestedMeta::Meta(Meta::Path(path))) = items.first() {
            Type::Path(TypePath {
                qself: None,
                path: path.clone(),
            })
        } else {
            return Err(Error::unexpected_type("non type"));
        };

        let wrapper = if let Some(NestedMeta::Meta(Meta::Path(path))) = items.get(1) {
            path.clone()
        } else {
            return Err(Error::unexpected_type("non type"));
        };

        Ok(Self { ty, wrapper })
    }
}
