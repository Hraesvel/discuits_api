#[cfg(test)]
pub mod test {
    use std::error::Error;

    use lazy_static::lazy_static;
    use tokio::sync::Mutex;

    use discuits_api::engine::db::arangodb::ArangoDb;
    use discuits_api::engine::{db::*, session::*};

    lazy_static! {
        pub static ref USED_IDS: Mutex<Vec<String>> = {
            let u = vec![];
            Mutex::new(u)
        };
    }

    pub static mut SESS: Option<Session<Db<ArangoDb>>> = None;

    pub type SimpleResult = Result<(), BoxedError>;
    type BoxedError = Box<dyn Error + Sync + Send>;

    /// Creates a server session using ArangoDb as the database
    pub async fn setup_with_arangodb() -> Result<Session<Db<ArangoDb>>, BoxedError> {
        let database = ArangoDb::builder()
            .auth_type(AuthType::Jwt {
                user: "discuits_test",
                pass: "",
            })
            .db_name("discuits_test")
            .connect()
            .await?;

        let session = Session::new(database);
        Ok(session)
    }

    pub async fn with_arangodb() -> Result<Session<Db<ArangoDb>>, BoxedError> {
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
        let engine = session.get_ref().db().read().await;
        assert!(engine.validate_db().await.is_ok());
        Ok(())
    }
}
