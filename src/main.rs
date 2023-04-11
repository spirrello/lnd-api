pub mod setuplndclient;
use crate::lnd::getinfo::GetInfoResponse;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::{Map, Value};

use actix_web::middleware::Logger;
use serde::Serialize;
use setuplndclient::NodeConfigurations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || App::new().service(get_info).wrap(Logger::default()))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

fn get_node_configurations() -> NodeConfigurations {
    NodeConfigurations::new("nodeconfig.json")
}

#[get("/getinfo/{node}")]
async fn get_info(node_name: web::Path<String>) -> HttpResponse {
    const MESSAGE: &str = "get_info response";

    let node_configurations = get_node_configurations();
    let node_index = node_configurations.get_node_index(node_name.to_string());

    let mut client = lnd_grpc_rust::connect(
        node_configurations.nodes[node_index].cert.clone().unwrap(),
        node_configurations.nodes[node_index]
            .macaroon
            .clone()
            .unwrap(),
        node_configurations.nodes[node_index].socket.clone(),
    )
    .await
    .expect("failed to connect");

    let info = client
        .lightning()
        .get_info(lnd_grpc_rust::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    println!("{:#?}", info);

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
