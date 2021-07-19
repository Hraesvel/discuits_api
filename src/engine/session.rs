use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::engine::db::arangodb::ArangoDb;
use crate::engine::EngineError;

pub trait NewSession {}

pub struct Session<T>(Arc<RwLock<T>>);

// #[async_trait]
// pub trait Engine {
//     async fn insert<T: ReqiredTraits>(&self, doc: T) -> Result<(), EngineError>;
// }

impl<T> Session<T> {
    pub fn from(t: T) -> Result<Session<T>, EngineError> {
        Ok(Session(Arc::new(RwLock::new(t))))
    }

    pub fn clone(&self) -> Arc<RwLock<T>> {
        self.0.clone()
    }
}

#[async_trait]
impl NewSession for ArangoDb {}

#[cfg(test)]
pub(crate) mod test {
    use crate::engine::db::arangodb::ArangoDb;
    use crate::engine::db::AuthType;
    use crate::engine::session::Session;
    use crate::engine::EngineError;

    pub async fn common_session_db() -> Result<Session<ArangoDb>, EngineError> {
        let db = ArangoDb::new()
            .db_name("discket_dev")
            .auth_type(AuthType::Jwt {
                user: "discket",
                pass: "babyYoda",
            })
            .connect()
            .await?;
        let session: Session<ArangoDb> = Session::from(db)?;

        Ok(session)
    }
}
