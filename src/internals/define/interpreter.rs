use proc_macro2::{TokenStream, Ident};
use quote::quote;
use crate::internals::Interpret;
use super::{Define, New, Content, EnumContent};


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
        let mut list = Vec::new();
        interpret_new(self.0, &mut list);
        List(list)
    }
}

fn interpret_new(new: New, list: &mut Vec<Def>) {
    match new {
        New::Struct {
            _struct,
            name,
            _brace,
            fields
        } => {
            let mut struct_fields = Vec::new();
            for field in fields {
                let field_name = field.name;
                let value_type = match field.value {
                    Content::Existing(type_expr) => {
                        quote!(#type_expr)
                    },
                    Content::New(new) => {
                        let type_name = new.name();
                        interpret_new(new, list);
                        quote!(#type_name)
                    },
                };
                struct_fields.push(
                    FieldDef {
                        name: field_name,
                        value_type,
                    }
                )
            }
            list.push(Def {
                name,
                _type:  Type::Struct,
                fields: struct_fields,
            })
        },
        New::Enum {
            _enum,
            name,
            _brace,
            fields
        } => {
            let mut enum_fields = Vec::new();
            for field in fields {
                let variant_name = field.name;
                let variant_content = match field.content {
                    None => TokenStream::new(),
                    Some(content) => match content {
                        EnumContent::Tupple {
                            _paren,
                            types
                        } => {
                            let mut types_stream = TokenStream::new();
                            for type_expr in types {
                                types_stream.extend(quote!(
                                    #type_expr,
                                ))
                            }
                            quote!(
                                ( #types_stream )
                            )
                        },
                        EnumContent::Struct {
                            _brace,
                            fields
                        } => {
                            let mut fields_stream = TokenStream::new();
                            for field in fields {
                                let field_name = field.name;
                                let value_type = match field.value {
                                    Content::Existing(type_expr) => quote!(#type_expr),
                                    Content::New(new) => {
                                        let type_name = new.name();
                                        interpret_new(new, list);
                                        quote!(#type_name)
                                    },
                                };
                                fields_stream.extend(quote!(
                                    #field_name: #value_type,
                                ))
                            }
                            quote!(
                                { #fields_stream }
                            )
                        },
                    }
                };
                enum_fields.push(
                    FieldDef {
                        name: variant_name,
                        value_type: variant_content,
                    }
                )
            }
            list.push(Def {
                name,
                _type:  Type::Enum,
                fields: enum_fields,
            })
        },
    }
}