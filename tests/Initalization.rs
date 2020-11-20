#[cfg(test)]
mod test {
    use std::error::Error;

    use discket_api::engine::{
        db::*,
        session::*,
    };

    type SimpleResult = Result<(), Box<dyn Error + Sync + Send>>;

    pub async fn setup_with_arangodb() -> Result<Session<Db>, Box<dyn Error + Sync + Send>>
    {
        let database = Db::new()
            .auth_type(AuthType::Jwt { user: "discket_test", pass: "" })
            .db_name("discket_test")
            .connect().await?;

        let session = Session::from(database)?;
        Ok(session)
    }

    #[tokio::test]
    async fn create_session_db_test() -> SimpleResult {
        let session = setup_with_arangodb().await?;
        {
            let sess_inst = session.clone();
            let engine = sess_inst.read().await;
            assert!(engine.validate_db().await.is_ok());
        }

        Ok(())
    }
}