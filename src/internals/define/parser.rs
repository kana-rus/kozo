use proc_macro2::Span;
use syn::{parse::Parse, token, braced, Error, parenthesized, Ident};

use super::*;

impl Parse for Define {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}
impl Parse for New {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(token::Struct) {
            let fields_buf;
            Ok(Self::Struct {
                _struct: input.parse().unwrap(),
                name:    input.parse()?,
                _brace:  braced!(fields_buf in input),
                fields:  fields_buf.parse_terminated(StructField::parse)?,
            })
        } else if input.peek(token::Enum) {
            let fields_buf;
            Ok(Self::Enum {
                _enum:  input.parse().unwrap(),
                name:   input.parse()?,
                _brace: braced!(fields_buf in input),
                fields: fields_buf.parse_terminated(EnumField::parse)?
            })
        } else {
            Err(Error::new(
                Span::call_site(),
                "content of `define!` must starts with `struct` or `enum`."
            ))
        }
    }
}

impl Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name:   input.parse()?,
            _colon: input.parse()?,
            value:  input.parse()?,
        })
    }
}

impl Parse for EnumField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name:    input.parse()?,
            content:
                if input.peek(token::Paren)
                || input.peek(token::Brace) {
                    Some(input.parse()?)
                } else {
                    None
                },
        })
    }
}
impl Parse for EnumContent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(token::Paren) {
            let types_buf;
            Ok(Self::Tupple {
                _paren: parenthesized!(types_buf in input),
                types:  types_buf.parse_terminated(Type::parse)?,
            })
        } else if input.peek(token::Brace) {
            let fields_buf;
            Ok(Self::Struct {
                _brace: braced!(fields_buf in input),
                fields: fields_buf.parse_terminated(StructField::parse)?
            })
        } else {
            unreachable!()
        }
    }
}

impl Parse for Content {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(token::Struct) || input.peek(token::Enum) {
            Ok(Self::New(
                input.parse()?
            ))
        } else if input.peek(Ident) {
            Ok(Self::Existing(
                input.parse().unwrap()
            ))
        } else  {
            Err(Error::new(
                Span::call_site(),
"Only
- existing type
- new struct
- new enum
are allowed as type"
            ))
        }
    }
}