use std::borrow::Cow;

use async_trait::async_trait;

#[async_trait]
pub trait Get<T> {
    type E;
    type OUT;

    async fn get_all(engine: T) -> Result<Vec<Self::OUT>, Self::E>;

    async fn get(id: &'static str, engine: T) -> Result<Self::OUT, Self::E>;

    // fn get(id: Cow<'static, str>) -> Self::Data;
}
