#![warn(missing_docs)]
//! # Discuits Api
#[macro_use]
extern crate serde;

/// Modules for storage engines and sessions
pub mod engine;
/// Modules for defining read and writes traits for storage engines.
pub mod io;
/// Modules for Models
pub mod models;
