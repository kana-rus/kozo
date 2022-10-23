use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, token::{Comma, Colon, Brace, Paren, Struct, Enum}, Type};

mod parser;
mod interpreter;
mod builder;


pub struct Define(
    New
);
pub(super) enum New {
    Struct {
        _struct: Struct,
        name:    Ident,
        _brace:  Brace,
        fields:  Punctuated<StructField, Comma>,
    },
    Enum {
        _enum:  Enum,
        name:   Ident,
        _brace: Brace,
        fields: Punctuated<EnumField, Comma>,
    },
} impl New {
    pub fn name(&self) -> Ident {
        match self {
            New::Struct {
                _struct,
                name,
                _brace,
                #[allow(unused)]  // match で fields を取り出すので直接アクセスしない
                fields
            } => name,
            New::Enum {
                _enum,
                name,
                _brace,
                #[allow(unused)]  // match で fields を取り出すので直接アクセスしない
                fields
            } => name,
        }.clone()
    }

}

pub(super) struct StructField {
    name:   Ident,
    _colon: Colon,
    value:  Content,
}

pub(super) struct EnumField {
    name:    Ident,
    content: Option<EnumContent>,
}
pub(super) enum EnumContent {
    Tupple {
        _paren: Paren,
        types:  Punctuated<Type, Comma>,
    },
    Struct {
        _brace: Brace,
        fields: Punctuated<StructField, Comma>,
    },
}

pub(super) enum Content {
    Existing(TokenStream),
    New(New),
}