use syn::{
    Block, Local, ExprField, ExprClosure, ExprBlock,
    FnArg, ExprPath, ExprIf, ExprWhile, ExprForLoop,
    punctuated::Punctuated, token::Comma, Expr, Member,
    ItemFn, Pat, Type, Stmt, TypePath, ExprCall, PatIdent};
use proc_macro2::TokenStream as ProcTokenStream;
use crate::field_whitelist::WhitelistArgs;
use std::collections::HashSet;
use quote::quote;


pub fn assert_mutate_impl(macro_data: &WhitelistArgs, function: &ItemFn) -> ProcTokenStream {
    // NOTE: For better code layout, we will require separate proc-macro for each field
    // to be whitelisted type of #[struct_name: (field1, field2, field3, ...)].
    
    let struct_name = &macro_data.struct_name;
    let whitelist = &macro_data.values;

    let mut errors: Vec<Error> = Vec::new();
    let inputs: &Punctuated<FnArg, Comma> = &function.sig.inputs;
    let block: &Box<Block> = &function.block;

    // Track found instances for further mutation checks.
    let mut found_instances = HashSet::new();
    // Entry point: figure out the instance name by exploring the function input arguments.
    // If not found, then need to parse the function body for the inner declaration check.
    extract_instance_names(inputs, struct_name, &mut found_instances);

    // Parse function recursively and as a state machine
    // extract new definitions on the way if needed.
    check_block_for_mutation(
        block, 
        whitelist,
        &mut found_instances, 
        struct_name, 
        &mut errors);


    if !errors.is_empty() {
        // Construct the error message based on each isntance of the struct_name.
        let header = "Function contains mutations to non-whitelisted struct fields:\n";
        let error_messages: Vec<String> = errors.iter()
            .map(|e| format!(" - {}", e.message))
            .collect();
        let error_message = [header, &error_messages.join("\n")].concat();

        let tokens = quote! { compile_error!(#error_message); };
        return tokens.into();
    }

    // Return the original function if no errors.
    let output = quote! { #function };
    output.into()
}

/// Extracts all instance names from given function 
/// arguments if matches the specified struct_name.
fn extract_instance_names(
    inputs: &Punctuated<FnArg, Comma>,
    struct_name: &str,
    found_instances: &mut HashSet<String>
) {
    for arg in inputs {
        match arg {
            FnArg::Typed(pat_type) => {
                let pat = &*pat_type.pat;
                let ty = &*pat_type.ty;

                match pat {
                    Pat::Ident(pat_ident) => {
                        // println!("Argument pattern: {:?}", pat_ident.ident);
                        if let Type::Path(TypePath { path, .. }) = ty {
                            if path.is_ident(struct_name) {
                                found_instances.insert(pat_ident.ident.to_string());
                            }
                        }
                    }
                    _ => {
                        // Raise a compile-time error if a non-identifier pattern is found, since we cannot parse it.
                        panic!("This macro requires all function arguments to be explicitly typed. \n
                        \t Non-typed argument detected: {:?}", quote! { #pat }.to_string());
                    }
                }
            }

            FnArg::Receiver(receiver) => {
                // Handle the case where the argument is `self` for methods.
                if receiver.reference.is_some() || receiver.mutability.is_some() {
                    found_instances.insert("self".to_string());
                }
            }
        }
    }
}

/// Extracts all inner instance names from the function 
/// body if matches the specified struct_name on Expr::CAll.
fn extract_inner_instance(
    left: &Pat,
    right: &Expr,
    found_instances: &mut HashSet<String>,
    struct_name: &str,
) {
    // TODO: Check later on if there are more 
    // complicated cases for the struct initialization.
    if let Expr::Call(ExprCall { func, .. }) = right {
        if let Expr::Path(ExprPath { path, .. }) = &**func {
            let segments = &path.segments;
            // Ensure the first segment matches the struct_name.
            if segments.len() > 0 && segments[0].ident == struct_name {
                // Check if the next segment is an initialization method.
                if segments.len() > 1 {
                    let init_method = &segments[1].ident.to_string();
                    if init_method == "default" || init_method == "new" {
                        // Extract instance name from the left side of the initialization.
                        if let Pat::Ident(PatIdent { ident, .. }) = left {
                            found_instances.insert(ident.to_string());
                            // println!("Instance found: {}", ident.to_string());
                        }
                    }
                }
            }
        }
    }
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
    field_ident_str: &String, 
    whitelist: &[String], 
    errors: &mut Vec<Error>, 
    message: &str
) {
    if !whitelist.contains(field_ident_str) {
        // Custom assetion based on whitelist data and found AST calls.
        errors.push(Error::new(format!("{}: `{}`", message, field_ident_str)));
    }
}

#[allow(dead_code)]
fn print_ast<T>(item: &T, label: &str)
where
    T: quote::ToTokens,
{
    // A helper function to print AST tokens;
    let tokens: ProcTokenStream = quote! { #item };
    let item_string = tokens.to_string();
    println!("{}: {}", label, item_string);
}

// Recursive check all statements in the block.
fn check_block_for_mutation(
    block: &Block,
    whitelist: &[String],
    found_instances: &mut HashSet<String>,
    struct_name: &str,
    errors: &mut Vec<Error>,
) {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Expr(expr, _) => {
                // print_ast(expr, "Found Expression");
                // Explore Netsted Expression for struct field mutation.
                check_expr_for_mutation(expr, whitelist, errors, found_instances, struct_name);
            }
            Stmt::Local(Local { pat, init, .. }) => {
                if let Some(init) = init {
                    // print_ast(&init.expr, "Found Initialization Expression");
                    // Extract instance name if initialization expression is a struct creation.
                    extract_inner_instance(pat, &init.expr, found_instances, struct_name);
                    // Check the initialization expression for instance names and mutation.
                    check_expr_for_mutation(&init.expr, whitelist, errors, found_instances, struct_name);
                }
            }
            _ => {}
        }
    }
}


fn check_expr_for_mutation(
    expr: &Expr,
    whitelist: &[String],
    errors: &mut Vec<Error>,
    found_instances: &mut HashSet<String>,
    struct_name: &str,
) {
    match expr {
        Expr::Binary(binary_expr) => {
            // Handle various binary operations, including compound assignments.
            if let Expr::Field(ExprField { base, member, .. }) = &*binary_expr.left {
                if let Member::Named(field_ident) = member {
                    // Check if the base is one of the found instances.
                    if let Expr::Path(ExprPath { path, .. }) = &**base {
                        if let Some(instance) = path.get_ident() {
                            let instance_name = instance.to_string();
                            if found_instances.contains(&instance_name) {
                                let field_ident_str = field_ident.to_string();
                                check_whitelist(
                                    &field_ident_str,
                                    whitelist,
                                    errors,
                                    &format!("Mutation to field `{}` is not whitelisted", field_ident_str)
                                );
                            }
                        }
                    }
                }
            }
        }

        Expr::Assign(assign_expr) => {
            // Handle simple assignments (fails for everything => this is a mutation).
            if let Expr::Field(ExprField { base, member, .. }) = &*assign_expr.left {
                if let Member::Named(field_ident) = member {
                    if let Expr::Path(ExprPath { path, .. }) = &**base {
                        if let Some(instance) = path.get_ident() {
                            let instance_name = instance.to_string();
                            if found_instances.contains(&instance_name) {
                                let field_ident_str = field_ident.to_string();
                                check_whitelist(
                                    &field_ident_str,
                                    whitelist,
                                    errors,
                                    &format!("Mutation to field `{}` is not whitelisted", field_ident_str)
                                );
                            }
                        }
                    }
                }
            }
        }

        Expr::Block(ExprBlock { block, .. }) => {
            // Handle a block of code: `{ ... }`.
            check_block_for_mutation(&block, whitelist, found_instances, struct_name, errors);
        }

        Expr::If(ExprIf { then_branch, else_branch, .. }) => {
            // Process the `then` block.
            check_block_for_mutation(&then_branch, whitelist, found_instances, struct_name, errors);
            // Process the `else` branch if present.
            if let Some((_, else_expr)) = else_branch {
                match &**else_expr {
                    Expr::Block(ExprBlock { block, .. }) => {
                        // Process the block inside `else_expr`
                        check_block_for_mutation(&block, whitelist, found_instances, struct_name, errors);
                    },
                    // Handle other types of `else_expr` if necessary
                    _ => check_expr_for_mutation(expr, whitelist, errors, found_instances, struct_name),
                }
            }
        }

        Expr::While(ExprWhile { body, .. }) => {
            // Handle the expression inside the while loop (always block).
            check_block_for_mutation(
                &body, 
                whitelist, 
                found_instances, 
                struct_name,
                errors);
        }

        Expr::ForLoop(ExprForLoop { body, .. }) => {
            // Handle the expression inside the for loop (always block).
            check_block_for_mutation(&body, whitelist, found_instances, struct_name, errors);
        }

        Expr::Closure(ExprClosure { body, .. }) => {
            // Handle closures (either block or expression).
            if let Expr::Block(ExprBlock { block, .. }) = &**body {
                check_block_for_mutation(block, whitelist, found_instances, struct_name, errors);
            } else {
                check_expr_for_mutation(body, whitelist, errors, found_instances, struct_name);
            }
        }

        _ => {}
    }
}
