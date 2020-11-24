use async_trait::async_trait;

#[async_trait]
pub trait Delete<T> {
    async fn remove(id: &str) -> std::io::Result<()>;
}


#[async_trait]
pub trait EngineDelete<T> {
    async fn remove(self, id: &str) -> std::io::Result<()>;
}
