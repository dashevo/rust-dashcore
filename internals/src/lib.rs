// Written by the Rust Dash developers.
// SPDX-License-Identifier: CC0-1.0

//! # Rust DashCore Internal
//!
//! This crate is only meant to be used internally by crates in the
//! [rust-dash](https://github.com/rust-dashcore) ecosystem.
//!

#![no_std]
// Experimental features we need.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Coding conventions
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod hex;
pub mod macros;

/// Mainly reexports based on features.
pub(crate) mod prelude {
    #[cfg(feature = "alloc")]
    pub(crate) use alloc::string::String;
}
