use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, Visibility};

pub fn assert_private_fields_impl(whitelist: &[String], input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    let whitelist_set: std::collections::HashSet<String> = whitelist.iter().cloned().collect();
    let mut public_fields = Vec::new();

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

    if public_fields.is_empty() {
        TokenStream::new()
    } else {
        let expanded = quote! {
            compile_error!(concat!("Struct ", stringify!(#struct_name), " has public fields: ", #(#public_fields),* ,"; these fields must be private."));
        };
        TokenStream::from(expanded)
    }
}
