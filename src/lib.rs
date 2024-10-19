//! [![Banner](https://raw.githubusercontent.com/anton-mel/proc-assertions/main/static/proc_assertions_banner.png)]
//!
//! <div align="right">
//!     <a href="https://crates.io/crates/proc_assertions">
//!         <img src="https://img.shields.io/crates/d/proc_assertions" alt="Downloads">
//!     </a>
//!     <br><br>
//! </div>
//!
//! # Overview
//!
//! The `proc_assertions` crate provides a set of procedural macros for enforcing compile-time assertions in Rust code. These macros help ensure the correctness of your code by validating structural and behavioral properties, such as field visibility, size, alignment, and mutation rules. 
//!
//! # Usage
//!
//! You can use this crate as either a direct dependency or an indirect dependency through [`proc_assertions`]. 
//!
//! ## Direct Dependency
//!
//! Add the following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! proc_assertions = "0.1.0"
//! ```
//!
//! Then include it in your crate root (`main.rs` or `lib.rs`):
//!
//! ```rust
//! #[macro_use]
//! extern crate proc_assertions;
//! # fn main() {}
//! ```
//!
//! ## Indirect Dependency
//!
//! Alternatively, you can add it with features:
//!
//! ```toml
//! [dependencies]
//! proc_assertions = { version = "0.1.0", features = ["proc"] }
//! ```
//!
//! This will also import all macros available in `proc_assertions`.
//!
//! # License
//!
//! This project is licensed under the [MIT License](LICENSE).
//!
#![doc(html_root_url = "https://docs.rs/proc_assertions/0.1.0")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anton-mel/proc-assertions/main/static/proc_assertions_logo.png"
)]
#![deny(missing_docs)]

// Procedural macros operate on the abstract syntax tree (AST) of the code.
// The quote crate helps in constructing these syntax trees. Using `quote`
// allows us to write code almost as if we're directly writing Rust code,
// facilitating seamless injection of variables like #input, #size, or #align.
// Without quote, we would have to manually construct token streams and manage
// all syntactic intricacies ourselves, which is prone to errors and cumbersome.
extern crate quote;
extern crate proc_macro;
extern crate syn;

// The release notes for syn v.2 say that AttributeArgs was removed.
// Here we build out custom parse_macro_input! implementation.
// https://docs.rs/syn/latest/syn/meta/fn.parser.html#example
mod parser;
mod macros;

use parser::whitelist;
use parser::field_whitelist;

use macros::private_fields;
use macros::size_align;
use macros::consumes;
use macros::mutates;
use macros::calls;

// Function-like macros in Rust take only one TokenStream parameter and return a TokenStream.
// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
use proc_macro::TokenStream;

use syn::{
    parse_macro_input, DeriveInput, 
    ItemStruct, ItemFn};

    
/// A procedural macro to assert that all fields in a struct are private.
#[proc_macro_attribute]
pub fn private_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let fields = parse_macro_input!(attr as whitelist::WhitelistArgs);

    private_fields::assert_private_fields_impl(&fields.values, input).into()
}

/// A procedural macro attribute to assert the size and alignment of a struct.
#[proc_macro_attribute]
pub fn assert_align_size(attr: TokenStream, item: TokenStream) -> TokenStream {
    let size_align = parse_macro_input!(attr as size_align::SizeAlign);
    let size_align::SizeAlign { size, align } = size_align;

    let input = parse_macro_input!(item as DeriveInput);

    size_align::assert_align_size_impl(size, align, &input).into()
}

/// A function consumes a list of instances of certain types. Allows to 
/// quickly assert function argument types where Rustc cannot access.
#[proc_macro_attribute]
pub fn consumes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let types = parse_macro_input!(attr as whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);

    consumes::assert_function_consumes_impl(&types.values, input).into()
}


/// Checks if a function includes all the whitelisted method calls.
/// This macro ensures that only the methods listed in the whitelist are called within the function.
/// If any method call outside of the whitelist is found, a compile-time error will be generated.
/// 
/// Usage: #[calls("func1", "func2", "func3"...)]
#[proc_macro_attribute]
pub fn calls(attr: TokenStream, item: TokenStream) -> TokenStream {
    let whitelist = parse_macro_input!(attr as whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    calls::assert_call_impl(&whitelist.values, &input).into()
}

/// Checks if only whitelisted fields of an instance type are mutated by a function.
/// This macro enforces that only the fields listed in the whitelist can be mutated by the function.
/// If any field not in the whitelist is mutated, a compile-time error will be generated.
///
/// Usage: `#[mutates(MyStructName: "field1", "field2", "field3", ...)]`
#[proc_macro_attribute]
pub fn mutates(attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_data = parse_macro_input!(attr as field_whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    mutates::assert_mutate_impl(&macro_data, &input, false).into()
}

#[proc_macro_attribute]
/// Checks if a public field of an instance type is only mutated by specific whitelisted functions.
/// This macro ensures that only the functions listed in the whitelist can mutate public fields of the instance type.
/// If any public field is mutated by a function not in the whitelist, a compile-time error will be generated.
///
/// Usage: `#[nomutates(MyStructName: "func1", "func2", "func3", ...)]`
pub fn nomutates(attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_data = parse_macro_input!(attr as field_whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    mutates::assert_mutate_impl(&macro_data, &input, true).into()
}
