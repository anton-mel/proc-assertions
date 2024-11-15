# proc-assertions

Verify at compile-time your code via procedural assumptions in Rust; developed by [Efficient Computing Lab](https://www.yecl.org/). 
Find previous commits for the crate v.0.1.1 [here](https://github.com/Ramla-I/static-assertions/tree/antonmel).

## Installation

This crate is available
[on crates.io](https://crates.io/crates/proc_assertions) [[documentation](https://docs.rs/proc_assertions/0.1.1/proc_assertions/)] and can be used by
adding the following to your project's
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
proc_assertions = "0.1.1"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
#[macro_use]
extern crate proc_assertions;
```

## Usage

This crate exposes the following proc-macros:
- #[`calls`]
- #[`nocalls`]
- #[`mutates`]
- #[`nomutates`]
- #[`private_fields`]
- #[`size_align`]
- #[`consumes`]

## FAQ

- **Q:** How can I contribute?
Contributions are welcome via pull requests to the [GitHub repository](https://github.com/anton-mel/proc-assertions).
- **Q:** Will this affect my compiled binary?
No, these assertions are only used at compile-time and don't affect the final binary.
- **Q:** Will this affect my compile times?
There may be a slight increase in compile times due to additional checks.

## License

This project is licensed under the [MIT License](https://github.com/anton-mel/proc-assertions/LICENSE-MIT).
