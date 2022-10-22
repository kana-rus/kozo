use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, parse::Parse};
use super::Build;

mod keyword {
    syn::custom_keyword!(from);
}

pub(super) struct Retrieve {
    idents: Punctuated<Ident, Comma>,
    from:   keyword::from,
    target: Ident,
}

impl Parse for Retrieve {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            idents: input.parse_terminated(Ident::parse)?,
            from:   input.parse()?,
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
                let #ident = #target.#ident
            ))
        }
        result
    }
}