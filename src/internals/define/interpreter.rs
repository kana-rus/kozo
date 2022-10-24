use proc_macro2::{TokenStream, Ident};
use quote::quote;
use crate::internals::Interpret;
use super::{Define, New, Content, EnumContent};


pub(crate) struct List(Vec<Def>);
#[derive(Clone)]
pub(crate) struct Def {
    pub content_type: ContentType,
    pub name:         Ident,
    pub fields:       Vec<FieldDef>,
}
#[derive(Clone)]
pub(crate) enum ContentType {
    Struct,
    Enum,
}
#[derive(Clone)]
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
                content_type: ContentType::Struct,
                fields:       struct_fields,
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
                content_type: ContentType::Enum,
                fields:       enum_fields,
            })
        },
    }
}


#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use quote::{quote, format_ident};
    use syn::parse2;
    use crate::internals::{define::Define, Interpret};
    use super::{List, Def, FieldDef, ContentType};

    #[test]
    fn interpret_nested_1() {
        let case = parse2::<Define>(quote!(
            struct NestedStruct {
                a: struct A {
                    b: u8,
                    c: u8,
                },
            }
        )).unwrap(/* this parsing passed in mod.rs::test */);
        assert_eq!(
            case.interpret(),
            List(vec![
                Def {
                    content_type: ContentType::Struct,
                    name: format_ident!("NestedStruct"),
                    fields: vec![
                        FieldDef {
                            name: format_ident!("a"),
                            value_type: quote!(A),
                        },
                    ],
                },
                Def {
                    content_type: ContentType::Struct,
                    name: format_ident!("A"),
                    fields: vec![
                        FieldDef {
                            name: format_ident!("b"),
                            value_type: quote!(u8),
                        },
                        FieldDef {
                            name: format_ident!("c"),
                            value_type: quote!(u8),
                        },
                    ],
                }
            ])
        )
    }
    #[test]
    fn interpret_nested_2() {
        let case = parse2::<Define>(quote!(
            struct NestedStruct {
                a: struct A {
                    b: u8,
                    c: u8,
                },
                d: struct D {
                    e: String,
                    f: Vec<u8>,
                },
            }
        )).unwrap(/* this parsing passed in mod.rs::test */);
        assert_eq!(
            case.interpret(),
            List(vec![
                Def {
                    content_type: ContentType::Struct,
                    name: format_ident!("NestedStruct"),
                    fields: vec![
                        FieldDef {
                            name: format_ident!("a"),
                            value_type: quote!(A),
                        },
                        FieldDef {
                            name: format_ident!("d"),
                            value_type: quote!(D),
                        }
                    ],
                },
                Def {
                    content_type: ContentType::Struct,
                    name: format_ident!("A"),
                    fields: vec![
                        FieldDef {
                            name: format_ident!("b"),
                            value_type: quote!(u8),
                        },
                        FieldDef {
                            name: format_ident!("c"),
                            value_type: quote!(u8),
                        },
                    ],
                },
                Def {
                    content_type: ContentType::Struct,
                    name: format_ident!("D"),
                    fields: vec![
                        FieldDef {
                            name: format_ident!("e"),
                            value_type: quote!(String),
                        },
                        FieldDef {
                            name: format_ident!("f"),
                            value_type: quote!(Vec<u8>),
                        },
                    ],
                }
            ])
        )
    }




    impl PartialEq for List {
        fn eq(&self, other: &Self) -> bool {
            eq_as_set(&self.0, &other.0)
        }
    }
    impl PartialEq for Def {
        fn eq(&self, other: &Self) -> bool {
            self.content_type == other.content_type &&
            self.name == other.name &&
            eq_as_set(&self.fields, &other.fields)
        }
    }
    impl PartialEq for ContentType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                ContentType::Enum => match other {
                    ContentType::Enum   => true,
                    ContentType::Struct => false,
                },
                ContentType::Struct => match other {
                    ContentType::Enum   => false,
                    ContentType::Struct => true,
                }
            }
        }
    }
    impl PartialEq for FieldDef {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name &&
            self.value_type.to_string() == other.value_type.to_string()
        }
    }

    impl Debug for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "List{:?}", self.0)
        }
    }
    impl Debug for Def {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self.content_type {
                ContentType::Enum => {
                    let mut fmt = format!("enum {}{{", self.name);
                    for variant in &self.fields {
                        fmt += &format!("{}{},", variant.name, variant.value_type)
                    }
                    fmt + "}"
                },
                ContentType::Struct => {
                    let mut fmt = format!("struct {}{{", self.name);
                    for field in &self.fields {
                        fmt += &format!("{}:{},", field.name, field.value_type)
                    }
                    fmt + "}"
                },
            })
        }
    }

    fn eq_as_set<T: PartialEq + Clone>(x: &Vec<T>, y: &Vec<T>) -> bool {
        let mut count = x.len();
        if count != y.len() {return false}

        let mut xindex = (0..count).collect::<Vec<_>>();
        for ty in y {
            let mut found = false;
            for i in 0..count {
                if &x[xindex[i]] == ty {
                    found = true;
                    xindex.remove(i);
                    count -= 1;
                    if count == 0 {return true}
                    break
                }
            }
            if !found {return false}
        }
        false
    }
}