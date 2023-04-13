use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

use super::super::httpresponse::LNDHTTPResponse;
use super::super::setuplndclient::NodeConnection;
use actix_web::{get, web, HttpResponse};

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPeers {
    pub peers: Vec<Peer>,
}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    pub pub_key: String,
    pub address: String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub sat_sent: i64,
    pub sat_recv: i64,
    pub inbound: bool,
    pub ping_time: i64,
    pub sync_type: i32,
    // NEED TO WORK ON THESE
    // pub features: Option<Features>,
    // pub errors: Vec<Value>,
    pub flap_count: i32,
    pub last_flap_ns: i64,
    pub last_ping_payload: Vec<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "0")]
    pub n0: N0,
    #[serde(rename = "5")]
    pub n5: N5,
    #[serde(rename = "7")]
    pub n7: N7,
    #[serde(rename = "9")]
    pub n9: N9,
    #[serde(rename = "12")]
    pub n12: N12,
    #[serde(rename = "14")]
    pub n14: N14,
    #[serde(rename = "17")]
    pub n17: N17,
    #[serde(rename = "23")]
    pub n23: N23,
    #[serde(rename = "27")]
    pub n27: N27,
    #[serde(rename = "31")]
    pub n31: N31,
    #[serde(rename = "45")]
    pub n45: N45,
    #[serde(rename = "2023")]
    pub n2023: N2023,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N0 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N5 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N7 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N9 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N12 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N14 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N17 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N23 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N27 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N31 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N45 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N2023 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[get("/listpeers/{node}")]
pub async fn list_peers(node_name: web::Path<String>) -> HttpResponse {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let latest_error = true;
    let list_peers_response = node_connection
        .client
        .lightning()
        .list_peers(lnd_grpc_rust::lnrpc::ListPeersRequest { latest_error })
        .await
        .expect("failed to get info");

    let list_peers_response = list_peers_response.into_inner();

    let mut list_peers = ListPeers::default();

    for peer in list_peers_response.peers.iter() {
        let p = PeerBuilder::default()
            .pub_key(peer.pub_key.clone())
            .address(peer.address.clone())
            .bytes_sent(peer.bytes_sent)
            .bytes_recv(peer.bytes_recv)
            .sat_recv(peer.sat_recv)
            .sat_sent(peer.sat_sent)
            .inbound(peer.inbound)
            .ping_time(peer.ping_time)
            .sync_type(peer.sync_type)
            .flap_count(peer.flap_count)
            .last_flap_ns(peer.last_flap_ns)
            .last_ping_payload(peer.last_ping_payload.clone())
            .build()
            .unwrap();

        list_peers.peers.push(p)
    }

    let response_json = &LNDHTTPResponse {
        status: "success".to_string(),
        message: list_peers,
    };
    HttpResponse::Ok().json(response_json)
}
