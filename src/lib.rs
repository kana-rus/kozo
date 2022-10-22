#![doc(html_root_url = "https://docs.rs/kozo/0.1.0")]

use proc_macro::TokenStream;
mod internals;


/// `define!` macro enables to define nested structs in a way easy to see.
/// 
/// ```edition2021
/// define!(struct DeepNestedStruct {
///     a: Vec<u8>,
///     b: struct B {
///         c: String,
///         d: struct D {
///             e: u8,
///             f: u8,
///         },
///     },
///     b2: B,
///     e: struct E {
///         f: &'static str,
///         g: enum G {
///             X,
///             Y,
///             Other {
///                 name: String,
///                 id: usize
///             },
///         },
///     },
/// });
/// ```
/// Then, `define!` is **just a syntax sugar** of defining each named structs separately, so please pay attension to that **all structs declared in define!(); are visible** in its scope.
#[proc_macro]
pub fn define(content: TokenStream) -> TokenStream {
    match internals::define(content.into()) {
        Err(error) => error.into_compile_error(),
        Ok(result) => result,
    }.into()
}


/// `retrieve!` enables to simply get more than 1 value from a struct:
/// 
/// ```edition2021
/// use kozo::{define, retrieve};
/// 
/// define!(struct Sample {
///     a: u8,
///     b: struct B {
///         c: String,
///         d: Vec<u8>,
///     },
/// });
/// 
/// fn main() {
///     let s = Sample {
///         a: 0,
///         b: B {
///             c: "I have an apple?".into(),
///             d: vec![1, 1, 0, 1, 0, 1, 1,],
///         },
///     };
///     retrieve!(a, b from s);
/// 
///     println!("{a}");  // 0,
///     println!("{}", b.c);
/// }
/// ```
#[proc_macro]
pub fn retrieve(stream: TokenStream) -> TokenStream {
    match internals::retrieve(stream.into()) {
        Err(error) => error.into_compile_error(),
        Ok(result) => result,
    }.into()
}