use proc_macro2::TokenStream;
use quote::quote;

use super::interpreter::{List, Type};
use crate::internals::Build;


impl Build for List {
    fn build(self) -> proc_macro2::TokenStream {
        let mut result = TokenStream::new();
        
        let list: List = self.into();
        for def in list {
            let name = def.name;
            match def._type {
                Type::Struct => {
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
                Type::Enum => {
                    result.extend(quote!(
                        enum #name
                    ));

                    let mut variants = TokenStream::new();
                    for variant in def.fields {
                        let (name, content) = (variant.name, variant.value_type);
                        variants.extend(quote!(
                            #name #content
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
