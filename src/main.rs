pub mod lnd;
pub mod setuplndclient;
use lnd::getinfo::{GetInfo, GetInfoBuilder};
use setuplndclient::NodeConfigurations;

use actix_web::{get, web, App, HttpResponse, HttpServer};

use actix_web::middleware::Logger;
use serde::Serialize;

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

    let get_info_response = client
        .lightning()
        .get_info(lnd_grpc_rust::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");
    let get_info_response_inner = get_info_response.into_inner();
    let get_info = GetInfoBuilder::default()
        .version(get_info_response_inner.version)
        .commit_hash(get_info_response_inner.commit_hash)
        .identity_pubkey(get_info_response_inner.identity_pubkey)
        .alias(get_info_response_inner.alias)
        .color(get_info_response_inner.color)
        .num_active_channels(get_info_response_inner.num_active_channels)
        .num_pending_channels(get_info_response_inner.num_pending_channels)
        .num_inactive_channels(get_info_response_inner.num_inactive_channels)
        .num_peers(get_info_response_inner.num_peers)
        .block_height(get_info_response_inner.block_height)
        .block_hash(get_info_response_inner.block_hash)
        .best_header_timestamp(get_info_response_inner.best_header_timestamp)
        .synced_to_chain(get_info_response_inner.synced_to_chain)
        .synced_to_graph(get_info_response_inner.synced_to_graph)
        .uris(get_info_response_inner.uris)
        .require_htlc_interceptor(get_info_response_inner.require_htlc_interceptor)
        .store_final_htlc_resolutions(get_info_response_inner.store_final_htlc_resolutions)
        .build()
        .unwrap();

    let response_json = &GetInfoHTTPResponse {
        status: "success".to_string(),
        message: get_info,
    };
    HttpResponse::Ok().json(response_json)
}

#[derive(Serialize)]
pub struct GetInfoHTTPResponse {
    pub status: String,
    pub message: GetInfo,
}
