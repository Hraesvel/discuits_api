#[warn(missing_docs)]
use arangors::{ClientError, Connection, Database};
use arangors::client::reqwest::ReqwestClient;
use async_trait::async_trait;
use serde::export::Formatter;

use crate::engine::EngineError;
use crate::engine::session::Session;

pub(crate) mod arangodb;

const DEFAULT_HOST: &'static str = "http://127.0.0.1:8529";

#[derive(Debug)]
pub enum AuthType {
    Basic {
        user: &'static str,
        pass: &'static str,
    },
    Jwt {
        user: &'static str,
        pass: &'static str,
    },
}

#[derive(Debug)]
pub struct Db {
    conn: Connection,
    db: Database<ReqwestClient>,
}

#[async_trait]
pub trait DbActions<T>
    where
        T: 'static,
{
    // async fn get(&self, id: &'static str) -> Result<T, EngineError>;
    async fn insert(&self, doc: T) -> Result<(), EngineError>;
}

impl Db {
    /// Creates a `DbBuilder` with a default host to `http://127.0.0.1:8529`
    /// host can be altered using the method `DbBuilder::host(&mut self, host: &'static str)`.
    pub fn new() -> DbBuilder {
        let mut builder = DbBuilder::default();
        builder.host = DEFAULT_HOST;
        builder
    }

    /// Check if a url to a server is still valid.
    pub async fn validate_server(&self) -> Result<(), ClientError> {
        Connection::validate_server(self.conn.url().as_str()).await
    }

    /// Checks if a `Connection` go a server is still valid,
    /// An invalidation can happen if there is a server crashes or restarts while using a `JWT` as the
    /// authentication method.
    /// This method is intended to be used as a means to automate a reconnection
    pub async fn validate_connection(&self) -> Result<(), EngineError> {
        let _ = self
            .conn
            .session()
            .0
            .get("http://127.0.0.1:8529/_db")
            .send()
            .await?;
        Ok(())
    }

    pub async fn validate_db(&self) -> Result<(), EngineError> {
        let db = format!(
            "http://127.0.0.1:8529/_db/{}/_api/simple/any",
            self.db.name()
        );
        let resp = self.conn.session().0.put(&db).send().await?;
        dbg!(resp.status());
        Ok(())
    }

    pub async fn reconnect_jwt(
        &mut self,
        usr: &'static str,
        pass: &'static str,
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

#[derive(Debug, Default)]
pub struct DbBuilder {
    auth_type: Option<AuthType>,
    host: &'static str,
    db_name: &'static str,
}

impl DbBuilder {
    /// Method to altering the host address from `DEFAULT_HOST`
    pub fn host(&mut self, host: &'static str) -> &mut Self {
        self.host = host;
        self
    }

    pub fn auth_type(&mut self, auth: AuthType) -> &mut Self {
        self.auth_type = Some(auth);
        self
    }

    pub fn db_name(&mut self, db_nam: &'static str) -> &mut Self {
        self.db_name = db_nam;
        self
    }

    pub async fn connect(&mut self) -> Result<Db, EngineError> {
        if self.host.is_empty() {}
        let conn: Connection = match self.auth_type {
            None => Connection::establish_without_auth(self.host).await?,
            Some(AuthType::Basic { user, pass }) => {
                Connection::establish_basic_auth(self.host, user, pass).await?
            }
            Some(AuthType::Jwt { user, pass }) => {
                Connection::establish_jwt(self.host, user, pass).await?
            }
        };

        let db = conn.db(self.db_name).await?;

        let database: Db = Db { conn, db };

        Ok(database)
    }
}

#[derive(Debug)]
pub enum DbError {
    MissHost,
    MissDbName,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::MissHost => write!(f, "Empty host address was likely provided"),
            DbError::MissDbName => write!(f, "No database name was given"),
        }
    }
}

impl std::error::Error for DbError {}

#[cfg(test)]
mod test {
    use tokio;

    use crate::engine::db::*;
    use crate::engine::session::test::common_session_db;

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

        dbg!(db.read().await.db.name());

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
        dbg!(db.read().await.db.name());

        {
            let mut new_db = db.write().await;

            *new_db = Db::new().db_name("discket_test")
                .auth_type(AuthType::Jwt {
                    user: "discket_test",
                    pass: "",
                }).connect().await?;

            assert!(new_db.validate_server().await.is_ok());
            assert!(new_db.validate_connection().await.is_ok());
            assert!(new_db.validate_db().await.is_ok());
        }

        dbg!(db.read().await.db.name());
        dbg!(db2.read().await.db.name());

        Ok(())
    }
}
