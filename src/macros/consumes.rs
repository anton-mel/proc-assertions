use proc_macro::TokenStream;
use syn::{FnArg, ItemFn};
use quote::quote;

pub fn assert_function_consumes_impl(whitelist: &[String], function: ItemFn) -> TokenStream {
    let mut errors = Vec::new();

    // Iterate over each type in the whitelist.
    for w in whitelist {
        let mut found_match = false;

        for input_arg in &function.sig.inputs {
            if let FnArg::Typed(arg) = input_arg {
                let arg_type = &arg.ty;
                let arg_type_str = quote! { #arg_type }.to_string();

                // Check if the whitelist type matches the type string.
                if is_type_compatible(&arg_type_str, w) {
                    found_match = true;
                    break;
                }
            }
        }

        if !found_match {
            errors.push(syn::Error::new(
                function.sig.ident.span(),
                format!("Consumes-macro error: `{}` type is not consumed by the `{}` function", w, function.sig.ident),
            ));
        }
    }

    if !errors.is_empty() {
        let error_messages: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
        let error_message = error_messages.join("\n");

        return TokenStream::from(quote! {
            compile_error!(#error_message);
        });
    }

    TokenStream::from(quote! { #function })
}

fn is_type_compatible(arg_type_str: &str, whitelist_type: &str) -> bool {
    // println!("Comparing argument type `{}` with whitelist type `{}`", arg_type_str, whitelist_type);
    // Basic match improvement: trim and compare exact strings
    arg_type_str.trim() == whitelist_type
}
