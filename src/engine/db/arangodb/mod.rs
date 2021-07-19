use arangors::client::reqwest::ReqwestClient;
use arangors::{ClientError, Connection, Database};

use crate::engine::db::{DbBuilder, DEFAULT_HOST};
use crate::engine::EngineError;

pub mod aql_snippet;
pub mod ops;

#[derive(Debug)]
pub struct ArangoDb {
    pub(crate) conn: Connection,
    pub(crate) db: Database<ReqwestClient>,
}

impl ArangoDb {
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
