use serde_derive::Serialize;

#[derive(Debug, Deserialize)]
struct Response {
    metadata: MetadataMap,
    message: GetInfoResponse,
    extensions: Extensions,
}

#[derive(Debug, Deserialize)]
struct MetadataMap {
    headers: HeaderMap,
}

#[derive(Debug, Deserialize)]
struct HeaderMap {
    #[serde(rename = "content-type")]
    content_type: String,
    #[serde(rename = "grpc-status")]
    grpc_status: String,
    #[serde(rename = "grpc-message")]
    grpc_message: String,
}

#[derive(Debug, Deserialize)]
pub struct GetInfoResponse {
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
    best_header_timestamp: u64,
    synced_to_chain: bool,
    synced_to_graph: bool,
    testnet: bool,
    chains: Vec<Chain>,
    uris: Vec<String>,
    features: Vec<(u32, Feature)>,
    require_htlc_interceptor: bool,
    store_final_htlc_resolutions: bool,
}

#[derive(Debug, Deserialize)]
pub struct Chain {
    chain: String,
    network: String,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    name: String,
    is_required: bool,
    is_known: bool,
}

struct Extensions;
