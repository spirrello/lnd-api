use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

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
