use std::borrow::Cow;

use arangors::document::options::InsertOptions;
use async_trait::async_trait;
use uuid::Uuid;

use crate::engine::db::Db;
use crate::engine::EngineError;
use crate::io::write::Write;
use crate::models::DocDetail;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Inventory {
    _id: Cow<'static, str>,
    _key: Cow<'static, str>,
    count: u8,
}

impl Inventory {
    pub fn new() -> Self {
        let uid = Uuid::new_v4().to_string()[0..8].to_string();
        Inventory {
            _key: Cow::from(uid),
            ..Inventory::default()
        }
    }

    pub fn amount(&mut self, count: u8) -> &mut Self {
        self.count = count;
        self
    }
}

impl DocDetail for Inventory {
    /// Returns data type name used by DB.
    /// Helper function to  avoid hard coding a collection's name in business logic code
    fn collection_name<'a>() -> &'a str {
        "inventory"
    }

    fn key(&self) -> String {
        self._key.to_string()
    }

    fn id(&self) -> String {
        format!("{}/{}", Self::collection_name(), self._key)
    }
}

#[async_trait]
impl Write<Inventory> for Db {
    type E = EngineError;
    type Document = Inventory;

    async fn insert(&self, doc: Inventory) -> Result<(), EngineError> {
        let col = self.db().collection("inventory").await?;
        col.create_document(doc, InsertOptions::default()).await?;
        Ok(())
    }

    async fn update(&self) -> Result<(), Self::E> {
        unimplemented!()
    }
}
