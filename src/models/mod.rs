use arangors::document::options::InsertOptions;
use async_trait::async_trait;

use crate::engine::db::arangodb::aql_snippet;
use crate::engine::db::Db;
use crate::engine::EngineError;
use crate::io::read::EngineGet;

pub mod album;
pub mod artist;
pub mod inventory;
pub mod variant;


pub trait ReqModelTraits: serde::de::DeserializeOwned + serde::ser::Serialize + DocDetails + Sync + Send + Clone {}

pub trait BoxedDoc : std::fmt::Debug {}

pub trait DocDetails {
    fn collection_name<'a>() -> &'a str;

    fn key(&self) -> String;

    fn id(&self) -> String;
}
