use syn::{Fields, ItemStruct, Visibility};
use std::collections::HashSet;
use proc_macro::TokenStream;
use quote::quote;

pub fn assert_private_fields_impl(whitelist: &[String], input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    let whitelist_set: HashSet<String> = whitelist.iter().cloned().collect();
    let mut public_fields = Vec::new();

    // Check fields and identify public fields that should be private
    if let Fields::Named(ref fields) = input.fields {
        for field in fields.named.iter() {
            let field_name = field.ident.as_ref().unwrap().to_string();
            if whitelist_set.contains(&field_name) {
                if let Visibility::Public(_) = field.vis {
                    public_fields.push(field_name);
                }
            }
        }
    }

    let error = if !public_fields.is_empty() {
        let error_message = format!(
            "Struct {} has public fields: {}; these fields must be private.",
            struct_name,
            public_fields.join(", ")
        );
        quote! {
            compile_error!(#error_message);
        }
    } else {
        quote! {}
    };

    let output = quote! {
        #input
        #error
    };

    TokenStream::from(output)
}
