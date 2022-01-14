use actix_web::web::{self, get, Data};
use actix_web::{App, HttpResponse, HttpServer};

// Todo: preludes module for discuits_api
use discuits_api::preludes::*;
use discuits_api::preludes::read::EngineGet;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session = config_database()
        .await
        .unwrap_or_else(|e| panic!("{:?}", e));
    let shared_data = session;
    let session = shared_data.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(session.clone())
            .service(web::scope("/app").route("", get().to(get_all_albums)))
    })
    .bind("127.0.0.1:8181")?
    .run()
    .await
}

async fn config_database() -> Result<Session<Db<ArangoDb>>, EngineError> {
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

async fn get_all_albums(data: Session<Db<ArangoDb>>) -> actix_web::Result<HttpResponse> {
    let db = data.db().read().await;
    let a = db.get_all::<Album>().await.map_err(|err| {
        if let Some(db_error) = err.downcast_ref::<DbError>() {
            match db_error {
                DbError::ItemNotFound => actix_web::error::ErrorNotFound("Not found"),
                _ => actix_web::error::ErrorInternalServerError("Whoops"),
            }
        } else {
            actix_web::error::ErrorInternalServerError("Whoops")
        }
    })?;

    Ok(HttpResponse::Ok().json(a))
}
