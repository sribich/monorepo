use r#enum::impl_enum;
use macro_util::ast::Input;
use options::Options;
use proc_macro::TokenStream;
use proc_macro_error2::abort;
use r#struct::impl_struct;
use syn::Item;
use syn::parse_macro_input;

mod r#enum;
mod options;
mod r#struct;

pub fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(args as Options);
    let item = parse_macro_input!(item as Item);

    inner_expand(options, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

pub fn inner_expand(options: Options, item: Item) -> syn::Result<proc_macro2::TokenStream> {
    let abort_item = item.clone();

    Ok(match item {
        Item::Struct(input) => impl_struct(options, input)?,
        Item::Enum(input) => {
            let input = input.into();
            let input = Input::from_syn(&input)?;

            if let Input::Enum(input) = input {
                impl_enum(options, input)?
            } else {
                abort!(abort_item, "Assert error")
            }
        }
        Item::Const(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::ExternCrate(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Fn(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::ForeignMod(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Impl(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Macro(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Mod(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Static(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Trait(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::TraitAlias(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Type(item) => abort!(item, "Settings must be either ann enum or a struct"),
        Item::Union(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Use(item) => abort!(item, "Settings must be either an enum or a struct"),
        Item::Verbatim(item) => abort!(item, "Settings must be either an enum or a struct"),
        item => abort!(item, "Settings must be either an enum or a struct"),
    })
}
