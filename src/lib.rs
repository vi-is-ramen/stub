//! A lightweight stub type that implements common `core` traits (`Default`,
//! `Debug`, `Display`, `Clone`, `Copy` and so on).
//!
//! This crate provides a zero-sized type `Stub` that can be used as a placeholder
//! in generic code or as a default value for trait bounds. It is `no_std` compatible
//! and works on both stable and nightly Rust.
//!
//! # Features
//! - **nightly**: enables const impls for `Default` and `Clone` (requires
//!   nightly compiler).
//! - **no_core**: disables the `core` prelude entirely (requires nightly). When
//!   enabled, no one trait would be implemented. Crate still requires `copy`
//!   lang_item definition.
//!
//! # Example
//! ```
//! use stub::Stub;
//!
//! trait Device {
//!     const NAME: &'static str;
//! }
//!
//! impl Device for Stub {
//!     const NAME: &'static str = "{..}";
//! }
//!
//! fn device_name<D: Device = Stub>() -> &'static str {
//!     D::NAME
//! }
//!
//! assert_eq!(device_name(), "{..}");
//! ```

#![cfg_attr(feature = "no_core", feature(no_core))]
#![cfg_attr(feature = "no_core", no_core)]
#![cfg_attr(
    all(feature = "nightly", not(feature = "no_core")),
    feature(const_clone, const_default, const_trait_impl)
)]
#![cfg_attr(
    all(feature = "nightly", feature = "no_core"),
    feature(const_trait_impl)
)]
#![no_std]

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "nightly")]
#[allow(unused_imports)]
pub use nightly::*;

#[cfg(not(feature = "nightly"))]
mod stable;
#[cfg(not(feature = "nightly"))]
#[allow(unused_imports)]
pub use stable::*;
