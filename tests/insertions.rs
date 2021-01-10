
pub mod initialisation;

#[cfg(test)]
mod test {
    use discuits_api::engine::db::Db;
    use discuits_api::models::{album::*, artist::*};
    use crate::initialisation::test::*;
    use std::sync::RwLockReadGuard;
    use discuits_api::io::write::EngineWrite;
    use discuits_api::engine::session::Session;


    #[tokio::test]
    async fn insert_multiple_types() -> SimpleResult {
        let session  = with_arangodb().await?;
        let db = session.read().await;

        let mut album = Album::new();
        let mut artist = Artist::new();
        album.name("album test").description("insert made by test").change_id("12345");

        let mut artist = Artist::new();
        artist.name("artist test").change_id("12345");

        let a = db.insert(album);
        let b = db.insert(artist);

        tokio::join!(a,b);

        Ok(())
    }
}