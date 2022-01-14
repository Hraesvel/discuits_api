use serde::de::DeserializeOwned;

#[crate::async_trait]
pub trait EngineDelete {
    type E;

    async fn remove<T>(&self, id: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned + Send + Sync;
}
