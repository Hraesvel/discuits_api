pub mod album;
pub mod artist;
pub mod inventory;
pub mod variant;
pub mod fake;


#[cfg(feature="arangodb")]
pub mod edge;


pub fn timestamp() -> u64 {
 0
}

pub trait ReqModelTraits:
    serde::de::DeserializeOwned + serde::ser::Serialize + DocDetails + Sync + Send + Clone
{
}

pub trait BoxedDoc: std::fmt::Debug {}

pub trait DocDetails {
    fn collection_name<'a>() -> &'a str;

    fn key(&self) -> String;

    fn id(&self) -> String;
}
