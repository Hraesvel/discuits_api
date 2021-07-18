use std::borrow::Cow;

#[warn(missing_docs)]
use arangors::{ClientError, Connection, Database};
use arangors::client::reqwest::ReqwestClient;
use serde::__private::Formatter;

use crate::engine::{DbError, EngineError};
use crate::models::ReqModelTraits;

pub(crate) mod arangodb;
pub mod ops;

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

#[derive(Debug)]
pub struct Db {
    conn: Connection,
    db: Database<ReqwestClient>,
}

impl Db {
    /// Creates a `DbBuilder` with a default host to `http://127.0.0.1:8529`
    /// host can be altered using the method `DbBuilder::host(&mut self, host: &'static str)`.
    pub fn new<'a>() -> DbBuilder<'a> {
        let mut builder = DbBuilder::default();
        builder.host = DEFAULT_HOST;
        builder
    }

    /// Check if a url to a server is still valid.
    pub async fn validate_server(&self) -> Result<(), ClientError> {
        Connection::validate_server(self.conn.url().as_str()).await
    }

    /// Checks if a `Connection` to server is still valid,
    /// Invalidation can happen if there is a server crashes or restarts while using a `JWT` as the
    /// authentication method.
    /// This method is intended to be used as a means to create an automations process for reconnecting.
    pub async fn validate_connection(&self) -> Result<(), EngineError> {
        let url = format!("{}/_db", self.db.url());
        let _ = self.conn.session().client.get(&url).send().await?;
        Ok(())
    }

    pub async fn validate_db(&self) -> Result<(), EngineError> {
        let db = format!(
            "http://{}:{}/_db/{}/_api/simple/any",
            self.db.url().host().unwrap(),
            self.db.url().port().unwrap(),
            self.db.name()
        );
        self.conn.session().client.put(&db).send().await?;
        Ok(())
    }

    /// JWT token can become invalid if the database is reset.
    /// This method attempts to reconnect (revalidate) the database, used existing information.
    pub async fn reconnect_jwt<'a>(
        &mut self,
        usr: &'a str,
        pass: &'a str,
    ) -> Result<(), EngineError> {
        let new_conn = Connection::establish_jwt(self.conn.url().as_str(), usr, pass).await?;

        self.db = new_conn.db(self.db.name()).await?;
        self.conn = new_conn;
        Ok(())
    }

    pub fn db(&self) -> &Database<ReqwestClient> {
        &self.db
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
    pub async fn connect(&mut self) -> Result<Db, EngineError> {
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

        let database: Db = Db {
            conn,
            db,
        };

        Ok(database)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use tokio;

    use crate::engine::db::*;
    use crate::engine::db::{AuthType, Db};
    use crate::engine::EngineError;
    use crate::engine::session::Session;
    use crate::engine::session::test::common_session_db;

    pub async fn common() -> Result<Db, EngineError> {
        let auth = AuthType::Basic {
            user: "discket",
            pass: "babyYoda",
        };
        let db = Db::new()
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

            *new_db = Db::new()
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
