use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

use super::setuplndclient::NodeConnection;
use actix_web::{get, web, HttpResponse};

#[derive(Debug, Builder, Deserialize, Serialize)]
pub struct GetInfo {
    version: String,
    commit_hash: String,
    identity_pubkey: String,
    alias: String,
    color: String,
    num_pending_channels: u32,
    num_active_channels: u32,
    num_inactive_channels: u32,
    num_peers: u32,
    block_height: u32,
    block_hash: String,
    best_header_timestamp: i64,
    synced_to_chain: bool,
    synced_to_graph: bool,
    uris: Vec<String>,
    require_htlc_interceptor: bool,
    store_final_htlc_resolutions: bool,
}

#[derive(Serialize)]
pub struct GetInfoHTTPResponse {
    pub status: String,
    pub message: GetInfo,
}

#[get("/getinfo/{node}")]
pub async fn get_info(node_name: web::Path<String>) -> HttpResponse {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let get_info_response = node_connection
        .client
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
