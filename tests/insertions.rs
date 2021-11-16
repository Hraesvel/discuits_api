pub mod initialisation;

#[cfg(test)]
mod test_generics {
    use discuits_api::engine::db::DbBasics;
    use discuits_api::io::{read::*, Write};
    use discuits_api::models::BoxedDoc;
    use discuits_api::models::{album::*, artist::*, edge::*, DocDetails};
    use discuits_api::{insert_many, one_to_many};

    use crate::initialisation::test::*;

    #[tokio::test]
    async fn insert_multiple_types() -> SimpleResult {
        let session = with_arangodb().await?;
        let db = session.get_ref().db().read().await;

        let mut album = Album::new();
        let _artist = Artist::new();
        album.name("album test").description("insert made by test");
        // .change_id("12345");

        let mut artist = Artist::new();
        artist.name("artist test");
        // .change_id("12345");

        let resp = insert_many!(db, album, artist);
        dbg!(resp);
        Ok(())
    }

    #[tokio::test]
    async fn insert_artist_album_with_edge() -> SimpleResult {
        let session = with_arangodb().await?;
        let db = session.get_ref().db().read().await;

        let mut album = Album::new();
        album.name("owl house");
        let product = db.insert(album).await?;

        if let Ok(art) = db.find::<Artist>("Dana Terrace", "name").await {
            let resp = one_to_many!(db, "artist_to", art.id(), [product.0]);
            dbg!(&resp);
            assert!(!resp.iter().any(|r| matches!(r, Err(_))))
        } else {
            let mut artist = Artist::new();
            artist.name("Dana Terrace");
            let art = db.insert(artist).await?;
            let _e = Edge::link_one_to_many(&db, "artist_to", art.0, vec![product.0]).await?;
            // let edge = Edge::new("artist_to", art.0, product.0);
        };

        Ok(())
    }
}

#[cfg(test)]
mod test_mono {
    use discuits_api::engine::db::DbBasics;
    use futures::future::join_all;

    use discuits_api::io::write::Write;
    use discuits_api::models::{album::*, artist::*};

    use crate::initialisation::test::*;

    #[tokio::test]
    async fn insert_multiple_types() -> SimpleResult {
        let session = with_arangodb().await?;
        let db = session.get_ref().db().read().await;

        let mut album = Album::new();
        let _artist = Artist::new();
        album
            .name("album test")
            .description("insert made by test")
            .gen_id();

        let mut artist = Artist::new();
        artist.name("artist test").gen_id();

        let v = vec![db.insert(artist)];

        let r = join_all(v).await;
        dbg!(r);

        Ok(())
    }
}
