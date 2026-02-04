use darling::{Error, FromAttributes, FromMeta, Result, ast::NestedMeta};
use syn::{Lit, LitStr, Meta, Path, Type, TypePath, parse_quote};

#[derive(Debug, FromAttributes)]
#[darling(attributes(error))]
pub struct ContainerAttributes {
    #[darling(default = "ContainerAttributes::default_crate_path")]
    pub crate_path: Path,
    pub display: Option<DisplayAttr>,

    pub module: Option<ContainerModuleAttribute>,
}

#[derive(Debug, FromMeta)]
pub enum ContainerModuleAttribute {
    #[darling(word)]
    Default,
    WithName(String),
}

/*
Option (inside #[snafu(...)])	Description
display("{field:?}: {}", foo)	Sets the display implementation for this error variant using format_args! syntax. If this is omitted, the default is `“VariantName”
context(false)	Skips creation of the context selector, implements From for the mandatory source error
context(suffix(N))	Changes the suffix of the generated context selector to N
context(suffix(false))	No suffix for the generated context selector
transparent	Delegates Display and Error::source to this error’s source, implies context(false)
*/
#[derive(Debug, FromAttributes)]
#[darling(attributes(error))]
pub struct VariantAttributes {
    pub display: Option<DisplayAttr>,
    pub transparent: Option<bool>,
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

        Ok(DisplayAttr { fmt, paths })
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

        Ok(FromAttr { ty, wrapper })
    }
}

impl ContainerAttributes {
    fn default_crate_path() -> Path {
        parse_quote!(::railgun::error)
    }
}
