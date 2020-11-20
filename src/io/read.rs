use async_trait::async_trait;

use crate::models::RequiredTraits;

/// Trait for implementing `GET` like methods.
#[async_trait]
pub trait Get<T> {
    type E;
    type Element;

    /// Method to get all Elements
    async fn get_all(engine: &T) -> Result<Vec<Self::Element>, Self::E>;

    /// Method to get a single Element
    async fn get(id: &str, engine: &T) -> Result<Self::Element, Self::E>;
}

/// Trait for implementing `GET for engines` like methods.
#[async_trait]
pub trait EngineGet {
    type E;

    /// Method to get all Elements
    async fn get_all<T: RequiredTraits>(&self) -> Result<Vec<T>, Self::E>;

    /// Method to get a single Element
    async fn get<T: RequiredTraits>(&self, id: &str) -> Result<T, Self::E>;
}
