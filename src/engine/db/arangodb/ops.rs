use std::collections::HashMap;

use arangors::document::options::{InsertOptions, RemoveOptions, UpdateOptions};
use arangors::document::response::DocumentResponse;
use arangors::{AqlQuery, Cursor, Document};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::engine::db::arangodb::ArangoDb;
use crate::engine::{DbError, EngineError};
use crate::io::{delete::EngineDelete, read::EngineGet, write::EngineWrite};
use crate::models::{BoxedDoc, DocDetails, ReqModelTraits};

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

#[async_trait]
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

#[async_trait]
impl EngineWrite for ArangoDb {
    type E = EngineError;

    async fn insert<T: ReqModelTraits + BoxedDoc + 'static>(
        &self,
        doc: T,
    ) -> Result<(String, Box<dyn BoxedDoc>), Self::E> {
        let io = InsertOptions::builder().overwrite(false).build();
        let _col = self
            .db()
            .collection(T::collection_name())
            .await?
            .create_document::<T>(doc.clone(), io)
            .await?;
        Ok((doc.id(), Box::new(doc)))
    }

    async fn update<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E> {
        let col = self.db().collection(T::collection_name()).await?;
        let _updated_doc = col
            .update_document::<T>(&doc.key(), doc.clone(), UpdateOptions::default())
            .await?;
        Ok(())
    }
}

#[async_trait]
impl EngineDelete for ArangoDb {
    type E = EngineError;

    async fn remove<T>(&self, id: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned + Send + Sync,
    {
        let parse = id.split('/').collect::<Vec<&str>>();
        let query = format!(
            r#"REMOVE '{key}' in '{id}'
            let removed = OLD
            RETURN removed"#,
            id = parse[0],
            key = parse[1]
        );
        let value: Vec<T> = self.db.aql_str(&query).await?;
        value.into_iter().nth(0).ok_or(DbError::ParseFail.into())
    }
}
