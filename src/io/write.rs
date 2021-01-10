use async_trait::async_trait;

use crate::models::ReqModelTraits;

#[async_trait]
pub trait Write<T> {
    type E;
    type Document;

    async fn insert(&self, doc: T) -> Result<(), Self::E>;

    async fn update(&self) -> Result<(), Self::E>;
}


#[async_trait]
pub trait EngineWrite {
    type E;

    /// Method to inserting a new document
    async fn insert<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E>;

    /// Method to updating a single document
    async fn update<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E>;
}

