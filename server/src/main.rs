mod core;

use crate::core::response::*;
use crate::core::fallback::configure;
use actix_web::{web::{Data,self}, Responder, App as HttpApp, HttpServer, App, get};



#[tokio::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(configure)
    })
        .bind("127.0.0.1:8080").unwrap().run().await.unwrap();
}

#[get("/")]
async fn index() -> impl Responder {
    Response::new(code::Common::Ok)
}

