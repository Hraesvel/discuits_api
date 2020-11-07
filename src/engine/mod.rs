use arangors::client::ClientExt;

use crate::engine::db::Db;

pub mod file_system;
pub mod db;


pub struct Session<T> {
    engine: T
}
