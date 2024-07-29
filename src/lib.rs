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
use parser::whitelist;
use parser::field_whitelist;

mod private_fields;
mod size_align;
mod mutatedby;

mod calledby; // to be removed
mod consumes;
mod mutates;
mod calls;

// Function-like macros in Rust take only one TokenStream parameter and return a TokenStream.
// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
use proc_macro::TokenStream;

use syn::{
    parse_macro_input, DeriveInput, 
    ItemStruct, ItemImpl, ItemFn};

    
/// A procedural macro to assert that all fields in a struct are private.
#[proc_macro_attribute]
pub fn assert_private_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
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
pub fn assert_function_consumes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let types = parse_macro_input!(attr as whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);

    consumes::assert_function_consumes_impl(&types.values, input).into()
}


// Going direct way to protect the function usage is hard, since we 
// don't know where the call is completed outside the whitelist-macro, so we go 
// the other way by generating a check-function __callsite or __mutates awaiting 
// for the function name as an arguemnt. This function is injected in the 
// crate at compile time, so that developer can manually make a check-call via
// Origin::__callsite/__mutates("function_name").

/// A procedural macro attribute to hold the whitelist of functions.
/// Checks if a field of a type is only mutated in certain functions.
#[proc_macro_attribute]
pub fn mutatedby(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let fns = parse_macro_input!(attr as whitelist::WhitelistArgs);

    mutatedby::assert_mutatedby_impl(&fns.values, input).into()
}

/// This macro is to further simplify the process of verification by 
/// automatically generating __mutates() calls when needed.
#[proc_macro_attribute]
pub fn assert_mutates(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = mutatedby::assert_mutates_impl(input);
    TokenStream::from(expanded)
}


/// A function is only called in certain functions.
#[proc_macro_attribute]
pub fn calledby(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fns = parse_macro_input!(attr as whitelist::WhitelistArgs);

    calledby::assert_calledby_impl(&fns.values, input).into()
}

/// This macro is to further simplify the process of verification by 
/// automatically generating __callsite() calls when needed.
#[proc_macro_attribute]
pub fn assert_callsite(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = calledby::assert_callsite_impl(input);
    TokenStream::from(expanded)
}


// This are new correct implementations. 

/// A function is only called in certain functions.
#[proc_macro_attribute]
pub fn calls(attr: TokenStream, item: TokenStream) -> TokenStream {
    let whitelist = parse_macro_input!(attr as whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    calls::assert_call_impl(&whitelist.values, &input, false).into()
}

/// A function is restricted to be called only in certain functions.
#[proc_macro_attribute]
pub fn nocalls(attr: TokenStream, item: TokenStream) -> TokenStream {
    let whitelist = parse_macro_input!(attr as whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    calls::assert_call_impl(&whitelist.values, &input, true).into()
}

/// Checks if a field of a type is only mutated in certain functions.
#[proc_macro_attribute]
pub fn mutates(attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_data = parse_macro_input!(attr as field_whitelist::WhitelistArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    mutates::assert_mutate_impl(&macro_data, &input).into()
}
