use arangors::document::response::DocumentResponse;
use async_trait::async_trait;

use crate::models::{BoxedDoc, ReqModelTraits};

#[async_trait]
pub trait Write<T>
where
    T: ReqModelTraits,
{
    type E;
    type Document;

    async fn insert(&self, doc: T) -> Result<(String, Box<dyn BoxedDoc>), Self::E>;

    async fn update(&self) -> Result<(), Self::E>;
}

#[async_trait]
pub trait EngineWrite {
    type E;

    /// Method to inserting a new document
    async fn insert<T: ReqModelTraits + BoxedDoc + 'static>(
        &self,
        doc: T,
    ) -> Result<(String, Box<dyn BoxedDoc>), Self::E>;

    /// Method to updating a single document
    async fn update<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E>;
}
