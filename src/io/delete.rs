use crate::models::ReqModelTraits;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Delete<T> {
    type E;
    async fn take(&self, id: &str) -> Result<T, Self::E>;
}

#[async_trait]
pub trait EngineDelete {
    type E;

    async fn remove<T>(&self, id: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned + Send + Sync;
}
