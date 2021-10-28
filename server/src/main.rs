


use actix_web::{App, HttpServer, Responder};

use actix_web::web::{self, Data};

use discuits_api::engine::db::{ArangoDb, AuthType, Db, DbBasics, DbBuilder};
use discuits_api::engine::EngineError;
use discuits_api::engine::session::Session;
use discuits_api::io::Get;
use discuits_api::models::album::Album;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session = config_database_2()
        .await
        .unwrap_or_else(|e| panic!("{:?}", e));
    let shared_data = session;
    let session = shared_data.clone();
    HttpServer::new(move || {
        App::new()
            .data(session.clone())
            .service(web::scope("/app")
                .route("", web::get().to(hello)))
    })
        .bind("127.0.0.1:8181")?
        .run()
        .await
}

async fn config_database() -> Result<Db<ArangoDb>, EngineError> {
    let db = DbBuilder::new()
        .db_name("discket_test")
        .auth_type(AuthType::Jwt {
            user: "discket_test",
            pass: "",
        })
        .connect()
        .await?;
    Ok(Db::new(db))
}


async fn config_database_2() -> Result<Session<Db<ArangoDb>>, EngineError> {
    let db = DbBuilder::new()
        .db_name("discket_test")
        .auth_type(AuthType::Jwt {
            user: "discket_test",
            pass: "",
        })
        .connect()
        .await?;
    Ok(Session::new(db))
}

async fn hello(data: Data<Session<Db<ArangoDb>>>) -> impl Responder {
    let reader = data.db().read().await;
    let a = Album::get_all(&reader).await;
    format!("Using Database: {:?}", a)
}
