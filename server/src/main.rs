pub mod core;
mod routes;

use crate::core::response::*;
use crate::core::fallback;
use actix_web::{web::{Data}, Responder, App as HttpApp, HttpServer, get};
use crate::core::database::Database;


#[derive(Clone)]
struct App {
    database: Database,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = core::config::Config::load();
    let state = Data::new(App { database: Database::new(config.database_url.as_str()).await });
    HttpServer::new(move || {
        HttpApp::new()
            .service(index)
            .configure(fallback::config)
            .configure(routes::config)
            .app_data(state.clone())
    })
        .bind(config.bind).expect("Can not bind")
        .run().await.expect("Can not run server");
}

#[get("/")]
async fn index() -> impl Responder {
    Response::new(code::Common::Ok)
}

