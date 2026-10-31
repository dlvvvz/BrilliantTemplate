mod core;
mod routes;

use crate::core::response::*;
use crate::core::fallback;
use actix_web::{web::{Data,self}, Responder, App as HttpApp, HttpServer, get};

struct App {

}

#[tokio::main]
async fn main() {
    HttpServer::new(|| {
        HttpApp::new()
            .service(index)
            .configure(fallback::config)
            .configure(routes::config)
            .app_data(Data::new(App {}))
    })
        .bind("127.0.0.1:8080").unwrap().run().await.unwrap();
}

#[get("/")]
async fn index() -> impl Responder {
    Response::new(code::Common::Ok)
}

