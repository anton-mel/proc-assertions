# proc-assertions

Verify at compile-time your code via procedural assumptions in Rust; developed by [Efficient Computing Lab](https://www.yecl.org/). 
Find previous commits for the crate 1.1.0 version [here](https://github.com/Ramla-I/static-assertions/tree/antonmel).

## Installation

This crate is available
[on crates.io](https://crates.io/crates/proc_assertions) and can be used by
adding the following to your project's
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
proc_assertions = "1.1.0"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
#[macro_use]
extern crate proc_static_assertions;
```

## Usage

This crate exposes the following proc-macros:
- #[`calls`]
- #[`notcalls`]
- #[`mutates`]
- #[`notmutates`]
- #[`private_fields`]
- #[`size_align`]
- #[`consumes`]

## FAQ

- **Q:** When would I want to use this?
- **Q:** How can I contribute?
- **Q:** Will this affect my compiled binary?
- **Q:** Will this affect my compile times?

## License

This project is licensed under the [MIT License](https://github.com/anton-mel/proc-assertions/LICENSE-MIT).
