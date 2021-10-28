use std::fmt::Formatter;

pub mod db;
pub mod session;

pub type EngineError = Box<dyn std::error::Error + Sync + Send>;

//TODO: Rework error handling
#[derive(Debug)]
#[non_exhaustive]
pub enum DbError {
    NoHostProvided,
    BlankDatabaseName,
    InvalidIdentification,
    ParseFail,
    ItemNotFound,
    FailedToCreate,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            DbError::NoHostProvided => {
                write!(f, "{:?}: A host was not provided to the builder.", self)
            }
            DbError::BlankDatabaseName => write!(
                f,
                "{:?}, No Name of the Database was provided to the builder.",
                self
            ),
            DbError::InvalidIdentification => {
                write!(f, "{:?}", self)
            }
            DbError::ParseFail => {
                write!(f, "{:?}", self)
            }
            DbError::ItemNotFound => {
                write!(f, "Could not find item in the database.")
            }
            DbError::FailedToCreate => {
                write!(f, "Failed to create new item")
            }
        }
    }
}

impl std::error::Error for DbError {}

#[cfg(test)]
mod test {

    use crate::engine::{DbError, EngineError};

    #[test]
    fn test_error() {
        let err: std::result::Result<(), EngineError> = Err(DbError::NoHostProvided.into());
        if let Err(e) = err {
            println!("{}", e);
        }
    }
}
