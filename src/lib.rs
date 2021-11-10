#![warn(
// missing_docs
)]
//! # Discuits Api

#[macro_use]
extern crate serde;


pub use async_trait::async_trait;

pub use model_write_derive as macros;

pub mod engine;
/// Modules for defining read and writes traits for storage engines.
pub mod io;
/// Modules for Models
pub mod models;

pub mod time;

