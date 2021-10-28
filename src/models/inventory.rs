use std::borrow::Cow;

use uuid::Uuid;

use model_write_derive::*;

#[derive(Debug, Clone, ModelTrait, WriteToArango, Default, Deserialize, Serialize)]
pub struct Inventory {
    /// ArangonDb _id
    #[serde(rename(deserialize = "_id", serialize = "_id"))]
    id: Cow<'static, str>,
    /// ArangonDb _key
    #[serde(rename(deserialize = "_key", serialize = "_key"))]
    key: Cow<'static, str>,
    count: u8,
}

impl Inventory {
    pub fn new() -> Self {
        let uid = Uuid::new_v4().to_string()[0..8].to_string();
        Inventory {
            id: format!("{}/{}", Self::collection_name(), &uid).into(),
            key: Cow::from(uid),
            ..Inventory::default()
        }
    }

    pub fn amount(&mut self, count: u8) -> &mut Self {
        self.count = count;
        self
    }
}

