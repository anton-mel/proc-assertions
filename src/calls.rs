use syn::{
    Block, Local, ExprIf, ExprWhile, 
    ExprForLoop, ExprMethodCall, ExprBlock, 
    ItemFn, Expr, ExprCall, Stmt, ExprClosure};
use proc_macro2::TokenStream as ProcTokenStream;
use proc_macro::TokenStream;
use quote::quote;

pub fn assert_call_impl(whitelist: &[String], function: &ItemFn, restricted_mode: bool) -> ProcTokenStream {
    let mut errors = Vec::new();
    let block = &function.block;

    // Start the recursive checking from the function body.
    check_block_for_calls(block, whitelist, &mut errors, restricted_mode);

    if !errors.is_empty() {
        let mut error_message = String::from("Function contains calls restricted by the whitelist macro:\n");
        
        for error in &errors {
            error_message.push_str(&format!(" - {}\n", error.message));
        }

        return TokenStream::from(quote! {
            compile_error!(#error_message);
        }).into();
    }

    TokenStream::from(quote! { #function }).into()
}


#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Self {
        Error { message }
    }
}

fn check_whitelist(
    name: &str, 
    whitelist: &[String], 
    errors: &mut Vec<Error>, 
    message: &str,
    mode: bool,
) {
    let is_whitelisted = whitelist.contains(&name.to_string());

    if mode {
        if is_whitelisted {
            // When mode is true, error if the name is found in the whitelist.
            errors.push(Error::new(format!("{}: `{}` is whitelisted but should not be.", message, name)));
        }
    } else {
        if !is_whitelisted {
            // When mode is false, error if the name is not found in the whitelist.
            errors.push(Error::new(format!("{}: `{}` is not whitelisted.", message, name)));
        }
    }
}


fn _print_ast<T>(item: &T, label: &str)
where
    T: quote::ToTokens,
{
    // A helper function to print AST tokens;
    let tokens: ProcTokenStream = quote! { #item };
    let item_string = tokens.to_string();
    println!("{}: {}", label, item_string);
}

/// Recursively check all statements in a block.
fn check_block_for_calls(
    block: &Block, 
    whitelist: &[String], 
    errors: &mut Vec<Error>, 
    mode: bool
) {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Expr(expr, _) => {
                // print_ast(expr, "Found Expression");
                // Explore Netsted Expression for new calls.
                check_expr_for_calls(expr, whitelist, errors, mode);
            }
            Stmt::Local(Local { init, .. }) => {
                // Handle variable definitions.
                if let Some(init) = init {
                    // print_ast(&init.expr, "Found Initialization Expression");
                    // Explore the local `let i = {__callsite__};` initialization.
                    check_expr_for_calls(&init.expr, whitelist, errors, mode);
                }
            }
            _ => {}
        }
    }
}

/// NOTE: Assumes that Rustc lint will catch any repetative names between instances.
fn check_expr_for_calls(expr: &Expr, whitelist: &[String], errors: &mut Vec<Error>, mode: bool) {
    match expr {
        Expr::Call(ExprCall { func, .. }) => {
            // Handle simple function calls.
            if let Expr::Path(path) = &**func {
                let func_name = path.path.segments.last()
                .map(|seg| seg.ident.to_string());
                
                if let Some(func_name) = func_name {
                    check_whitelist(
                        &func_name, 
                        whitelist, 
                        errors, 
                        "Function calls a non-whitelisted function",
                        mode,
                    );
                }
            }
        }
        
        Expr::MethodCall(ExprMethodCall { method, .. }) => {
            // Handle method calls type of instance.method_call().
            let method_name = method.to_string();

            check_whitelist(
                &method_name, 
                whitelist, 
                errors, 
                "Method call to a non-whitelisted method",
                mode,
            );
        }

        Expr::Block(ExprBlock { block, .. }) => {
            // Handle a block of code: `{ ... }`.
            check_block_for_calls(block, whitelist, errors, mode);
        }

        Expr::If(ExprIf { then_branch, else_branch, .. }) => {
            // Process the `then` block.
            check_block_for_calls(&then_branch, whitelist, errors, mode);
            // Process the `else` branch if present.
            if let Some((_, else_expr)) = else_branch {
                match &**else_expr {
                    Expr::Block(ExprBlock { block, .. }) => {
                        // Process the block inside `else_expr`
                        check_block_for_calls(&block, whitelist, errors, mode);
                    },
                    // Handle other types of `else_expr` if necessary
                    _ => check_expr_for_calls(expr, whitelist, errors, mode),
                }
            }
        }

        Expr::While(ExprWhile { body, .. }) => {
            // Handle the expression inside the while loop (always block).
            check_block_for_calls(&body, whitelist, errors, mode);
        }
        
        Expr::ForLoop(ExprForLoop { body, .. }) => {
            // Handle the expression inside the for loop (always block).
            check_block_for_calls(&body, whitelist, errors, mode);
        }

        Expr::Closure(ExprClosure { body, .. }) => {
            // Handle closures (either block or expression).
            if let Expr::Block(ExprBlock { block, .. }) = &**body {
                check_block_for_calls(block, whitelist, errors, mode);
            } else {
                check_expr_for_calls(body, whitelist, errors, mode);
            }
        }
        
        _ => {} // Handle other nested expression types if I missed anything important.
        // Read more via https://jeltef.github.io/derive_more/syn/enum.Expr.html.
    }
}
