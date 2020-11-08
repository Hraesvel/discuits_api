use std::borrow::Cow;

use arangors::{AqlQuery, Collection};
use arangors::client::reqwest::ReqwestClient;
use arangors::document::options::InsertOptions;
use async_trait::async_trait;

use crate::engine::db::{Db, DbActions};
use crate::engine::db::arangodb::aql_snippet;
use crate::engine::EngineError;
use crate::engine::file_system::{FileSystem, FsActions};
use crate::io::{read::Get, write::Write};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Album {
    id: Cow<'static, str>,
    key: Cow<'static, str>,
    barcode: Cow<'static, str>,
    cat_no: Cow<'static, str>,
    name: Cow<'static, str>,
    description: Cow<'static, str>,
}

impl Album {
    pub fn collection_name() -> &'static str {
        "album"
    }
}

pub mod read {
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
                .document(id).await?
                .document;
            Ok(col)
        }
    }

    #[async_trait]
    impl Get<FileSystem> for Album {
        type E = EngineError;
        type OUT = Self;

        async fn get_all(engine: FileSystem) -> Result<Vec<Self::OUT>, Self::E> {
            unimplemented!()
        }

        async fn get(id: &'static str, engine: FileSystem) -> Result<Self::OUT, Self::E> {
            unimplemented!()
        }
    }
}

pub mod write {
    use arangors::document::options::InsertOptions;

    use crate::engine::db::{Db, DbActions};
    use crate::engine::EngineError;
    use crate::models::album::Album;

    #[async_trait]
    impl DbActions<Album> for Db {
        async fn insert(&self, doc: Album) -> Result<(), EngineError> {
            let mut col = self.db().collection("album").await?;
            let doc = col
                .create_document(doc, InsertOptions::default())
                .await?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use arangors::client::reqwest::ReqwestClient;
    use arangors::Database;

    use crate::engine::db::{AuthType, Db, DbActions};
    use crate::engine::EngineError;
    use crate::io::read::Get;
    use crate::models::album::{Album, Fish};

    type TestResult = Result<(), EngineError>;

    async fn common() -> Result<Db, EngineError> {
        let auth = AuthType::Basic { user: "discket", pass: "babyYoda" };
        let db = Db::new()
            .auth_type(auth)
            .db_name("discket_dev")
            .connect().await?;

        Ok(db)
    }

    #[tokio::test]
    async fn test_album_db() -> TestResult
    {
        let db = common().await?;

        let new_album = Album {
            name: Cow::from("Owl House"),
            ..Album::default()
        };

        let a = db.insert(new_album).await;

        dbg!(&a);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_albums() -> TestResult {
        let db = common().await?;

        let album = Album::get_all(db).await?;

        dbg!(&album);

        Ok(())
    }
}