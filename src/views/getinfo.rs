use serde_derive::Serialize;
// use serde_json::{Result, Value};
use crate::lnd::lnd_client::lnrpc::GetInfoRequest;
use crate::lnd::{lnd_client::lnrpc::GetInfoResponse, node_connect::NodeConnection};
// use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse};

use serde::ser::{Serialize, SerializeStruct, Serializer};

// #[derive(Debug, Builder, Deserialize, Serialize)]
// pub struct GetInfo {
//     version: String,
//     commit_hash: String,
//     identity_pubkey: String,
//     alias: String,
//     color: String,
//     num_pending_channels: u32,
//     num_active_channels: u32,
//     num_inactive_channels: u32,
//     num_peers: u32,
//     block_height: u32,
//     block_hash: String,
//     best_header_timestamp: i64,
//     synced_to_chain: bool,
//     synced_to_graph: bool,
//     uris: Vec<String>,
//     require_htlc_interceptor: bool,
//     store_final_htlc_resolutions: bool,
// }

#[derive(Serialize)]
pub struct GetInfoHTTPResponse {
    pub message: GetInfoResponse,
}

impl Serialize for GetInfoResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("GetInfoResponse", 2)?;
        response.serialize_field("version", &self.version).unwrap();
        response
            .serialize_field("commit_hash", &self.commit_hash)
            .unwrap();
        response.end()
    }
}

// impl Responder for GetInfoHTTPResponse {
//     type Body = BoxBody;
//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap();
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }

// pub async fn get_node_info() -> impl Responder {
//     return getinfo();
// }

#[get("/getinfo/{node}")]
pub async fn get_info(node_name: web::Path<String>) -> HttpResponse {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let get_info_response = node_connection
        .client
        .lightning()
        .get_info(GetInfoRequest {})
        .await
        .expect("failed to get info");
    let get_info_response: GetInfoResponse = get_info_response.into_inner();

    let response_json = &GetInfoHTTPResponse {
        message: get_info_response,
    };
    HttpResponse::Ok().json(response_json)
}
