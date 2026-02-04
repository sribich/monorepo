mod api_error;

use proc_macro::TokenStream;

#[proc_macro_derive(ApiError, attributes(api))]
pub fn derive_api_error(input: TokenStream) -> TokenStream {
    api_error::derive(input)
}
