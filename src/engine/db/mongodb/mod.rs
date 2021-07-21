use async_trait::async_trait;

use crate::engine::db::{Db, DbBasics};
use crate::io::*;
use crate::models::{BoxedDoc, ReqModelTraits};

#[derive(Debug)]
pub struct MongoDb;

impl MongoDb {
    pub fn db_info(&self) {
        println!("MongoDb database");
    }
}

#[async_trait]
impl EngineWrite for MongoDb {
    type E = ();

    async fn insert<T: ReqModelTraits + BoxedDoc + 'static>(&self, doc: T) -> Result<(String, Box<dyn BoxedDoc>), Self::E> {
        todo!()
    }

    async fn update<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E> {
        todo!()
    }
}


#[cfg(feature = "mongodb")]
impl<'a> DbBasics<'a> for Db<MongoDb> {
    type Client = &'a MongoDb;

    fn db(&'a self) -> Self::Client {
        &self.db
    }

    fn db_info(&'a self) {
        self.db.db_info()
    }
}
