use proc_macro2::Span;
use syn::{
    AttrStyle, Expr, Ident, Lit, Meta, MetaList, MetaNameValue, Path, Token,
    ext::IdentExt,
    parse::{ParseBuffer, ParseStream},
    spanned::Spanned,
    token::Paren,
};

#[derive(Clone, Debug)]
pub struct Attribute {
    pub key: Ident,
    pub value: Option<AttributeValue>,
}

#[derive(Clone, Debug)]
pub enum AttributeValue {
    Path(Path),
    Lit(Lit),
    Nested {
        span: Span,
        attributes: Vec<Attribute>,
    },
}

impl Attribute {
    pub fn value_span(&self) -> Span {
        self.value
            .as_ref()
            .map_or_else(|| self.key.span(), AttributeValue::span)
    }

    pub fn parse_bool(&self) -> syn::Result<bool> {
        if let Some(AttributeValue::Lit(Lit::Bool(r#bool))) = &self.value {
            Ok(r#bool.value())
        } else {
            Err(syn::Error::new(
                self.value_span(),
                "Expected a bool literal.",
            ))
        }
    }

    pub fn parse_path(&self) -> syn::Result<Path> {
        if let Some(AttributeValue::Path(path)) = &self.value {
            Ok(path.clone())
        } else {
            Err(syn::Error::new(self.value_span(), "Expected a path."))
        }
    }

    pub fn parse_string(&self) -> syn::Result<String> {
        if let Some(AttributeValue::Lit(Lit::Str(string))) = &self.value {
            Ok(string.value())
        } else {
            Err(syn::Error::new(
                self.value_span(),
                "Expected a string literal.",
            ))
        }
    }

    /*
    pub fn parse_case(&self) -> Result<Case> {
        if let Some(AttributeValue::Lit(Lit::Str(string))) = &self.value {
            Ok(
                match string
                    .value()
                    .to_lowercase()
                    .replace(['-', '_'], "")
                    .as_str()
                {
                    "lowercase" => Case::Lower,
                    "uppercase" => Case::Upper,
                    "pascalcase" => Case::Pascal,
                    "camelcase" => Case::Camel,
                    "snakecase" => Case::Snake,
                    "screamingsnakecase" => Case::ScreamingSnake,
                    "kebabcase" => Case::Kebab,
                    "screamingkebabcase" => Case::UpperKebab,
                    _ => return Err(syn::Error::new_spanned(string, "Unknown case identifier")),
                },
            )
        } else {
            Err(syn::Error::new(
                self.value_span(),
                "Expected a string literal containing a case identifier",
            ))
        }
    }
    */
}

impl AttributeValue {
    pub fn span(&self) -> Span {
        match &self {
            Self::Path(path) => path.span(),
            Self::Lit(lit) => lit.span(),
            Self::Nested { span, .. } => *span,
        }
    }
}

impl TryFrom<&Expr> for AttributeValue {
    type Error = syn::Error;

    fn try_from(value: &Expr) -> Result<Self, Self::Error> {
        Ok(match value {
            Expr::Lit(lit) => Self::Lit(lit.lit.clone()),
            Expr::Path(path) => Self::Path(path.path.clone()),
            Expr::Array(_)
            | Expr::Assign(_)
            | Expr::Async(_)
            | Expr::Await(_)
            | Expr::Binary(_)
            | Expr::Block(_)
            | Expr::Break(_)
            | Expr::Call(_)
            | Expr::Cast(_)
            | Expr::Closure(_)
            | Expr::Const(_)
            | Expr::Continue(_)
            | Expr::Field(_)
            | Expr::ForLoop(_)
            | Expr::Group(_)
            | Expr::If(_)
            | Expr::Index(_)
            | Expr::Infer(_)
            | Expr::Let(_)
            | Expr::Loop(_)
            | Expr::Macro(_)
            | Expr::Match(_)
            | Expr::MethodCall(_)
            | Expr::Paren(_)
            | Expr::RawAddr(_)
            | Expr::Range(_)
            | Expr::Reference(_)
            | Expr::Repeat(_)
            | Expr::Return(_)
            | Expr::Struct(_)
            | Expr::Try(_)
            | Expr::TryBlock(_)
            | Expr::Tuple(_)
            | Expr::Unary(_)
            | Expr::Unsafe(_)
            | Expr::Verbatim(_)
            | Expr::While(_)
            | Expr::Yield(_)
            | _ => {
                return Err(syn::Error::new(
                    value.span(),
                    "Unable to convert value into an AttributeValue",
                ));
            },
        })
    }
}

impl TryFrom<&ParseBuffer<'_>> for AttributeValue {
    type Error = syn::Error;

    // TODO: This is fragile
    fn try_from(value: &ParseBuffer<'_>) -> Result<Self, Self::Error> {
        if value.peek(Lit) {
            Ok(Self::Lit(value.parse()?))
        } else {
            Ok(Self::Path(value.parse()?))
        }
    }
}

/// Parses a slice of `[syn::Attribute]` into our own `[Attribute]` form
/// which treats any attribute kind as a key=value pair.
pub fn parse_attributes(attributes: &[syn::Attribute]) -> syn::Result<Vec<Attribute>> {
    let mut result = Vec::with_capacity(attributes.len());

    for attribute in attributes {
        if let AttrStyle::Inner(_) = attribute.style {
            return Err(syn::Error::new_spanned(
                attribute,
                "Invalid position for inner attribute",
            ));
        }

        let ident = attribute
            .path()
            .segments
            .last()
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    attribute,
                    "Expected attribute to have at least 1 path segment",
                )
            })?
            .clone()
            .ident;

