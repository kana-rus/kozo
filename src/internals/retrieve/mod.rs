use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{token::Comma, parse::Parse};
use super::Build;

mod keyword {
    syn::custom_keyword!(from);
}

pub(super) struct Retrieve {
    idents: Vec<Ident>,
    _from:  keyword::from,
    target: Ident,
}

impl Parse for Retrieve {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            idents: {
                let mut idents = vec![
                    input.parse::<Ident>()?
                ];
                while !input.peek(keyword::from) {
                    input.parse::<Comma>()?;
                    idents.push(input.parse::<Ident>()?)
                }
                idents
            },
            _from:  input.parse()?,
            target: input.parse()?,
        })
    }
}

impl Build for Retrieve {
    fn build(self) -> TokenStream {
        let mut result = TokenStream::new();
        let target = self.target;
        for ident in self.idents {
            result.extend(quote!(
                let #ident = #target.#ident;
            ))
        }
        result
    }
}