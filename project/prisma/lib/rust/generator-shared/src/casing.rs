use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

pub fn cased_ident(item: &str, case: Case) -> Ident {
    format_ident!("{}", item.to_case(case))
}
