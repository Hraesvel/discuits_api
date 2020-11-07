use std::borrow::Cow;

use arangors::{Connection, Database};
use arangors::client::ClientExt;
use arangors::client::reqwest::ReqwestClient;

#[derive(Debug)]
pub(crate) struct Db {
    conn: Connection,
    db: Database<ReqwestClient>,
}

impl Db {
    pub fn new() -> DbBuilder
    {
        DbBuilder::default()
    }
}

#[derive(Debug, Default)]
pub(crate) struct DbBuilder {
    auth_type: Option<AuthType>,
    host: &'static str,
    db_name: &'static str,
}


impl DbBuilder {
    fn host(&mut self, host: &'static str) -> &mut Self {
        self.host = host;
        self
    }

    fn auth_type(&mut self, auth: AuthType) -> &mut Self {
        self.auth_type = Some(auth);
        self
    }

    fn db_name(&mut self, db_nam: &'static str) -> &mut Self {
        self.db_name = db_nam;
        self
    }

    async fn connect<C: ClientExt>(&mut self) -> Result<Db, Box<dyn std::error::Error + Send + Sync>> {
        let conn: Connection = match self.auth_type {
            None => {
                Connection::establish_without_auth(self.host).await?
            }
            Some(AuthType::Basic { user, pass }) => {
                Connection::establish_basic_auth(self.host, user, pass)
                    .await?
            }
            Some(AuthType::Jwt { user, pass }) => {
                Connection::establish_jwt(self.host, user, pass)
                    .await?
            }
        };

        let db = conn.db(self.db_name).await?;

        let database: Db = Db {
            conn,
            db,
        };


        Ok(database)
    }
}

#[derive(Debug)]
enum AuthType {
    Basic { user: &'static str, pass: &'static str },
    Jwt { user: &'static str, pass: &'static str },
}