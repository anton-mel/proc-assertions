use proc_macro::TokenStream;
use quote::{quote_spanned, quote};
use syn::spanned::Spanned;
use syn::{Expr, ExprMethodCall, Type, TypePath};
use syn::visit_mut::{VisitMut, self};

pub fn assert_calledby_impl(allowed_functions: &[String], input: syn::ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;
    let fn_span = input.span();

    // Generate a struct unique function name for the manual mutation check function at the callsite.
    //
    // NOTE: We cannot automatically check the callsite fn, unless use Rust Lints like Clippy.
    // However, that would require forking the repo and directly contributing there.
    let check_fn_ident = syn::Ident::new("__callsite", fn_span);

    // Generate the list of unauthorized checks.
    let unauthorized_check = allowed_functions.iter().map(|allowed_fn| {
        let allowed_fn_ident = syn::Ident::new(allowed_fn, fn_span);
        quote_spanned! { fn_span =>
            if stringify!(#allowed_fn_ident) == caller_name {
                return;
            }
        }
    });

    // Generate the injected AST for the callsite check function.
    let expanded = quote! {
        #input

        #[doc(hidden)]
        pub fn #check_fn_ident(caller_name: &str) {
            #(#unauthorized_check)* // For the DEMO purpose I keep it as panic, fix to compiler_error().
            panic!("Unauthorized function trying to call {}: {}", stringify!(#fn_name), caller_name);
        }
    };

    TokenStream::from(expanded)
}

pub fn assert_callsite_impl(input: syn::ItemFn) -> proc_macro2::TokenStream {
    let fn_name = input.sig.ident.clone();
    let fn_name_str = fn_name.to_string();
    
    // Visitor to find method calls
    struct FnCallVisitor {
        calls: Vec<(proc_macro2::Ident, Type)>,
    }
    
    impl VisitMut for FnCallVisitor {
        // Traverse the body of the function to find method calls
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            // Check if the current expression is a method call
            if let Expr::MethodCall(method_call) = expr {
                if let Some(receiver_type) = extract_receiver_type(&method_call) {
                    // Extract the method and receiver type
                    self.calls.push((method_call.method.clone(), receiver_type));
                }
            }
            // Continue traversing nested expressions
            visit_mut::visit_expr_mut(self, expr);
        }
    }

    // Extract the type of the receiver from the method call
    fn extract_receiver_type(method_call: &ExprMethodCall) -> Option<Type> {
        match &*method_call.receiver {
            Expr::Path(expr_path) => Some(Type::Path(TypePath {
                qself: None,
                path: expr_path.path.clone(),
            })),
            _ => None,
        }
    }
    
    let mut visitor = FnCallVisitor { calls: vec![] };
    let mut input_clone = input.clone();
    visitor.visit_item_fn_mut(&mut input_clone);
    let callsites = visitor.calls;

    // Prepare new TokenStreams for each callsite assertion
    let mut injected_code = proc_macro2::TokenStream::new();
    
    for (_, receiver_type) in callsites {
        let callsite_code = match &receiver_type {
            Type::Path(type_path) => {
                let path = &type_path.path;
                if path.is_ident("self") {
                    quote! {
                        Self::__callsite(#fn_name_str);
                    }
                } else {
                    quote! {
                        #path::__callsite(#fn_name_str);
                    }
                }
            },
            _ => {
                quote! {
                    compile_error!("Unsupported receiver type for __callsite");
                }
            }
        };
        injected_code.extend(quote! {
            #callsite_code
        });
    }
    
    let block = &input.block;
    let stmts = &block.stmts;

    let new_block = quote! {
        {
            #injected_code
            #(#stmts)*
        }
    };

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;

    let result = quote! {
        #(#attrs)*
        #vis #sig #new_block
    };

    result.into()
}
