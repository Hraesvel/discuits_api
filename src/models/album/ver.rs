use crate::macros::*;
use crate::models::album::Album;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AlbumVer {
    AlbumV1(Album),
}
