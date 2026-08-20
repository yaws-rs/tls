//#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
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

pub use ytls_traits::{CryptoConfig, CryptoRng};
#[cfg(feature = "server")]
pub use ytls_server::{Alpn};

//-----------------------------------------------
// All Errors
//-----------------------------------------------
mod error;
#[doc(inline)]
pub use error::*;

//-----------------------------------------------
// Blueprint impl
//-----------------------------------------------

mod ytls_blueprint;
#[doc(inline)]
pub use ytls_blueprint::*;

#[cfg(feature = "server")]
mod ytls_server;
#[doc(inline)]
#[cfg(feature = "server")]
pub use ytls_server::*;

mod position;
#[doc(inline)]
pub use position::*;

/*
mod tls_blueprints;
pub use tls_blueprints::*;

mod tls_entities;
pub use tls_entities::*;

*/

/*
#[cfg(feature = "util")]
pub mod util;

pub(crate) mod rustls;
*/
