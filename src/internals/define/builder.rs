use proc_macro2::TokenStream;
use quote::quote;
use super::interpreter::{List, ContentType};
use crate::internals::Build;


impl Build for List {
    fn build(self) -> proc_macro2::TokenStream {
        let mut result = TokenStream::new();
        
        let list: List = self.into();
        for def in list {
            let name = def.name;
            match def.content_type {
                ContentType::Struct => {
                    result.extend(quote!(
                        struct #name
                    ));

                    let mut fields = TokenStream::new();
                    for field in def.fields {
                        let (name, value_type) = (field.name, field.value_type);
                        fields.extend(quote!(
                            #name: #value_type,
                        ))
                    }
                    result.extend(quote!(
                        { #fields }
                    ))
                },
                ContentType::Enum => {
                    result.extend(quote!(
                        enum #name
                    ));

                    let mut variants = TokenStream::new();
                    for variant in def.fields {
                        let (name, content) = (variant.name, variant.value_type);
                        variants.extend(quote!(
                            #name #content,
                        ))
                    }
                    result.extend(quote!(
                        { #variants }
                    ))
                },
            }
        }

        result
    }
}


#[cfg(test)]
mod test {
    use quote::quote;
    use syn::parse2;
    use crate::internals::{define::Define, Interpret, Build};

    #[test]
    fn build_nested_1() {
        let case = parse2::<Define>(quote!(
            struct NestedStruct {
                a: struct A {
                    b: u8,
                    c: u8,
                },
            }
        )).unwrap(/* this parsing passed in deine::test */)
            .interpret(/* this interpreting passed in define::interpreter::test */);
        assert_eq!(
            case.build().to_string(),
            quote!(
                struct NestedStruct {
                    a: A,
                }
                struct A {
                    b: u8,
                    c: u8,
                }
            ).to_string()
        )
    }
    #[test]
    fn build_nested_2() {
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
        )).unwrap(/* this parsing passed in deine::test */)
            .interpret(/* this interpreting passed in define::interpreter::test */);
        assert_eq!(
            case.build().to_string(),
            quote!(
                struct NestedStruct {
                    a: A,
                    d: D,
                }
                struct D {
                    e: String,
                    f: Vec<u8>,
                }
                struct A {
                    b: u8,
                    c: u8,
                }
            ).to_string()
        )
    }
    #[test]
    fn build_double_nested_1() {
        let case = parse2::<Define>(quote!(
            struct NestedStruct {
                a: struct A {
                    b: struct B {
                        c: u8,
                        d: String,
                    },
                    e: u8,
                },
            }
        )).unwrap(/* this parsing passed in deine::test */)
            .interpret(/* this interpreting would pass in define::interpreter::test */);
        assert_eq!(
            case.build().to_string(),
            quote!(
                struct NestedStruct {
                    a: A,
                }
                struct A {
                    b: B,
                    e: u8,
                }
                struct B {
                    c: u8,
                    d: String,
                }
            ).to_string()
        )
    }
}
