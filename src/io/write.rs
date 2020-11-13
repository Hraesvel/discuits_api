use async_trait::async_trait;

#[async_trait]
pub trait Write<T> {
    type E;
    type Element;

    async fn insert(&self, doc: T) -> Result<(), Self::E>;

    async fn update(&self) -> Result<(), Self::E>;
}
