use std::borrow::Cow;

use uuid;

/// Album data type
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
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
        let uid = uuid::Uuid::new_v4().to_string()[0..8].to_string();
        Album {
            _key: Cow::from(uid),
            ..Album::default()
        }
    }

    /// Returns data type name used by DB.
    /// Helper function to avoid hard coding a collection's name in business logic code
    pub fn collection_name() -> &'static str { "album" }
}

pub mod read {
    //! module for handling reads for album
    use arangors::{AqlQuery, Cursor};
    use async_trait::async_trait;

    use crate::engine::db::arangodb::aql_snippet;
    use crate::engine::db::Db;
    use crate::engine::EngineError;
    use crate::io::read::Get;
    use crate::models::album::Album;

    #[async_trait]
    impl Get<Db> for Album {
        type E = EngineError;
        type Element = Self;

        /// Gets all Albums from storage `Db`
        // Todo: pagination
        async fn get_all(engine: Db) -> Result<Vec<Self::Element>, Self::E> {
            let query = AqlQuery::builder()
                .query(aql_snippet::GET_ALL)
                .bind_var("@collection", Album::collection_name())
                .batch_size(25)
                .build();

            let cursor: Cursor<Album> = engine.db().aql_query_batch(query).await?;
            let mut col: Vec<Album> = cursor.result;
            if let Some(mut i) = cursor.id {
                while let Ok(c) = engine.db().aql_next_batch(&i).await {
                    let mut r: Vec<Album> = c.result;
                    col.append(&mut r);
                    if let Some(next_id) = c.id {
                        i = next_id;
                    } else { break; }
                };
            };

            Ok(col)
        }

        /// Gets a single Albums from storage `Db`
        async fn get(id: &'static str, engine: Db) -> Result<Self::Element, Self::E> {
            let col: Album = engine
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

pub mod write {
    //! module for handling writes for album
    use arangors::document::options::InsertOptions;
    use async_trait::async_trait;

    use crate::engine::db::{Db, DbActions};
    use crate::engine::EngineError;
    use crate::models::album::Album;

    #[async_trait]
    impl DbActions<Album> for Db {
        async fn insert(&self, doc: Album) -> Result<(), EngineError> {
            let io = InsertOptions::builder().overwrite(false).build();
            let col = self.db().collection("album").await?;
            let _doc = col.create_document(doc, io).await?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::engine::db::{AuthType, Db, DbActions};
    use crate::engine::EngineError;
    use crate::engine::session::test::common_session_db;
    use crate::io::read::Get;
    use crate::models::album::Album;

    type TestResult = Result<(), EngineError>;

    async fn common() -> Result<Db, EngineError> {
        let auth = AuthType::Basic {
            user: "discket",
            pass: "babyYoda",
        };
        let db = Db::new()
            .auth_type(auth)
            .db_name("discket_dev")
            .connect()
            .await?;

        Ok(db)
    }

    #[tokio::test]
    async fn test_insert_album_db() -> TestResult {
        let db = common().await?;

        let mut new_album = Album::new();
        new_album.name = Cow::from("Owl House");

        let resp = db.insert(new_album).await;
        dbg!(&resp);

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
        dbg!(&resp);
        debug_assert!(resp.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_albums() -> TestResult {
        let db = common().await?;

        let album = Album::get_all(db).await?;

        dbg!(&album.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_session_insert_album() -> TestResult {
        let s = common_session_db().await?.clone();
        let s_read = s.read().await;

        let mut a = Album::new();
        a.name = Cow::from("with session");

        let resp = s_read.insert(a).await;

        dbg!(&resp);

        assert!(resp.is_ok());

        Ok(())
    }
}
