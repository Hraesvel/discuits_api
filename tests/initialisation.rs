use discuits_api::engine::{db::*, session::*};

#[cfg(test)]
pub mod test {
    use std::error::Error;
    use std::sync::Arc;

    use arangors::{ClientError, Collection};
    use arangors::client::ClientExt;
    use arangors::client::reqwest::ReqwestClient;
    use tokio::sync::RwLock;

    use discuits_api::engine::{db::*, session::*};
    use arangors::collection::options::{CreateOptions, CreateParameters};

    pub static mut SESS: Option<Session<Db>> = None;

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
