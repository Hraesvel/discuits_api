//! Modules for defining `IO` traits for storage engines to use.
pub mod delete;
pub mod read;
pub mod write;

pub use write::*;
pub use read::*;
pub use delete::*;