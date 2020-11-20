use async_trait::async_trait;

use crate::engine::db::arangodb::aql_snippet;
use crate::engine::db::Db;
use crate::engine::EngineError;
use crate::io::read::EngineGet;

pub mod album;
pub mod artist;
pub mod inventory;

pub trait RequiredTraits: serde::de::DeserializeOwned + TypeCheck + Sync + Send {}

pub trait TypeCheck {
    fn collection_name<'a>() -> &'a str;
}

#[async_trait]
impl EngineGet for Db {
    type E = EngineError;

    async fn get_all<T>(&self) -> Result<Vec<T>, Self::E>
        where
            T: RequiredTraits,
    {
        use arangors::{AqlQuery, Cursor};

        let query = AqlQuery::builder()
            .query(aql_snippet::GET_ALL)
            .bind_var("@collection", T::collection_name())
            .batch_size(25)
            .build();

        let cursor: Cursor<T> = self.db().aql_query_batch(query).await?;
        let mut collection: Vec<T> = cursor.result;

        /// Collecting via pagination.
        if let Some(mut i) = cursor.id {
            while let Ok(c) = self.db().aql_next_batch(&i).await {
                let mut r: Vec<T> = c.result;
                collection.append(&mut r);
                if let Some(next_id) = c.id {
                    i = next_id;
                } else {
                    break;
                }
            }
        };

        Ok(collection)
    }

    async fn get<T>(&self, id: &str) -> Result<T, Self::E> {
        unimplemented!()
    }
}
