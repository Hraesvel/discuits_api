use async_trait::async_trait;

#[async_trait]
pub trait Write<T> {
    async fn insert(&mut self);

    async fn update(&mut self);
}
