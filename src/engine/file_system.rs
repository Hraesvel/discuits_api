//! Todo : `FileSystem` engine will be placed on back burner till `Database` engine has been finished.
use std::borrow::Cow;
use std::path::Path;

use dashmap::DashMap;

/// File System for system without a database (ArangoDB) installed
pub struct FileSystem {
    /// root of shard location
    pub(crate) root: &'static Path,
    /// dir/file shards
    pub(crate) shards: DashMap<Cow<'static, str>, &'static Path>,
}

pub trait FsActions {}

impl FsActions for FileSystem {}