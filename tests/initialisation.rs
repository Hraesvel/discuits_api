use discuits_api::engine::{db::*, session::*};

#[cfg(test)]
pub mod test {
    use std::error::Error;
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use discuits_api::engine::{db::*, session::*};
    use arangors::Collection;
    use discuits_api::models::RequiredTraits;

    pub static mut SESS: Option<Session<Db>> = None;
    pub static mut COL: Option<Collections<Db, Collection<dyn RequiredTraits>>> = None;

    pub struct Collections<Engine, C> {
        session : Arc<RwLock<Engine>>,
        album: Collection<C>,
        artist: Collection<C>,
        inventory: Collection<C>,
        variant: Collection<C>,
    }

    pub type SimpleResult = Result<(), BoxedError>;
    type BoxedError = Box<dyn Error + Sync + Send>;

    /// Creates a server session using ArangoDb as the database
    pub async fn setup_with_arangodb() -> Result<Session<Db>, BoxedError> {
        let database = Db::new()
            .auth_type(AuthType::Jwt {
                user: "discket_test",
                pass: "",
            })
            .db_name("discket_test")
            .connect()
            .await?;


        let session = Session::from(database)?;
        Ok(session)
    }

    pub async fn with_arangodb() -> Result<Arc<RwLock<Db>>, BoxedError> {
       let result = unsafe {
            if let Some(s) = SESS.as_ref() {
                Ok(s.clone())
            } else {
                let session = setup_with_arangodb().await?;
                SESS = Some(session);
                Ok(SESS.as_ref().unwrap().clone())
            }
        };

        let s = result.clone()?;
        let read = s.read().await;

        let col = Collections {
            session : s,
            album: read.db().collection("album").await?,
            artist: read.db().collection("artist").await?,
            inventory: read.db().collection("inventory").await?,
            variant: read.db().collection("variant").await?
        };

        dbg!(col.album.document_count().await?.detail);



        result
    }


    #[tokio::test]
    async fn create_session_db_test() -> SimpleResult {
        let session = with_arangodb().await?;
        let engine = session.read().await;
        assert!(engine.validate_db().await.is_ok());
        Ok(())
    }
}
