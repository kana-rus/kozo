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
            result.extend(match def._type {
                Type::Struct => quote!(struct #name),
                Type::Enum   => quote!(enum #name),
            });

            let mut fields = TokenStream::new();
            for field in def.fields {
                let (name, value_type) = (field.name, field.value_type);
                fields.extend(quote!(
                    #name: #value_type,
                ))
            }
            result.extend(quote!(
                {#fields}
            ))
        }

        result
    }
}
