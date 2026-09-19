//#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    //missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(missing_docs, unused_variables, unused_imports, dead_code)]
#![doc = include_str!("../README.md")]

//***********************************************
// A/B Traits
//***********************************************
mod ab_traits;
pub(crate) use ab_traits::*;

//***********************************************
// Materials (certs, keys etc.)
// Exports const CA, KEY, CERT
//***********************************************
mod ab_materials;
pub(crate) use ab_materials::*;

//***********************************************
// Utilities relating to testing
//***********************************************
mod ab_util;
pub(crate) use ab_util::*;

//***********************************************
// Mock for Orbit Left/Right I/O
//***********************************************
mod ab_io_orbit;
pub(crate) use ab_io_orbit::*;

//***********************************************
// Mock for std::io::{Read, Write}
//***********************************************
mod ab_io_std;
pub(crate) use ab_io_std::*;

//***********************************************
// I/O Injector upon std Read call <> Orbit
//***********************************************
pub(crate) mod ab_injector;

//***********************************************
// Expected failure modes
//***********************************************
mod ab_failures;
pub use ab_failures::*;

//***********************************************
// Suites under Test represented through Orbits
//***********************************************
pub(crate) mod ab_rustls;
pub(crate) mod ab_ytls;

//***********************************************
// A/B Tester Driver
//***********************************************
mod ab_driver;
pub use ab_driver::*;
