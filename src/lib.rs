#![warn(missing_docs)]
//! # Discket Api
#[macro_use]
extern crate serde;

/// Modules for defining `IO` traits for storage engines to use.
pub mod io;
/// Modules for Models
pub mod models;
/// Modules for storage engines and sessions
pub mod engine;

