use std::borrow::{Borrow, Cow};

use crate::models::DocDetail;
use crate::models::RequiredTraits;

/// Artist Data type
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Artist {
    /// ArangonDb _id
    _id: Cow<'static, str>,
    /// ArangonDb _key
    _key: Cow<'static, str>,
    /// id/key used by source formatted `Source - ID`
    ///     # Example:
    ///     `discogs - 123456`
    foreign_key: Cow<'static, str>,
    /// Artist/Band name
    name: Cow<'static, str>,
    /// Common variations of the name
    aliases: Vec<Cow<'static, str>>,
    /// Description of artist.
    profile: Cow<'static, str>,
}

impl Artist {
    pub fn new() -> Self {
        use uuid::Uuid;

        let uid = Uuid::new_v4().to_string()[0..8].to_string();
        Self {
            _key: Cow::from(uid),
            ..Self::default()
        }
    }

    pub fn change_id(&mut self, new_id: String) {
        self._key = Cow::from(new_id);
    }
}

impl RequiredTraits for Artist {}

impl DocDetail for Artist {
    fn collection_name<'a>() -> &'a str {
        "artist"
    }

    fn key<'a>(&self) -> String {
        self._key.to_string()
    }
}

pub mod read {
    //! module for handling reads for artist
    use arangors::{AqlQuery, Cursor};
    use async_trait::async_trait;

    use crate::engine::db::arangodb::aql_snippet;
    use crate::engine::db::Db;
    use crate::engine::EngineError;
    use crate::io::read::Get;
    use crate::models::artist::Artist;
    use crate::models::DocDetail;

    #[async_trait]
    impl Get<Db> for Artist {
        type E = EngineError;
        type Document = Self;

        /// Gets all artists from storage `Db`
        async fn get_all(engine: &Db) -> Result<Vec<Self::Document>, Self::E>
            where
                Self: DocDetail,
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

        /// Gets a single artists from storage `Db`
        async fn get(id: &str, engine: &Db) -> Result<Self::Document, Self::E> {
            let col: Self = engine
                .db()
                .collection("artist")
                .await?
                .document(id)
                .await?
                .document;
            Ok(col)
        }
    }
}

pub mod write {
    //! module for handling writes for artist
    use arangors::document::options::InsertOptions;
    use async_trait::async_trait;

    use crate::engine::db::Db;
    use crate::engine::EngineError;
    use crate::io::write::Write;
    use crate::models::artist::Artist;
    use crate::models::DocDetail;

    #[async_trait]
    impl Write<Artist> for Db {
        type E = EngineError;
        type Document = Artist;

        async fn insert(&self, doc: Artist) -> Result<(), EngineError> {
            let io = InsertOptions::builder().overwrite(false).build();
            let col = self.db().collection(Artist::collection_name()).await?;
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
    use crate::engine::session::test::common_session_db;
    use crate::io::read::{EngineGet, Get};
    use crate::io::write::Write;
    use crate::models::artist::Artist;

    type TestResult = Result<(), EngineError>;

    #[tokio::test]
    async fn test_insert_artist_db() -> TestResult {
        let db = common().await?;
        let mut data = Artist::new();
        data.name = Cow::from("Disney");

        let resp = db.insert(data).await;

        assert!(resp.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn fail_on_overwrite_artist_db() -> TestResult {
        let db = common().await?;

        let mut data = Artist::new();
        data.name = Cow::from("Disney");

        db.insert(data.clone()).await?;
        let resp = db.insert(data).await;
        assert!(resp.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_artists() -> TestResult {
        let db = common().await?;

        let db_artist = db.get_all::<Artist>().await?;
        let artists = Artist::get_all(&db).await?;

        dbg!(db_artist.len());
        println!("><><><><>><><>><><><><><>><");
        dbg!(artists.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_session_insert_artist() -> TestResult {
        let s = common_session_db().await?.clone();
        let s_read = s.read().await;

        let mut a = Artist::new();
        a.name = Cow::from("with session");

        let resp = s_read.insert(a).await;

        // dbg!(&resp);

        assert!(resp.is_ok());

        Ok(())
    }
}
