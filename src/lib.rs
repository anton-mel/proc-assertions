//! [![Banner](https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Banner.png)](https://github.com/nvzqz/static-assertions-rs)
//!
//! <div align="center">
//!     <a href="https://crates.io/crates/proc_static_assertions">
//!         <img src="https://img.shields.io/crates/d/proc_static_assertions.svg" alt="Downloads">
//!     </a>
//!     <a href="https://travis-ci.org/nvzqz/static-assertions-rs">
//!         <img src="https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master" alt="Build Status">
//!     </a>
//!     <br><br>
//! </div>
//!
//! Procedural macro [compile-time] assertions as an extension of
//! [`static_assertions`].
//!
//! # Usage
//!
//! There's two main ways of using this crate: as a direct dependency or
//! indirect dependency (via [`static_assertions`]).
//!
//! ## Direct Dependency
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! proc_static_assertions = "0.0.0"
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```
//! #[macro_use]
//! extern crate proc_static_assertions;
//! # fn main() {}
//! ```
//!
//! ## Indirect Dependency
//!
//! Add the following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! static_assertions = { version = "1.1.0", features = ["proc"] }
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```ignore
//! #[macro_use]
//! extern crate static_assertions;
//! ```
//!
//! This will also import all macros in `proc_static_assertions`.
//!
//! # Donate
//!
//! This project is made freely available (as in free beer), but unfortunately
//! not all beer is free! So, if you would like to buy me a beer (or coffee or
//! *more*), then consider supporting my work that's benefited your project
//! and thousands of others.
//!
//! <a href="https://www.patreon.com/nvzqz">
//!     <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
//! </a>
//! <a href="https://www.paypal.me/nvzqz">
//!     <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
//! </a>
//!
//! [`static_assertions`]: https://github.com/nvzqz/static-assertions-rs
//! [crate]: https://crates.io/crates/static_assertions
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [compile-time]: https://en.wikipedia.org/wiki/Compile_time

#![doc(html_root_url = "https://docs.rs/proc_static_assertions/0.0.0")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Icon.png"
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
