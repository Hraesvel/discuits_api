pub mod initialisation;

#[cfg(test)]
mod test {
    use discuits_api::engine::db::DbBasics;
    use discuits_api::io::read::EngineGet;
    use discuits_api::models::{album::*, artist::*};

    use crate::initialisation::test::*;

    #[tokio::test]
    async fn read_multiple_types() -> SimpleResult {
        let s = with_arangodb().await?;

        let db = s.get_ref().db().read().await;

        let boop = db.get_all::<Album>().await?;
        let bop = db.get_all::<Artist>().await?;
        let art: Vec<Artist> = db.get_all().await?;

        dbg!(boop);
        dbg!(bop);
        dbg!(art);
        Ok(())
    }
}
