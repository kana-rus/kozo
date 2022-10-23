use proc_macro2::Ident;
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
    Existing(Type),
    New(New),
}


#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use proc_macro2::Span;
    use quote::{quote, format_ident};
    use syn::{punctuated::Punctuated, token, Type, parse2};
    use super::{Define, New, StructField, EnumField, EnumContent, Content};

    #[test]
    fn parse_non_nested_1() {
        let case = quote!(
            struct NestedStruct {
                a: Vec<u8>,
                b: u8,
            }
        );
        assert_eq!(
            match parse2::<Define>(case) {
                Err(error) => panic!("{}", error.to_string()),
                Ok(define) => define
            },
            Define(New::Struct {
                _struct: token::Struct(Span::call_site()),
                name:    format_ident!("NestedStruct"),
                _brace:  token::Brace(Span::call_site()),
                fields:  Punctuated::<StructField, token::Comma>::from_iter([
                    StructField {
                        name:   format_ident!("a"),
                        _colon: token::Colon(Span::call_site()),
                        value:  Content::Existing(Type::Verbatim(quote!(
                            Vec<u8>
                        ))),
                    },
                    StructField {
                        name:   format_ident!("b"),
                        _colon: token::Colon(Span::call_site()),
                        value:  Content::Existing(Type::Verbatim(quote!(
                            u8
                        ))),
                    },
                ].into_iter()),
            })
        )
    }


    fn punctuated_eq<T: PartialEq>(x: &Punctuated<T, token::Comma>, y: &Punctuated<T, token::Comma>) -> bool {
        let count = x.len();
        if count != y.len() {return false}

        let mut buf = vec![];
        for tx in x {buf.push(tx)}

        let mut pos = 0;
        for ty in y {
            if ty != buf[pos] {return false}
            pos += 1;
        }
        true
    }
    fn punctuated_types_eq(x: &Punctuated<Type, token::Comma>, y: &Punctuated<Type, token::Comma>) -> bool {
        let count = x.len();
        if count != y.len() {return false}

        let mut buf = vec![];
        for tx in x {buf.push(tx)}

        let mut pos = 0;
        for ty in y {
            if !type_eq(ty, buf[pos]) {return false}
            pos += 1;
        }
        true
    }
    fn type_eq(x: &Type, y: &Type) -> bool {
        let (x, y) = (quote!(#x), quote!(#y));
        x.to_string() == y.to_string()
    }
    impl PartialEq for Define {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl PartialEq for New {
        fn eq(&self, other: &Self) -> bool {
            match self {
                New::Struct {
                    _struct,
                    name: self_name,
                    _brace,
                    fields: self_fields
                } => match other {
                    New::Struct {
                        _struct,
                        name: other_name,
                        _brace,
                        fields: other_fields
                    } => self_name==other_name && punctuated_eq(self_fields, other_fields),
                    _ => false,
                }
                New::Enum {
                    _enum,
                    name: self_name,
                    _brace,
                    fields: self_variants
                } => match other {
                    New::Enum {
                        _enum,
                        name: other_name,
                        _brace,
                        fields: other_variants
                    } => self_name==other_name && punctuated_eq(self_variants, other_variants),
                    _ => false
                }
            }
        }
    }
    impl PartialEq for StructField {
        fn eq(&self, other: &Self) -> bool {
            self.name==other.name && self.value==other.value
        }
    }
    impl PartialEq for EnumField {
        fn eq(&self, other: &Self) -> bool {
            self.name==other.name && self.content==other.content
        }
    }
    impl PartialEq for EnumContent {
        fn eq(&self, other: &Self) -> bool {
            match self {
                EnumContent::Struct {
                    _brace,
                    fields: self_fields
                } => match other {
                    EnumContent::Struct {
                        _brace,
                        fields: other_fields
                    } => punctuated_eq(self_fields, other_fields),
                    _ => false,
                },
                EnumContent::Tupple {
                    _paren,
                    types: self_types
                } => match other {
                    EnumContent::Tupple {
                        _paren,
                        types: other_types
                    } => punctuated_types_eq(self_types, other_types),
                    _ => false
                }
            }
        }
    }
    impl PartialEq for Content {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Content::New(self_new) => match other {
                    Content::New(other_new) => self_new == other_new,
                    _ => false,
                }
                Content::Existing(self_type) => match other {
                    Content::Existing(other_type) => type_eq(self_type, other_type),
                    _ => false,
                }
            }
        }
    }

    impl Debug for Define {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Define({:?})", self.0)
        }
    }
    impl Debug for New {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                New::Struct {
                    _struct, name, _brace, fields
                } => {
                    let mut fmt = format!("{name}{{");
                    for field in fields {
                        fmt += &format!("{:?},", field)
                    }
                    fmt + "}"
                },
                New::Enum {
                    _enum,
                    name,
                    _brace,
                    fields
                } => {
                    let mut fmt = format!("{name}{{");
                    for variant in fields {
                        fmt += &format!("{:?},", variant)
                    }
                    fmt + "}"
                }
            })
        }
    }
    impl Debug for StructField {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    impl Debug for EnumField {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    impl Debug for EnumContent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    impl Debug for Content {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}