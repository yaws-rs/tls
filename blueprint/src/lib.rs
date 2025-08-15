#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

//***********************************************
// Re-Exports
//***********************************************

//-----------------------------------------------
// All Errors
//-----------------------------------------------
mod error;
pub use error::*;

//-----------------------------------------------
// Blueprint impl
//-----------------------------------------------
mod tls_blueprints;
pub use tls_blueprints::*;

mod tls_entities;
pub use tls_entities::*;

//-----------------------------------------------
// Internal fix-ups
//-----------------------------------------------
// Gate with rustls use in future as it requres it.
#[cfg(all(not(feature = "std"), not(test)))]
extern crate alloc;
#[cfg(all(not(feature = "std"), not(test)))]
pub(crate) use alloc::sync::Arc;
#[cfg(any(feature = "std", test))]
pub(crate) use std::sync::Arc;
