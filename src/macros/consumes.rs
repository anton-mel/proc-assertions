use proc_macro::TokenStream;
use syn::{FnArg, ItemFn, Receiver};
use quote::quote;

pub fn assert_function_consumes_impl(whitelist: &[String], function: ItemFn) -> TokenStream {
    let mut errors = Vec::new();

    // Iterate over each type in the whitelist.
    for w in whitelist {
        let mut found_match = false;

        for input_arg in &function.sig.inputs {
            match input_arg {
                // Handle named argument types (e.g., `arg1: i32`, `arg2: u8`)
                FnArg::Typed(arg) => {
                    let arg_type = &arg.ty;
                    let arg_type_str = quote! { #arg_type }.to_string();

                    // Check if the whitelist type matches the type string.
                    if is_type_compatible(&arg_type_str, w) {
                        found_match = true;
                        break;
                    }
                }
                // NEW: Handle `self`, `&self`, `mut self`, `&mut self`
                FnArg::Receiver(receiver) => {
                    if is_receiver_compatible(receiver, w) {
                        found_match = true;
                        break;
                    }
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

// Helper function to compare a `Receiver` (`self`, `&self`, etc.) to a whitelist type
fn is_receiver_compatible(receiver: &Receiver, whitelist_type: &str) -> bool {
    let is_ref = receiver.reference.is_some();
    let is_mut = receiver.mutability.is_some();

    match (is_ref, is_mut, whitelist_type) {
        (false, false, "self") => true,         // self (owned)
        (false, true, "mut self") => true,      // mut self (owned, mutable)
        (true, false, "&self") => true,         // &self (borrowed, immutable)
        (true, true, "&mut self") => true,      // &mut self (borrowed, mutable)
        _ => false,
    }
}

fn is_type_compatible(arg_type_str: &str, whitelist_type: &str) -> bool {
    // println!("Comparing argument type `{}` with whitelist type `{}`", arg_type_str, whitelist_type);
    // Basic match improvement: trim and compare exact strings
    arg_type_str.trim() == whitelist_type
}
