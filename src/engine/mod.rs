use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;

use crate::engine;
use crate::engine::db::{AuthType, Db, DbActions};
use crate::engine::file_system::{FileSystem, FsActions};
use crate::io::read::Get;

pub mod file_system;
pub mod db;

pub(crate) type EngineError = Box<dyn std::error::Error + Sync + Send>;

pub trait NewSession {}

pub struct Session<T> {
    engine: Arc<T>
}

impl<T> Session<T>
{
    pub fn from(t: T) -> Result<Session<T>, EngineError> {
        Ok(Session { engine: Arc::new(t) })
    }
}


#[async_trait]
impl NewSession for Db {}


#[cfg(test)]
mod test {
    use crate::engine::{EngineError, Session};
    use crate::engine::db::{AuthType, Db};

    #[tokio::test]
    async fn test_session_db() -> Result<(), EngineError> {
        let db =
            Db::new()
                .db_name("discket_dev")
                .auth_type(AuthType::Jwt { user: "discket", pass: "babyYoda" })
                .connect().await?;
        let session: Session<Db> = Session::from(db)?;

        Ok(())
    }
}