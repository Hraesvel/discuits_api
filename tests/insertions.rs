pub mod initialisation;

#[cfg(test)]
mod test_generics {
    use std::process::Output;
    use std::sync::RwLockReadGuard;

    use tokio::macros::support::Future;

    use discuits_api::engine::db::arangodb::ArangoDb;
    use discuits_api::engine::session::Session;
    use discuits_api::engine::EngineError;
    use discuits_api::io::delete::{Delete, EngineDelete};
    use discuits_api::io::write::EngineWrite;
    use discuits_api::models::{album::*, artist::*, ReqModelTraits};

    use crate::initialisation::test::*;
    use serde_json::Value;

    #[tokio::test]
    async fn insert_multiple_types() -> SimpleResult {
        let session = with_arangodb().await?;
        let db = session.read().await;

        let mut album = Album::new();
        let mut artist = Artist::new();
        album
            .name("album test")
            .description("insert made by test")
            .change_id("12345");

        let mut artist = Artist::new();
        artist.name("artist test").change_id("12345");

        // let a = db.insert(album);
        // let b = db.insert(artist);
        let mut v = vec![];

        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));

        // tokio::join!(v);
        let resp = futures::future::join_all(v).await;
        let mut all_ids = used_ids.lock().await;
        // return Ok(());
        let mut ids = resp
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap().0)
            .collect::<Vec<String>>();

        let a = ids.swap_remove(0);

        let remove_album = db.remove::<Album>(&a).await?;
        dbg!(remove_album);

        for id in ids {
            let _ = db.remove::<Value>(&id).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_mono {
    use std::sync::RwLockReadGuard;

    use discuits_api::engine::db::arangodb::ArangoDb;
    use discuits_api::engine::session::Session;
    use discuits_api::io::write::Write;
    use discuits_api::models::{album::*, artist::*};

    use crate::initialisation::test::*;

    #[tokio::test]
    async fn insert_multiple_types() -> SimpleResult {
        let session = with_arangodb().await?;
        let db = session.read().await;

        let mut album = Album::new();
        let mut artist = Artist::new();
        album
            .name("album test")
            .description("insert made by test")
            .gen_id();

        let mut artist = Artist::new();
        artist.name("artist test").gen_id();

        let mut v = vec![];

        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));
        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));
        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));
        v.push(db.insert(album.clone().gen_id().to_owned()));
        v.push(db.insert(artist.clone().gen_id().to_owned()));

        let r = futures::future::join_all(v).await;
        dbg!(r);
        // let mut col_ids  = r.into_iter()
        //     .filter(|x| x.is_ok())
        //     .map(|x| x.unwrap())
        //     .collect::<Vec<_>>();
        // let mut ids = used_ids.lock().await;

        // dbg!(&col_ids);

        Ok(())
    }
}
