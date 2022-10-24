#![doc(html_root_url = "https://docs.rs/kozo/0.1.0")]

use proc_macro::TokenStream;
mod internals;


/// `define!` macro enables to define nested structs in a way easy to see.
/// 
/// ```edition2021
/// use kozo::define;
/// 
/// define!(struct NestedStruct {
///     a: Vec<u8>,
///     b: struct B {
///         c: struct C {
///             d: u8,
///             e: u8,
///         },
///         f: enum F {
///             X,
///             Y,
///             Other {
///                 name: String,
///                 id: usize
///             },
///         },
///     },
/// });
/// 
/// fn main() {
///     let sample = NestedStruct {
///         a: vec![1, 1, 0, 1, 0, 1, 1],
///         b: B {
///             c: C {
///                 d: 0,
///                 e: 1,
///             },
///             f: F::X,
///         },
///     };
/// 
///     println!("{}", sample.b.c.d);  // 0
/// }
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
///             d: vec![1, 1, 0, 1, 0, 1, 1],
///         },
///     };
///     retrieve!(a, b from s);
/// 
///     println!("{a}");  // 0
///     println!("{:?}", b.d);  // [1, 1, 0, 1, 0, 1, 1]
/// }
/// ```
#[proc_macro]
pub fn retrieve(stream: TokenStream) -> TokenStream {
    match internals::retrieve(stream.into()) {
        Err(error) => error.into_compile_error(),
        Ok(result) => result,
    }.into()
}