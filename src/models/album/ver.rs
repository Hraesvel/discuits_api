use crate::models::album::Album;
use crate::macros::*;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AlbumVer {
    AlbumV1(Album)
}