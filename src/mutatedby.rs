use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Type, visit_mut, FnArg};
use syn::visit_mut::VisitMut;

pub fn assert_mutatedby_impl(allowed_functions: &[String], input: syn::ItemImpl) -> TokenStream {
    let struct_name = input.self_ty.clone().into_token_stream();

    // Generate a unique function name for the manual mutation check function at the callsite.
    //
    // NOTE: We cannot automatically check the callsite fn, unless use Rust Lints like Clippy.
    // However, that would require forking the repo and directly contributing there.
    let check_fn_ident = syn::Ident::new("__mutates", proc_macro2::Span::call_site());

    // Generate injected AST for the mutation check.
    let check_fn_code = quote! {
        impl #struct_name {
            pub fn #check_fn_ident(caller_name: &str) {
                match caller_name {
                    #(#allowed_functions => return,)* // For the DEMO purpose I keep it as panic, fix to compiler_error().
                    _ => panic!("Unauthorized function trying to mutate fields in {}: {}", stringify!(#struct_name), caller_name),
                }
            }
        }
    };

    let mut output = input.into_token_stream();
    output.extend(check_fn_code);

    output.into()
}

pub fn assert_mutates_impl(input: syn::ItemFn) -> TokenStream2 {
    let fn_name = input.sig.ident.clone();
    let fn_name_str = fn_name.to_string();
    
    // Visitor to find mutable arguments and detect mutations in the function body.
    struct MutateVisitor {
        mutates: Vec<Type>,
    }
    
    impl VisitMut for MutateVisitor {
        fn visit_fn_arg_mut(&mut self, arg: &mut FnArg) {
            if let FnArg::Typed(pat_type) = arg {
                if let Type::Reference(ref_type) = &*pat_type.ty {
                    if ref_type.mutability.is_some() {
                        // Extract type from mutable reference.
                        self.mutates.push(*ref_type.elem.clone());
                    }
                }
            }
            visit_mut::visit_fn_arg_mut(self, arg);
        }

        fn visit_expr_field_mut(&mut self, i: &mut syn::ExprField) {
            if let syn::Expr::Path(expr_path) = &*i.base {
                if let Some(segment) = expr_path.path.segments.last() {
                    if segment.ident == "self" {
                        // Assume self is mutable if it is being accessed.
                        let ty = Type::Path(syn::TypePath { qself: None, path: expr_path.path.clone() });
                        self.mutates.push(ty);
                    }
                }
            }
            visit_mut::visit_expr_field_mut(self, i);
        }
    }

    let mut visitor = MutateVisitor { mutates: vec![] };
    let mut input_clone = input.clone();
    visitor.visit_item_fn_mut(&mut input_clone);
    let mutates = visitor.mutates;

    // Generate the injected AST for the mutates check function.
    let mut injected_code = TokenStream2::new();
    
    if !mutates.is_empty() {
        let mutates_code = quote! {
            Self::__mutates(#fn_name_str);
        };
        injected_code.extend(mutates_code);
    }

    let block = &input.block;
    let stmts = &block.stmts;

    // Create new block with injected code.
    let new_block = quote! {
        {
            #(#stmts)*
            #injected_code
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
