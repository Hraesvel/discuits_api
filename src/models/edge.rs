use std::borrow::Cow;

use arangors::aql::AqlQuery;

use crate::engine::{DbError, EngineError};
use crate::engine::db::{ArangoDb};
use crate::io::Write;
use crate::models::{BoxedDoc, DocDetails, ReqModelTraits};

/// A module containing backend components
/// for handling ArangoDb edge collections
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Edge {
    #[serde(default)]
    edge_name: Cow<'static, str>,
    _id: Option<String>,
    _key: Option<String>,
    _from: Cow<'static, str>,
    _to: Cow<'static, str>,
}

impl DocDetails for Edge {
    fn collection_name<'a>() -> &'a str {
        "generic_edge"
    }

    fn key(&self) -> String {
        if let Some(k) = self._key.as_ref() {
            k.to_owned()
        } else {
            "NULL".to_string()
        }
    }

    fn id(&self) -> String {
        if let Some(k) = self._id.as_ref() {
            k.to_owned()
        } else {
            "NULL".to_string()
        }
    }
}

impl ReqModelTraits for Edge {}

impl BoxedDoc for Edge {}

impl Edge {
    pub fn new<T: Into<Cow<'static, str>>>(edge_name: &'static str, parent: T, child: T) -> Self {
        Edge {
            edge_name: edge_name.into(),
            _id: None,
            _key: None,
            _from: parent.into(),
            _to: child.into(),
        }
    }

    /// Method for linking many entities to one
    /// via arangodb edge
    /// this method doesn't check if the parent or children
    pub async fn link_one_to_many(
        engine: &ArangoDb,
        edge_name: &'static str,
        parent: String,
        children: Vec<String>,
    ) -> Result<Vec<Box<dyn BoxedDoc>>, EngineError> {
        let mut jobs = Vec::new();

        // create collection of futures
        for child in children {
            let edge = Edge::new(edge_name, parent.clone(), child);
            jobs.push(engine.insert(edge));
        }

        let out = futures::future::join_all(jobs).await;
        if out.is_empty() {
            return Err(Box::new(DbError::FailedToCreate));
        }

        let v  = out
            .into_iter()
            .filter_map(|o| o.ok() )
            .map(|(_s, d)| d)
            .collect();

        Ok(v)
    }
}

#[crate::async_trait]
impl Write<Edge> for ArangoDb {
    type E = EngineError;
    type Document = Edge;

    async fn insert(&self, doc: Edge) -> Result<(String, Box<dyn BoxedDoc>), Self::E> {
        use crate::engine::db::arangodb::aql_snippet::UPSERT_EDGE;
        #[derive(Serialize)]
        struct _Edge_ {
            _from: Cow<'static, str>,
            _to: Cow<'static, str>,
        }

        let edge = _Edge_ {
            _from: doc._from,
            _to: doc._to,
        };
        let value = serde_json::to_value(&edge).unwrap();
        let aql = AqlQuery::builder()
            .query(UPSERT_EDGE)
            .bind_var("doc", value)
            .bind_var("@collection", "artist_to")
            .build();

        let resp: Vec<Edge> = self.db.aql_query(aql).await?;
        let out = resp[0].clone();

        Ok((out._id.as_ref().unwrap().to_string(), Box::new(out)))
    }

    async fn update(&self, _doc: Edge) -> Result<(), Self::E> {
        todo!()
    }
}

#[macro_export]
macro_rules! one_to_many {
    ($db:expr, $edge_col:expr, $parent:expr, [$($child:expr)+]) => {{
        let mut jobs = Vec::new();
        $(
            let edge = Edge::new($edge_col, $parent, $child);
            jobs.push($db.insert(edge));
        )+
        futures::future::join_all(jobs).await
    }};
}
#[cfg(test)]
mod test {
    use crate::engine::db::arangodb::aql_snippet::FILTER;
    use crate::models::edge::*;

    #[test]
    fn test_attribute() {
        let aa = Edge::default();
        println!("Arist to Album: {:?}", aa);
        println!("aqul {}", FILTER);
    }
}
