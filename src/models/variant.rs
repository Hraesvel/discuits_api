use std::borrow::Cow;

use arangors::document::options::InsertOptions;
use async_trait::async_trait;
use uuid::Uuid;

use crate::engine::db::Db;
use crate::engine::EngineError;
use crate::io::write::Write;
use crate::models::{DocDetail, ReqModelTraits};
use crate::models::album::Album;
use crate::models::inventory::Inventory;

#[derive(Debug, Serialize, Deserialize)]
struct Variant {
    _id: Cow<'static, str>,
    _key: Cow<'static, str>,
    pub _from: Cow<'static, str>,
    pub _to: Cow<'static, str>,
    #[serde(default)]
    details: Cow<'static, str>,
    medium: Medium,
    quality: Quality,
    edition: Edition,
}

impl Default for Variant {
    fn default() -> Self {
        let uid = Uuid::new_v4().to_string()[0..8].to_string();

        Variant {
            _id: Default::default(),
            _key: Cow::from(uid),
            _from: Default::default(),
            _to: Default::default(),
            details: Default::default(),
            medium: Medium::Vinyl,
            quality: Quality::F,
            edition: Edition::Standard,
        }
    }
}

impl Variant {
    pub fn new() -> Self {
        let uid = Uuid::new_v4().to_string()[0..8].to_string();
        Variant {
            _key: Cow::from(uid),
            ..Variant::default()
        }
    }

    pub fn vertex<T: Into<Cow<'static, str>>>(&mut self, vtx: T) -> &mut Self {
        self._from = vtx.into();
        self
    }

    pub fn dest<T: Into<Cow<'static, str>>>(&mut self, dest: T) -> &mut Self {
        self._to = dest.into();
        self
    }

    pub fn details(&mut self, details: &'static str) -> &mut Self {
        self.details = Cow::from(details);
        self
    }

    pub fn medium(&mut self, medium: Medium) -> &mut Self {
        self.medium = medium;
        self
    }

    pub fn quality(&mut self, quality: Quality) -> &mut Self {
        self.quality = quality;
        self
    }

    pub fn edition(&mut self, edition: Edition) -> &mut Self {
        self.edition = edition;
        self
    }

    /// Use this to connect the edge to its vertex —`vtx`— and generate a new
    /// `Inventory` document to be the destination with the variant.
    pub fn create_inventory_variant(&mut self, vtx: &Album, count: u8, var: Variant) -> Result<Inventory, EngineError>
    {
        let mut inventory = Inventory::new();
        inventory.amount(count);
        self.vertex(vtx.id())
            .dest(inventory.id());
        Ok(inventory)
    }
}

impl DocDetail for Variant {
    fn collection_name<'a>() -> &'a str {
        "variant"
    }

    fn key(&self) -> String {
        self._key.to_string()
    }

    fn id(&self) -> String {
        format!("{}/{}", Self::collection_name(), self._key)
    }
}

impl ReqModelTraits for Variant {}

#[derive(Debug, Serialize, Deserialize)]
enum Medium {
    Vinyl,
    CD,
    Cassette,
}

/// Quality rating is based off of Discogs
/// https://support.discogs.com/hc/en-us/articles/360001566193-How-To-Grade-Items
#[derive(Debug, Serialize, Deserialize)]
enum Quality {
    F,
    G,
    GP,
    VG,
    VGP,
    NM,
    M,
}


#[derive(Debug, Serialize, Deserialize)]
enum Edition {
    Standard,
    Limited,
}

// #[async_trait]
// impl Write<Variant> for Db {
//     type E = EngineError;
//     type Document = Variant;
//
//     async fn insert(&self, doc: Variant) -> Result<(), Self::E> {
//         let io = InsertOptions::builder().overwrite(false).build();
//         let col = self.db().collection(Variant::collection_name()).await?;
//         col.create_document(doc, io).await?;
//         Ok(())
//     }
//
//     async fn update(&self) -> Result<(), Self::E> {
//         unimplemented!()
//     }
// }

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::engine::db::test::common;
    use crate::engine::EngineError;
    use crate::io::write::EngineWrite;
    use crate::models::variant::Variant;

    type TestResult = Result<(), EngineError>;

    #[tokio::test]
    async fn test_associate_variant() -> TestResult {
        let db = common().await?;
        let mut v = Variant::default();
        v._from = Cow::from("album/7782da0a");
        v._to = Cow::from("inventory/1158719");
        v.details = Cow::from("Test Variant");
        dbg!(db.insert(v).await)
    }
}