#![warn(missing_docs)]
//! # Discuits Api
#[macro_use]
extern crate serde;

/// Modules for storage engines and sessions
pub mod engine;
/// Modules for defining `IO` traits for storage engines to use.
pub mod io;
/// Modules for Models
pub mod models;
