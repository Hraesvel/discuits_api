use discuits_api::engine::{db::*, session::*};

#[cfg(test)]
pub mod test {
    use std::error::Error;
    use std::sync::Arc;

    use arangors::client::reqwest::ReqwestClient;
    use arangors::client::ClientExt;
    use arangors::collection::options::{CreateOptions, CreateParameters};
    use arangors::{ClientError, Collection};
    use lazy_static::lazy_static;
    use tokio::sync::{Mutex, RwLock};

    use discuits_api::engine::db::arangodb::ArangoDb;
    use discuits_api::engine::{db::*, session::*};

    lazy_static! {
        pub static ref used_ids: Mutex<Vec<String>> = {
            let mut u = vec![];
            Mutex::new(u)
        };
    }

    pub static mut SESS: Option<Session<ArangoDb>> = None;

    pub type SimpleResult = Result<(), BoxedError>;
    type BoxedError = Box<dyn Error + Sync + Send>;

    /// Creates a server session using ArangoDb as the database
    pub async fn setup_with_arangodb() -> Result<Session<ArangoDb>, BoxedError> {
        let database = ArangoDb::new()
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

    pub async fn with_arangodb() -> Result<Arc<RwLock<ArangoDb>>, BoxedError> {
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
