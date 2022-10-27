#[cfg(test)]
mod partial_eq {
    use quote::quote;
    use syn::{punctuated::Punctuated, token, Type};
    use crate::internals::define::{
        Define, New, StructField, EnumField, EnumContent, Content,
        interpreter::{List, Def, ContentType, FieldDef}
    };


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
}




mod debug {
    use std::fmt::Debug;
    use quote::quote;
    use crate::internals::define::{
        Define, New, StructField, EnumField, EnumContent, Content,
        interpreter::{List, Def, ContentType}
    };


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
            write!(f, "{}:{:?}", self.name, self.value)
        }
    }
    impl Debug for EnumField {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}{}", self.name, {
                let content = &self.content;
                match content {
                    None => "".into(),
                    Some(enum_content) => format!("{:?}", enum_content)
                }
            })
        }
    }
    impl Debug for EnumContent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                EnumContent::Tupple {
                    _paren, types
                } => {
                    let mut fmt = String::from("(");
                    for t in types {
                        fmt += &quote!(#t).to_string();
                        fmt += ","
                    }
                    fmt + ")"
                },
                EnumContent::Struct {
                    _brace, fields
                } => {
                    let mut fmt = String::from("{");
                    for f in fields {
                        fmt += &format!("{:?},", f);
                    }
                    fmt + "}"
                },
            })
        }
    }
    impl Debug for Content {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Content::Existing(t) => quote!(#t).to_string(),
                Content::New(new) => format!("{:?}", new)
            })
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
}