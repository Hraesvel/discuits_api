use arangors::document::options::UpdateOptions;
use arangors::Cursor;

use serde::de::DeserializeOwned;

use crate::engine::db::arangodb::ArangoDb;
use crate::engine::{DbError, EngineError};
use crate::io::{delete::EngineDelete, read::EngineGet, write::EngineWrite};
use crate::models::{BoxedDoc, ReqModelTraits};

/// handles pagination
pub async fn cursor_digest<T: DeserializeOwned>(
    cursor: Cursor<T>,
    engine: &ArangoDb,
) -> Result<Vec<T>, EngineError> {
    let mut col: Vec<T> = cursor.result;
    if let Some(mut i) = cursor.id {
        while let Ok(c) = engine.db().aql_next_batch(&i).await {
            let mut r: Vec<T> = c.result;
            col.append(&mut r);
            if let Some(next_id) = c.id {
                i = next_id;
            } else {
                break;
            }
        }
    };

    Ok(col)
}

#[crate::async_trait]
impl EngineGet for ArangoDb {
    type E = EngineError;

    async fn get_all<T>(&self) -> Result<Vec<T>, Self::E>
    where
        T: ReqModelTraits,
    {
        use crate::engine::db::arangodb::aql_snippet;
        use arangors::{AqlQuery, Cursor};

        let query = AqlQuery::builder()
            .query(aql_snippet::GET_ALL)
            .bind_var("@collection", T::collection_name())
            .batch_size(5)
            .build();

        let cursor: Cursor<T> = self.db().aql_query_batch(query).await?;
        let collection = cursor_digest(cursor, self).await?;

        Ok(collection)
    }

    async fn get<T>(&self, _id: &str) -> Result<T, Self::E>
    where
        T: ReqModelTraits,
    {
        let col: T = self
    async fn find<T: ReqModelTraits>(&self, k: &str, v: &str) -> Result<T, Self::E> {
        let val = v.trim().to_ascii_lowercase();
    let resp : Option<T> = self
            .db()
            .aql_query(ArangoDb::aql_filter(k, &val, T::collection_name()))
            .await?
            .pop();
        if let Some(doc) = resp { Ok(doc) } else {
             DbError::ItemNotFound.into()
        }

    }
}

#[crate::async_trait]
impl EngineWrite for ArangoDb {
    type E = EngineError;

    async fn insert<T: ReqModelTraits + BoxedDoc + 'static>(
        &self,
        doc: T,
    ) -> Result<(String, Box<dyn BoxedDoc>), Self::E> {
        let resp: Vec<T> = self
            .db
            .aql_query(ArangoDb::insert(doc.clone(), T::collection_name()))
            .await?;
        if resp.is_empty() {
            return Err(Box::new(DbError::FailedToCreate));
        }
        let new_doc = resp[0].clone();

        Ok((new_doc.id(), Box::new(new_doc)))
    }

    async fn update<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E> {
        let col = self.db().collection(T::collection_name()).await?;
        let _updated_doc = col
            .update_document::<T>(&doc.key(), doc.clone(), UpdateOptions::default())
            .await?;
        Ok(())
    }
}

#[crate::async_trait]
impl EngineDelete for ArangoDb {
    type E = EngineError;

    async fn remove<T>(&self, id: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned + Send + Sync,
    {
        let parse = id.split('/').collect::<Vec<&str>>();
        let aql = ArangoDb::remove(parse[1], parse[0]);
        let mut value: Vec<T> = self.db.aql_query(aql).await?;
        if value.is_empty() {
            return DbError::ItemNotFound.into();
        }
        Ok(value.swap_remove(0))
    }
}
