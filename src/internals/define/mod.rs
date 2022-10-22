use proc_macro2::Ident;
use syn::{punctuated::Punctuated, token::{Comma, Colon}};

pub(super) struct Define {
    fields: Punctuated<Field, Comma>,
}
pub struct Field {
    name:   Ident,
    _colon: Colon,
    
}