use crate::models::{BoxedDoc, ReqModelTraits};

#[crate::async_trait]
pub trait Write<T>
where
    T: ReqModelTraits + 'static,
{
    type E;
    type Document;

    async fn insert(&self, doc: T) -> Result<(String, Box<dyn BoxedDoc>), Self::E>;

    async fn update(&self, doc: T) -> Result<(), Self::E>;

    async fn insert_collection(
        &self,
        jobs: Vec<T>,
    ) -> Result<Vec<String>, Self::E> {
        let mut resp = Vec::new();
        for job in jobs {
            let r = self.insert(job).await?;
            resp.push(r.0);
        }
        Ok(resp)
    }
}

#[crate::async_trait]
pub trait EngineWrite {
    type E;

    /// Method to inserting a new document
    async fn insert<T: ReqModelTraits + BoxedDoc + 'static>(
        &self,
        doc: T,
    ) -> Result<(String, Box<dyn BoxedDoc>), Self::E>;

    /// Method to updating a single document
    async fn update<T: ReqModelTraits>(&self, doc: T) -> Result<(), Self::E>;

    async fn insert_collection<T: ReqModelTraits + BoxedDoc + 'static>(
        &self,
        jobs: Vec<T>,
    ) -> Result<Vec<String>, Self::E> {
        let mut resp = Vec::new();
        for job in jobs {
            let r = self.insert(job).await?;
            resp.push(r.0);
        }
        Ok(resp)
    }
}

#[macro_export]
macro_rules! insert_many {
    ($db:expr, $($e:expr),*) => {{

        let mut v: Vec<Result<Box<dyn BoxedDoc>, $crate::engine::EngineError>> = Vec::new();
        $(
            let r = $db.insert($e).await;
            match r {
                Ok(doc) => v.push(Ok(doc.1)),
                Err(e) => v.push(Err(Box::new($crate::engine::DbError::FailedToCreate))),
            }
        )*
        v
    }};
}
