use async_trait::async_trait;

/// Trait for implementing `GET` like methods.
#[async_trait]
pub trait Get<T> {
    type E;
    type Element;

    /// Method to get all Elements
    async fn get_all(engine: T) -> Result<Vec<Self::Element>, Self::E>;

    /// Method to get a single Element
    async fn get(id: &'static str, engine: T) -> Result<Self::Element, Self::E>;

    // fn get(id: Cow<'static, str>) -> Self::Data;
}
