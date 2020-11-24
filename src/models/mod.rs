use async_trait::async_trait;

use crate::engine::db::arangodb::aql_snippet;
use crate::engine::db::Db;
use crate::engine::EngineError;
use crate::io::read::EngineGet;

pub mod album;
pub mod artist;
pub mod inventory;

pub trait RequiredTraits: serde::de::DeserializeOwned + serde::ser::Serialize + DocDetail + Sync + Send {}

pub trait DocDetail {
    fn collection_name<'a>() -> &'a str;

    fn key<'a>(&self) -> String;
}
