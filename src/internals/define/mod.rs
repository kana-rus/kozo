use proc_macro2::Ident;
use syn::{punctuated::Punctuated, token::{Comma, Colon, Brace, Paren, Struct, Enum}, Type};

mod parser;
mod interpreter;
mod builder;


pub struct Define(New);
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
    Existing(Ident),
    New(New),
}