        let value = match &attribute.meta {
            Meta::Path(_) => Attribute {
                key: ident,
                value: None,
            },
            Meta::List(MetaList { tokens, .. }) => Attribute {
                key: ident.clone(),
                value: Some(AttributeValue::Nested {
                    span: ident.span(),
                    attributes: syn::parse::Parser::parse2(parse_inner_attribute, tokens.clone())?,
                }),
            },
            Meta::NameValue(MetaNameValue { value, .. }) => Attribute {
                key: ident,
                value: Some(AttributeValue::try_from(value)?),
            },
        };

        result.push(value);
    }

    Ok(result)
}

pub fn parse_attribute_args(args: proc_macro2::TokenStream) -> syn::Result<Vec<Attribute>> {
    // let args =
    //     syn::parse::Parser::parse2(Punctuated::<syn::Meta, Token![,]>::parse_terminated, args)?;

    syn::parse::Parser::parse2(parse_inner_attribute, args)
}

/// Parses the inner value of a [`MetaList`] style attribute.
///
/// [`MetaList`]: syn::MetaList
fn parse_inner_attribute(content: ParseStream) -> syn::Result<Vec<Attribute>> {
    let mut result = Vec::new();

    while !content.is_empty() {
        let ident = content.call(Ident::parse_any)?;

        result.push(Attribute {
            key: ident.clone(),
            value: {
                if content.peek(Paren) {
                    let inner_content;
                    syn::parenthesized!(inner_content in content);

                    Some(AttributeValue::Nested {
                        span: ident.span(),
                        attributes: parse_inner_attribute(&inner_content)?,
                    })
                } else if content.peek(Token![=]) {
                    content.parse::<Token![=]>()?;
                    Some(AttributeValue::try_from(content)?)
                } else {
                    None
                }
            },
        });

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }
    }

    Ok(result)
}

/*
macro_rules! attribute_parser {
    ($parser:ident ($attribute:ident, $out:ident) {
        $($match_arm:pat => $match_expr:expr),*
        $(,)?
    }) => {
        impl $parser {
            fn try_from_attrs(
                ident: &'static str,
                attrs: &mut [Attribute],
                $out: &mut Self,
            ) -> Result<()> {
                for attr in attrs.iter_mut().filter(|attr| attr.key == ident) {
                    match &mut attr.value {
                        Some(crate::attributes::AttributeValue::Lit(lit)) => {
                            let $attribute = attr;

                            match $attribute.key.to_string().as_str() {
                                $($match_arm => $match_expr,)*
                                _ => {},
                            };
                        },
                        Some(crate::attributes::AttributeValue::Nested { attributes, .. }) => {
                            *attributes = std::mem::take(attributes)
                                .into_iter()
                                .map(|$attribute| {
                                    let mut was_passed_by_user = true;

                                    match $attribute.key.to_string().as_str() {
                                        $($match_arm => $match_expr,)*
                                        #[allow(unreachable_patterns)]
                                        _ => {
                                            was_passed_by_user = false;
                                        }
                                    }

                                    Ok(($attribute, was_passed_by_user))
                                })
                            .collect::<syn::Result<Vec<(crate::attributes::Attribute, bool)>>>()?
                                .into_iter()
                                .filter_map(
                                    |(attr, was_passed_by_user)| {
                                        if was_passed_by_user {
                                            None
                                        } else {
                                            Some(attr)
                                        }
                                    },
                                )
                                .collect();
                        }
                        _ => {}
                    }
                }

                Ok(())
            }

            fn try_from_attrs_pass_nested(
                ident: &'static str,
                attrs: &mut [Attribute],
                $out: &mut Self,
            ) -> Result<()> {
                for attr in attrs.iter_mut().filter(|attr| attr.key == ident) {
                    match &mut attr.value {
                        Some(crate::attributes::AttributeValue::Lit(_)) => {
                            let $attribute = attr;

                            match $attribute.key.to_string().as_str() {
                                $($match_arm => $match_expr,)*
                                _ => {},
                            };
                        },
                        Some(crate::attributes::AttributeValue::Nested { .. }) => {
                            let $attribute = attr;

                            match $attribute.key.to_string().as_str() {
                                $($match_arm => $match_expr,)*
                                _ => {},
                            };
                        }
                        _ => {}
                    }
                }

                Ok(())
            }
        }
    };
}
pub(crate) use attribute_parser;

*/
