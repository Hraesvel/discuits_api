use crate::models::ReqModelTraits;

/// Trait for implementing `GET` like methods.
/// Where the generic represents the engine i.e. `ArangoDB`
#[crate::async_trait]
pub trait Get<T> {
    type E;
    type Document;

    /// Method to get all Elements
    async fn get_all(engine: &T) -> Result<Vec<Self::Document>, Self::E>;

    /// Method to get a single Element
    async fn get(id: &str, engine: &T) -> Result<Self::Document, Self::E>;

    async fn find<'a>(with: &str, field: &str, engine: &T) -> Result<Self::Document, Self::E>;
}

/// Trait for implementing `GET for engines` like methods.
#[crate::async_trait]
pub trait EngineGet {
    type E;

    /// Method to get all Elements
    async fn get_all<T: ReqModelTraits>(&self) -> Result<Vec<T>, Self::E>;

    /// Method to get a single Element
    async fn get<T: ReqModelTraits>(&self, id: &str) -> Result<T, Self::E>;

    /// Method to find a single Element
    /// with a key(`field`), value pair
    async fn find<T: ReqModelTraits>(&self, value: &str, field: &str) -> Result<T, Self::E>;
}
