use std::borrow::Cow;

use arangors::client::reqwest::ReqwestClient;
#[warn(missing_docs)]
use arangors::{ClientError, Connection, Database};
use serde::__private::Formatter;

use crate::engine::{DbError, EngineError};
use crate::models::ReqModelTraits;

pub mod arangodb;

pub use arangodb::ArangoDb;

// Temporary host address - ArangoDB default
const DEFAULT_HOST: &'static str = "http://127.0.0.1:8529";

#[derive(Debug)]
pub enum AuthType<'a> {
    NoAuth,
    Basic { user: &'a str, pass: &'a str },
    Jwt { user: &'a str, pass: &'a str },
}

impl<'a> Default for AuthType<'a> {
    fn default() -> Self {
        AuthType::NoAuth
    }
}

pub struct Db<T>(T);

impl Db<ArangoDb> {
    pub fn db(&self) -> &Database<ReqwestClient> {
        &self.0.db
    }
}

/// Builder for `Db` (database) struct
#[derive(Debug, Default)]
pub struct DbBuilder<'a> {
    auth_type: AuthType<'a>,
    host: &'a str,
    db_name: &'a str,
}

impl<'a> DbBuilder<'a> {
    /// Method to altering the host address from `DEFAULT_HOST`
    pub fn host(&mut self, host: &'a str) -> &mut Self {
        self.host = host;
        self
    }

    /// Configure authentication type
    pub fn auth_type(&mut self, auth: AuthType<'a>) -> &mut Self {
        self.auth_type = auth;
        self
    }

    /// Configure the database name
    pub fn db_name(&mut self, db_nam: &'a str) -> &mut Self {
        self.db_name = db_nam;
        self
    }

    /// Attempt to connect to the Db
    pub async fn connect(&mut self) -> Result<ArangoDb, EngineError> {
        if self.host.is_empty() {
            return Err(DbError::NoHostProvided.into());
        } else if self.db_name.is_empty() {
            return Err(DbError::BlankDatabaseName.into());
        }
        let conn: Connection = match self.auth_type {
            AuthType::NoAuth => Connection::establish_without_auth(self.host).await?,
            AuthType::Basic { user, pass } => {
                Connection::establish_basic_auth(self.host, user, pass).await?
            }
            AuthType::Jwt { user, pass } => {
                Connection::establish_jwt(self.host, user, pass).await?
            }
        };

        let db = conn.db(self.db_name).await?;

        let database: ArangoDb = ArangoDb { conn, db };

        Ok(database)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use tokio;

    use crate::engine::db::arangodb::ArangoDb;
    use crate::engine::db::AuthType;
    use crate::engine::db::*;
    use crate::engine::session::test::common_session_db;
    use crate::engine::session::Session;
    use crate::engine::EngineError;

    pub async fn common() -> Result<ArangoDb, EngineError> {
        let auth = AuthType::Basic {
            user: "discket",
            pass: "babyYoda",
        };
        let db = ArangoDb::new()
            .auth_type(auth)
            .db_name("discket_dev")
            .connect()
            .await?;

        Ok(db)
    }

    #[tokio::test]
    async fn test_connection() -> Result<(), EngineError> {
        let db = common_session_db().await?.clone();
        {
            let d = db.read().await;
            assert!(d.validate_connection().await.is_ok());
            assert!(d.validate_server().await.is_ok());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_reconnect_jwt() -> Result<(), EngineError> {
        let db_session = common_session_db().await?;
        let db = db_session.clone();
        {
            let mut new_db = db.write().await;

            assert!(new_db.reconnect_jwt("discket", "babyYoda").await.is_ok());

            assert!(new_db.validate_server().await.is_ok());
            assert!(new_db.validate_connection().await.is_ok());
            assert!(new_db.validate_db().await.is_ok());
        }

        // dbg!(db.read().await.db.name());

        Ok(())
    }

    #[tokio::test]
    async fn test_modify_session() -> Result<(), EngineError> {
        let db_session = common_session_db().await?;

        let db = db_session.clone();
        let db2 = db_session.clone();
        {
            let d = db.read().await;
            assert!(d.validate_connection().await.is_ok());
            assert!(d.validate_server().await.is_ok());
        }
        // dbg!(db.read().await.db.name());

        {
            let mut new_db = db.write().await;

            *new_db = ArangoDb::new()
                .db_name("discket_test")
                .auth_type(AuthType::Jwt {
                    user: "discket_test",
                    pass: "",
                })
                .connect()
                .await?;

            assert!(new_db.validate_server().await.is_ok());
            assert!(new_db.validate_connection().await.is_ok());
            assert!(new_db.validate_db().await.is_ok());
        }

        dbg!(db.read().await.db.name());
        dbg!(db2.read().await.db.name());

        Ok(())
    }
}
