# proc-assertions

Verify your code via procedural assumptions in Rust developed by [Efficient Computing Lab](https://www.yecl.org/).

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
- #[`consumes!`]

