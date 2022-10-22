use proc_macro2::TokenStream;
use syn::{parse2, Result};

trait Interpret<T> {
    fn interpret(self) -> T;
}
trait Build {
    fn build(self) -> TokenStream;
}


mod define;
pub(super) fn define(stream: TokenStream) -> Result<TokenStream> {
    use define::Define;
    Ok(parse2::<Define>(stream)?.interpret().build())
}

mod retrieve;
pub(super) fn retrieve(stream: TokenStream) -> Result<TokenStream> {
    use retrieve::Retrieve;
    Ok(parse2::<Retrieve>(stream)?.build())
}