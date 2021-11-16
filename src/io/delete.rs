use serde::de::DeserializeOwned;

#[crate::async_trait]
pub trait Delete<T> {
    type E;
    async fn remove(&self, id: &str) -> Result<T, Self::E>;
}

#[crate::async_trait]
pub trait EngineDelete {
    type E;

    async fn remove<T>(&self, id: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned + Send + Sync;
}
