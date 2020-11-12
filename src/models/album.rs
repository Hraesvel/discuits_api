use std::borrow::Cow;

use uuid;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Album {
    _id: Cow<'static, str>,
    _key: Cow<'static, str>,
    barcode: Cow<'static, str>,
    cat_no: Cow<'static, str>,
    name: Cow<'static, str>,
    description: Cow<'static, str>,
}

impl Album {
    pub fn new() -> Self {
        // let uid = UUID_INST.lock()
        //     .unwrap()
        //     .next()[..8].to_string();
        let uid = uuid::Uuid::new_v4().to_string()[0..8].to_string();
        // .next()[..8].to_string();
        Album {
            _key: Cow::from(uid),
            ..Album::default()
        }
    }

    pub fn collection_name() -> &'static str {
        "album"
    }
}

pub mod read {
    use arangors::AqlQuery;
    use async_trait::async_trait;

    use crate::engine::db::arangodb::aql_snippet;
    use crate::engine::db::Db;
    use crate::engine::EngineError;
    use crate::engine::file_system::FileSystem;
    use crate::io::read::Get;
    use crate::models::album::Album;

    #[async_trait]
    impl Get<Db> for Album {
        type E = EngineError;
        type OUT = Self;

        async fn get_all(engine: Db) -> Result<Vec<Self::OUT>, Self::E> {
            let query = AqlQuery::builder()
                .query(aql_snippet::GET_ALL)
                .bind_var("@collection", Album::collection_name())
                .build();

            // let col: Vec<Album> = engine.db().collection(Album::collection_name()).await?.document();
            let col = engine.db().aql_query(query).await?;
            Ok(col)
        }

        async fn get(id: &'static str, engine: Db) -> Result<Self::OUT, Self::E> {
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

    #[async_trait]
    impl Get<FileSystem> for Album {
        type E = EngineError;
        type OUT = Self;

        async fn get_all(_engine: FileSystem) -> Result<Vec<Self::OUT>, Self::E> {
            unimplemented!()
        }

        async fn get(_id: &'static str, _engine: FileSystem) -> Result<Self::OUT, Self::E> {
            unimplemented!()
        }
    }
}

pub mod write {
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

        db.insert(new_album.clone()).await;
        let resp = db.insert(new_album).await;
        dbg!(&resp);
        debug_assert!(resp.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_albums() -> TestResult {
        let db = common().await?;

        let album = Album::get_all(db).await?;

        dbg!(&album);

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
