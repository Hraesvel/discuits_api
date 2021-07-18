pub mod initialisation;

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use tokio::io::AsyncReadExt;
    use tokio::sync::{RwLock, RwLockReadGuard};

    use discuits_api::engine::db::Db;
    use discuits_api::engine::session::Session;
    use discuits_api::io::read::EngineGet;
    use discuits_api::models::{album::*, artist::*};

    use crate::initialisation::test::*;
    use crate::test::SESS;

    #[tokio::test]
    async fn read_multiple_types() -> SimpleResult {
        let s = with_arangodb().await?;

        let reader = s.read().await;

        let boop = reader.get_all::<Album>().await?;
        let bop = reader.get_all::<Artist>().await?;
        let art: Vec<Artist> = reader.get_all().await?;

        dbg!(boop);
        dbg!(bop);
        dbg!(art);
        Ok(())
    }
}