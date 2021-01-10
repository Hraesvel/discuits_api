use async_trait::async_trait;

use crate::engine::db::arangodb::aql_snippet;
use crate::engine::db::Db;
use crate::engine::EngineError;
use crate::io::read::EngineGet;
use arangors::document::options::InsertOptions;

pub mod album;
pub mod artist;
pub mod inventory;
pub mod variant;


pub trait ReqModelTraits: serde::de::DeserializeOwned + serde::ser::Serialize + DocDetail + Sync + Send {}

pub trait DocDetail {
    fn collection_name<'a>() -> &'a str;

    fn key(&self) -> String;

    fn id(&self) -> String;
}
