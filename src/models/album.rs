use std::borrow::{Borrow, Cow};

use model_write_derive::*;

/// Album data type
#[derive(Debug, ModelTrait, WriteToArango, Default, Clone, Deserialize, Serialize)]
pub struct Album {
    /// field that reflects ArangoDB's `_id`
    _id: Cow<'static, str>,
    /// field that reflects ArangoDB's `_key`
    _key: Cow<'static, str>,
    /// field for storing an barcode of a album
    barcode: Cow<'static, str>,
    /// field for storing an catalog number of a album
    cat_no: Cow<'static, str>,
    /// Albums name
    name: Cow<'static, str>,
    /// Album details
    description: Cow<'static, str>,
}

impl Album {
    /// Creates a new blank Album with a unique identifier for `_key`
    pub fn new() -> Self {
        use uuid::Uuid;

        let uid = Uuid::new_v4().to_string()[0..8].to_string();
        Album {
            _key: Cow::from(uid),
            ..Album::default()
        }
    }

    pub fn change_id<T>(&mut self, new_id: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self._key = new_id.into();
        self
    }

    pub fn gen_id(&mut self) -> &mut Self {
        use uuid::Uuid;
        let uid = Uuid::new_v4().to_string()[0..8].to_string();
        self.change_id(uid);
        self
    }

    pub fn name(&mut self, name: &'static str) -> &mut Self {
        self.name = Cow::from(name);
        self
    }

    pub fn description(&mut self, desc: &'static str) -> &mut Self {
        self.description = Cow::from(desc);
        self
    }
}

pub mod read {
    //! module for handling reads for album
    use arangors::{AqlQuery, Cursor};
    use async_trait::async_trait;

    use crate::engine::db::arangodb::aql_snippet;
    use crate::engine::db::arangodb::ops::cursor_digest;
    use crate::engine::db::arangodb::ArangoDb;
    use crate::engine::EngineError;
    use crate::io::read::Get;
    use crate::models::{album::Album, DocDetails, ReqModelTraits};

    #[async_trait]
    impl Get<ArangoDb> for Album {
        type E = EngineError;
        type Document = Self;

        /// Gets all Albums from storage `Db`
        async fn get_all(engine: &ArangoDb) -> Result<Vec<Self::Document>, Self::E>
        where
            Self: ReqModelTraits,
        {
            let query = AqlQuery::builder()
                .query(aql_snippet::GET_ALL)
                .bind_var("@collection", Self::collection_name())
                .batch_size(25)
                .build();

            let cursor: Cursor<Self> = engine.db().aql_query_batch(query).await?;
            let col: Vec<Self> = cursor_digest(cursor, engine).await?;
            // let mut col: Vec<Self> = cursor.result;
            // if let Some(mut i) = cursor.id {
            //     while let Ok(c) = engine.db().aql_next_batch(&i).await {
            //         let mut r: Vec<Self> = c.result;
            //         col.append(&mut r);
            //         if let Some(next_id) = c.id {
            //             i = next_id;
            //         } else {
            //             break;
            //         }
            //     }
            // };

            Ok(col)
        }

        /// Gets a single Albums from storage `Db`
        async fn get(id: &str, engine: &ArangoDb) -> Result<Self::Document, Self::E> {
            let col: Self = engine
                .db()
                .collection("album")
                .await?
                .document(id)
                .await?
                .document;
            Ok(col)
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use tokio::time::{delay_for, Duration};

    use crate::engine::db::arangodb::ArangoDb;
    use crate::engine::db::test::common;
    use crate::engine::db::AuthType;
    use crate::engine::session::test::common_session_db;
    use crate::engine::EngineError;
    use crate::io::read::{EngineGet, Get};
    use crate::io::write::EngineWrite;
    use crate::models::album::Album;

    type TestResult = Result<(), EngineError>;

    #[tokio::test]
    async fn test_insert_album_db() -> TestResult {
        let db = common().await?;
        let mut new_album = Album::new();
        new_album.name = Cow::from("Owl House");

        let resp = db.insert(new_album).await;
        assert!(resp.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn fail_on_overwrite_album_db() -> TestResult {
        let db = common().await?;

        let mut new_album = Album::new();
        new_album.name = Cow::from("Owl House");

        db.insert(new_album.clone()).await?;
        let resp = db.insert(new_album).await;
        assert!(resp.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_albums() -> TestResult {
        let db = common().await?;

        delay_for(Duration::from_secs(2)).await;
        // Two flavors of get all either from Db type or using the Model type
        // and providing the Db
        let engine_read_trait = db.get_all::<Album>().await?;
        let implicit_get_from_db = Album::get_all(&db).await?;

        assert_eq!(engine_read_trait.len(), implicit_get_from_db.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_session_insert_album() -> TestResult {
        let s = common_session_db().await?.clone();
        let s_read = s.read().await;

        let mut a = Album::new();
        a.name = Cow::from("with session");

        let resp = s_read.insert(a).await;

        assert!(resp.is_ok());

        Ok(())
    }
}
