use std::borrow::Cow;

use arangors::document::options::InsertOptions;
use async_trait::async_trait;

use crate::engine::db::{Db, DbActions};
use crate::engine::EngineError;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Inventory {
    key: Cow<'static, str>,
    count: u8,
}

#[async_trait]
impl DbActions<Inventory> for Db {
    async fn insert(&self, doc: Inventory) -> Result<(), EngineError> {
        let col = self.db().collection("inventory").await?;
        let doc = col
            .create_document(doc, InsertOptions::default())
            .await?;
        Ok(())
    }
}