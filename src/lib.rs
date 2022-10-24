#![doc(html_root_url = "https://docs.rs/kozo/0.1.0")]

use proc_macro::TokenStream;
mod internals;


#[proc_macro]
pub fn define(content: TokenStream) -> TokenStream {
    match internals::define(content.into()) {
        Err(error) => error.into_compile_error(),
        Ok(result) => result,
    }.into()
}


#[proc_macro]
pub fn retrieve(stream: TokenStream) -> TokenStream {
    match internals::retrieve(stream.into()) {
        Err(error) => error.into_compile_error(),
        Ok(result) => result,
    }.into()
}