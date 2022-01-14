use std::ops::Deref;
use std::sync::Arc;

use crate::engine::db::Db;

pub trait NewSession {}

#[derive(Debug, Default)]
pub struct Session<T: ?Sized>(Arc<T>);

impl<T> Session<Db<T>> {
    pub fn new(t: T) -> Session<Db<T>> {
        Session(Arc::new(Db::new(t)))
    }
}

impl<T: ?Sized> Session<T> {
    /// Get reference to inner app Session.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Convert to the internal Arc<T>
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Session<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Session<T> {
    fn clone(&self) -> Session<T> {
        Session(self.0.clone())
    }
}

impl<T: ?Sized> From<Arc<T>> for Session<T> {
    fn from(arc: Arc<T>) -> Self {
        Session(arc)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::engine::db::arangodb::ArangoDb;
    use crate::engine::db::{AuthType, Db};
    use crate::engine::session::Session;
    use crate::engine::EngineError;

    pub async fn common_session_db() -> Result<Session<Db<ArangoDb>>, EngineError> {
        let auth = AuthType::Basic {
            user: "discuits_test",
            pass: "",
        };
        let db = ArangoDb::builder()
            .db_name("discuits_test")
            .auth_type(auth)
            .connect()
            .await?;
        let session: Session<Db<ArangoDb>> = Session::new(db);

        Ok(session)
    }
}
