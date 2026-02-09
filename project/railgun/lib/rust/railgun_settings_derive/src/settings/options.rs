use darling::FromMeta;
use darling::ast::NestedMeta;
use syn::Path;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::punctuated::Punctuated;

fn parse_meta_list(input: ParseStream) -> syn::Result<Vec<NestedMeta>> {
    let list = Punctuated::<NestedMeta, Token![,]>::parse_terminated(input)?
        .into_iter()
        .collect();

    Ok(list)
}

#[derive(FromMeta)]
pub struct Options {
    #[darling(default = "Options::default_impl_default")]
    pub impl_default: bool,
    #[darling(default = "Options::default_impl_debug")]
    pub impl_debug: bool,
    pub crate_path: Option<Path>,
}

impl Options {
    pub fn default_impl_default() -> bool {
        true
    }

    pub fn default_impl_debug() -> bool {
        cfg!(debug_assertions)
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            impl_default: Self::default_impl_default(),
            impl_debug: Self::default_impl_debug(),
            crate_path: None,
        }
    }
}

impl Parse for Options {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let options = if input.is_empty() {
            Self::default()
        } else {
            let meta_list = parse_meta_list(input)?;
            Self::from_list(&meta_list)?
        };

        Ok(options)
    }
}
