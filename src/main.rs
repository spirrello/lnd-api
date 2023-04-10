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
use bb8::{ManageConnection, PooledConnection};

use crate::lnd_grpc_rust::LndClient;
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use setuplndclient::{ClientConfig, ClientConfigBuilder, LndConnectionManager};

use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    let socket = "localhost:10001".to_string();
    let cert = "/Users/stefano/.polar/networks/1/volumes/lnd/alice/tls.cert".to_string();
    let macaroon = "/Users/stefano/.polar/networks/1/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon".to_string();
    let cc = ClientConfigBuilder::default()
        .set_cert(&cert)
        .set_mac(&macaroon)
        .socket(socket)
        .build()
        .unwrap();

    let manager = LndConnectionManager::new(cc.clone()).await.unwrap();
    let pool = bb8::Pool::builder()
        .build(manager)
        .await
        .unwrap();

    let conn = pool.get().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(conn.clone())
            .service(get_info)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[get("/getinfo")]
async fn get_info(lnd_conn_pool: web::Data<LndConnectionManager>) -> impl Responder {
    const MESSAGE: &str = "get_info response";

    let info = lnd_conn_pool.
        .lightning()
        // All calls require at least empty parameter
        .get_info(lnd_grpc_rust::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    println!("{:?}", info);

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

// #[get("/getinfo")]
// async fn get_info() -> impl Responder {
//     // async fn get_info(cc: web::Data<ClientConfig>) -> impl Responder {
//     const MESSAGE: &str = "get_info response";

//     let socket = "localhost:10001".to_string();
//     let cert = "/Users/stefano/.polar/networks/1/volumes/lnd/alice/tls.cert".to_string();
//     let macaroon = "/Users/stefano/.polar/networks/1/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon".to_string();
//     let cc = ClientConfigBuilder::default()
//         .set_cert(&cert)
//         .set_mac(&macaroon)
//         .socket(socket)
//         .build()
//         .unwrap();
//     let mut client = lnd_grpc_rust::connect(cc.cert, cc.macaroon, cc.socket)
//         .await
//         .expect("failed to connect");

//     let info = client
//         .lightning()
//         // All calls require at least empty parameter
//         .get_info(lnd_grpc_rust::lnrpc::GetInfoRequest {})
//         .await
//         .expect("failed to get info");

//     println!("{:?}", info);

//     let response_json = &GenericResponse {
//         status: "success".to_string(),
//         message: MESSAGE.to_string(),
//     };
//     HttpResponse::Ok().json(response_json)
// }

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
