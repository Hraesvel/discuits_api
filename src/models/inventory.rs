use std::borrow::Cow;

use arangors::document::options::InsertOptions;

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
        let mut col = self.db().collection("inventory").await?;
        let doc = col
            .create_document(doc, InsertOptions::default())
            .await?;
        Ok(())
    }
}