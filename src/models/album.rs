use std::borrow::Cow;

use crate::models::{RequiredTraits, TypeCheck};

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

impl RequiredTraits for Album {}

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

    pub fn change_id(&mut self, new_id: String) {
        self._key = Cow::from(new_id);
    }
}

pub mod read {
    //! module for handling reads for album
    use arangors::{AqlQuery, Cursor};
    use async_trait::async_trait;

    use crate::engine::db::arangodb::aql_snippet;
    use crate::engine::db::Db;
    use crate::engine::EngineError;
    use crate::io::read::Get;
    use crate::models::{RequiredTraits, TypeCheck};
    use crate::models::album::Album;

    #[async_trait]
    impl Get<Db> for Album {
        type E = EngineError;
        type Element = Self;

        /// Gets all Albums from storage `Db`
        async fn get_all(engine: &Db) -> Result<Vec<Self::Element>, Self::E>
            where
                Self: RequiredTraits,
        {
            let query = AqlQuery::builder()
                .query(aql_snippet::GET_ALL)
                .bind_var("@collection", Self::collection_name())
                .batch_size(25)
                .build();

            let cursor: Cursor<Self> = engine.db().aql_query_batch(query).await?;
            let mut col: Vec<Self> = cursor.result;
            if let Some(mut i) = cursor.id {
                while let Ok(c) = engine.db().aql_next_batch(&i).await {
                    let mut r: Vec<Self> = c.result;
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

        /// Gets a single Albums from storage `Db`
        async fn get(id: &str, engine: &Db) -> Result<Self::Element, Self::E> {
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

pub mod write {
    //! module for handling writes for album
    use arangors::document::options::InsertOptions;
    use async_trait::async_trait;

    use crate::engine::db::Db;
    use crate::engine::EngineError;
    use crate::io::write::Write;
    use crate::models::album::Album;
    use crate::models::TypeCheck;

    #[async_trait]
    impl Write<Album> for Db {
        type E = EngineError;
        type Element = Album;

        async fn insert(&self, doc: Album) -> Result<(), EngineError> {
            let io = InsertOptions::builder().overwrite(false).build();
            let col = self.db().collection(Album::collection_name()).await?;
            let _doc = col.create_document(doc, io).await?;
            Ok(())
        }

        async fn update(&self) -> Result<(), Self::E> {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::engine::db::{AuthType, Db};
    use crate::engine::db::test::common;
    use crate::engine::EngineError;
    use crate::engine::session::test::{common, common_session_db};
    use crate::io::read::{EngineGet, Get};
    use crate::io::write::Write;
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

        let db_album = db.get_all::<Album>().await?;
        let albums = Album::get_all(&db).await?;

        dbg!(db_album);
        println!("><><><><><><><><><><><><");
        dbg!(albums);

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

impl TypeCheck for Album {
    /// Returns data type name used by DB.
    /// Helper function to avoid hard coding a collection's name in business logic code
    fn collection_name<'a>() -> &'a str {
        "album"
    }
}
