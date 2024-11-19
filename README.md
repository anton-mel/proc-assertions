![proc-assertions banner](./static/proc_assertions_banner.png)

Verify at compile-time your code via procedural assumptions in Rust; developed by [Efficient Computing Lab](https://www.yecl.org/). 
Find previous commits for the crate v.0.1.1 [here](https://github.com/Ramla-I/static-assertions/tree/antonmel).

## Installation

This crate is available
[on crates.io](https://crates.io/crates/proc_assertions) (read crate [documentation](https://docs.rs/proc_assertions/0.1.1/proc_assertions/)) and can be used by
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

- Contributions are welcome via pull requests to the [GitHub repository](https://github.com/anton-mel/proc-assertions).
- These assertions are only used at compile-time and don't affect the final binary.
- There may be a slight increase in compile times due to additional assertions.
- Install `rust-analyzer` that employs `notify::Watcher` for real-time code monitoring.

## License

This project is licensed under the [MIT License](https://github.com/anton-mel/proc-assertions/LICENSE-MIT).
