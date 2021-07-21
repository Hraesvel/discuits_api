use async_trait::async_trait;

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

    async fn insert<T: ReqModelTraits + BoxedDoc + 'static>(&self, doc: T) -> Result<(String, Box<dyn BoxedDoc>), Self::E> {
        todo!()
    }

    async fn update<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E> {
        todo!()
    }
}

#[cfg(feature = "pgsql")]
impl<'a> DbBasics<'a> for Db<PostgresSQL> {
    type Client = &'a PostgresSQL;

    fn db(&'a self) -> Self::Client {
        &self.db
    }

    fn db_info(&'a self) {
        self.db.db_info()
    }
}
