use std::borrow::Cow;

use arangors::document::options::InsertOptions;
use async_trait::async_trait;

use crate::engine::db::Db;
use crate::engine::EngineError;
use crate::io::write::Write;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Inventory {
    _id: Cow<'static, str>,
    _key: Cow<'static, str>,
    count: u8,
}

#[async_trait]
impl Write<Inventory> for Db {
    type E = EngineError;
    type Element = Inventory;

    async fn insert(&self, doc: Inventory) -> Result<(), EngineError>
    {
        let col = self.db().collection("inventory").await?;
        col.create_document(doc, InsertOptions::default()).await?;
        Ok(())
    }

    async fn update(&self) -> Result<(), Self::E> {
        unimplemented!()
    }
}
