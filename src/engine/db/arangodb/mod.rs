
use arangors::{AqlQuery, ClientError, Connection, Database};
use arangors::uclient::reqwest::ReqwestClient;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::engine::db::{Db, DbBasics, DbBuilder, DEFAULT_HOST};
use crate::engine::db::arangodb::aql_snippet::*;
use crate::engine::EngineError;

pub mod aql_snippet;
pub mod ops;
pub mod preludes;


#[derive(Debug)]
pub struct ArangoDb {
    pub(crate) conn: Connection,
    pub(crate) db: Database<ReqwestClient>,
}

// Constructor
impl ArangoDb {
    /// Creates a `DbBuilder` with a default host to `http://127.0.0.1:8529`
    /// host can be altered using the method `DbBuilder::host(&mut self, host: &'static str)`.
    pub fn builder<'a>() -> DbBuilder<'a, Self> {
        let mut builder: DbBuilder<ArangoDb> = DbBuilder::new();
        builder.host = DEFAULT_HOST;
        builder
    }

    pub fn db_info(&self) {
        println!("ArangoDb database");
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
        let info = self.db.url();
        let db = format!(
            "http://{}:{}/_db/{}/_api/simple/any",
            info.host().unwrap(),
            info.port().unwrap(),
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

// methods
impl ArangoDb {
    pub fn filter<'a>(value: &'a str, field: &'a str, collection: &'a str) -> AqlQuery<'a> {
        AqlQuery::builder()
            .query(FILTER)
            .bind_var("field", field)
            .bind_var("value", value)
            .bind_var("@collection", collection)
            .build()
    }

    pub fn upsert<T: Clone + Serialize + 'static>(
        document: T,
        collection: &'static str,
    ) -> AqlQuery<'static> {
        AqlQuery::builder()
            .query(UPSERT_EDGE)
            .bind_var("@collection", collection)
            .bind_var("doc", serde_json::to_value(&document).unwrap())
            .build()
    }

    pub fn insert<T: Clone + Serialize + 'static>(
        document: T,
        collection: &'static str,
    ) -> AqlQuery<'static> {
        AqlQuery::builder()
            .query(INSERT)
            .bind_var("@collection", collection)
            .bind_var("doc", serde_json::to_value(&document).unwrap())
            .build()
    }

    pub fn remove<'a>(key: &'a str, collection: &'a str) -> AqlQuery<'static> {
        AqlQuery::builder()
            .query(REMOVE)
            .bind_var("@collection", collection)
            .bind_var("key", key)
            .build()
    }
}

#[crate::async_trait]
impl<'a> DbBasics<'a> for Db<ArangoDb> {
    type Client = &'a RwLock<ArangoDb>;

    fn db(&'a self) -> Self::Client {
        &self.db
    }

    async fn db_info(&'a self)   {
        self.db.read().await.db_info()
    }
}
