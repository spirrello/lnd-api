pub mod lnd_grpc_rust;
pub mod setuplndclient;
use std::fs;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{
    delete, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    ResponseError,
};

use serde::{Deserialize, Serialize};
use setuplndclient::{ClientConfigBuilder, LndConn};

use actix_web::middleware::Logger;
use std::error;
use std::fmt::Display;
use std::sync::Mutex;

use std::rc::Rc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    let cc = ClientConfigBuilder::default()
        .set_cert("/Users/stefano/.polar/networks/1/volumes/lnd/alice/tls.cert").set_mac("/Users/stefano/.polar/networks/1/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon")
        .socket("127.0.0.1:10001").build().unwrap();

    let lnd_conn = LndConn::new(cc).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(lnd_conn.clone())
            .service(get_info)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[get("/getinfo")]
async fn get_info(lnd_conn: web::Data<&LndConn>) -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
