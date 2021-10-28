
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::engine::db::{Db, DbBasics};
use crate::io::*;
use crate::models::{BoxedDoc, ReqModelTraits};

#[derive(Debug)]
pub struct PostgresSQL;

impl PostgresSQL {
    pub fn db_info(&self) {
        println!("Postgres SQL database");
    }
}

#[async_trait]
impl EngineWrite for PostgresSQL {
    type E = ();

    async fn insert<T: ReqModelTraits + BoxedDoc + 'static>(
        &self,
        _doc: T,
    ) -> Result<(String, Box<dyn BoxedDoc>), Self::E> {
        todo!()
    }

    async fn update<T: ReqModelTraits>(&self, _doc: T) -> Result<(), Self::E> {
        todo!()
    }
}

#[cfg(feature = "pgsql")]
#[crate::async_trait]
impl<'a> DbBasics<'a> for Db<PostgresSQL> {
    type Client = &'a RwLock<PostgresSQL>;

    fn db(&'a self) -> Self::Client {
        &self.db
    }

    async fn db_info(&'a self) {
        todo!()
        // self.db.borrow().db_info()
    }
}
