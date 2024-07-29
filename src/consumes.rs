use proc_macro::TokenStream;
use syn::{Error, FnArg, ItemFn};
use quote::quote;

pub fn assert_function_consumes_impl(whitelist: &[String], function: ItemFn) -> TokenStream {
    let mut errors = Vec::new();

    for w in whitelist {
        // Check if any argument type matches the current whitelist type.
        let mut found_match = false;
        for input_arg in &function.sig.inputs {
            if let FnArg::Typed(arg) = input_arg {
                let arg_type = &arg.ty;
                let arg_type_str = quote! { #arg_type }.to_string();
                let cleaned_type_str = clean_type_string(&arg_type_str);

                if is_type_compatible(&cleaned_type_str, w) {
                    found_match = true;
                    break;
                }
            }
        }

        if !found_match {
            errors.push(Error::new(
                function.sig.ident.span(),
                format!("Compilation error: `{}` type is not consumed by the `{}` function", w, function.sig.ident),
            ));
        }
    }

    if !errors.is_empty() {
        let mut error_message = String::from("Function argument types do not match the whitelist:\n");
        for error in &errors {
            error_message.push_str(&format!(" - {}\n", error));
        }

        return TokenStream::from(quote! {
            compile_error!(#error_message);
        });
    }

    TokenStream::from(quote! { #function })
}

fn clean_type_string(type_str: &str) -> String {
    type_str.replace(" ", "") // Remove any spaces
            .replace("\n", "") // Remove new lines
            .replace("\t", "") // Remove tabs
}

fn is_type_compatible(arg_type_str: &str, whitelist_type: &str) -> bool {
    arg_type_str.contains(whitelist_type)
}
