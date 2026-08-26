<div align="center">
<h1>Stub</h1>

<a href="https://crates.io/crates/stub">
<img src="https://img.shields.io/crates/v/stub.svg" alt="crates.io"/>
</a>
<a href="https://docs.rs/stub/latest/stub">
<img src="https://docs.rs/stub/badge.svg" alt="docs.rs"/>
</a>
<img src="https://img.shields.io/badge/no__std-compatible-green.svg" alt="no_std"/>
<img src="https://img.shields.io/badge/no__core-compatible-green.svg" alt="no_std"/>
<img src="https://img.shields.io/badge/MSRV-1.70-white.svg" alt="no_std"/>
</div>

A lightweight `no_std` stub type that implements `Default`, `Debug`, `Display`, `Clone` and `Copy`.

Useful as a placeholder in generics, default values, or when you need a type that satisfies common trait bounds without doing anything.

## Features

- **`nightly`** – enables const implementations of `Default` and `Clone` (requires nightly Rust).
- **`no_core`** – removes dependency on `core` (requires nightly). When enabled, only const impls are available.

By default, the crate works on stable Rust with standard `impl`s.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
stub = "0.1"
```

Then use the `Stub` type:

```rust
use stub::Stub;

// As a default
let s: Stub = Default::default();

// As a generic argument
trait MyTrait {
    const VALUE: u32;
}

impl MyTrait for Stub {
    const VALUE: u32 = 42;
}

fn get_value<T: MyTrait>() -> u32 {
    T::VALUE
}

assert_eq!(get_value::<Stub>(), 42);
```

## License

Licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
