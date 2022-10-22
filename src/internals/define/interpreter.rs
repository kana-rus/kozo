use proc_macro2::TokenStream;
use crate::internals::Interpret;
use super::*;


pub(crate) struct List(Vec<Def>);
pub(crate) struct Def {
    pub _type:  Type,
    pub name:   Ident,
    pub fields: Vec<FieldDef>,
}
pub(crate) enum Type {
    Struct,
    Enum,
}
pub(crate) struct FieldDef {
    pub name:       Ident,
    pub value_type: TokenStream,
}

impl Iterator for List {
    type Item = Def;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


impl Interpret<List> for Define {
    fn interpret(self) -> List {
        // ======================
        todo!()
        // ======================
    }
}
