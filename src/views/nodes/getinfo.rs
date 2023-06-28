use crate::lnd::lnd_client::lnrpc::{Chain, Feature, GetInfoRequest};
use crate::lnd::{lnd_client::lnrpc::GetInfoResponse, node_connect::NodeConnection};
use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::{body::BoxBody, web, HttpResponse, Responder};
use serde_derive::Serialize;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Debug)]
pub struct ReturnHTTPResponse {
    pub message: GetInfoResponse,
}
impl Serialize for Chain {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut thing = serializer.serialize_struct("Chain", 1)?;
        thing.serialize_field("chains", &self.chain)?;
        thing.end()
    }
}

impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut thing = serializer.serialize_struct("Feature", 1)?;
        thing.serialize_field("name", &self.name)?;
        thing.serialize_field("is_known", &self.is_known)?;
        thing.serialize_field("is_required", &self.is_required)?;

        thing.end()
    }
}
impl Serialize for GetInfoResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("GetInfoHTTPResponse", 1)?;
        response.serialize_field("version", &self.version).unwrap();
        response.serialize_field("alias", &self.alias).unwrap();
        response.serialize_field("block_hash", &self.block_hash)?;
        response.serialize_field("block_height", &self.block_height)?;
        response.serialize_field("best_header_timestamp", &self.best_header_timestamp)?;
        response.serialize_field("chains", &self.chains)?;
        response.serialize_field("color", &self.color)?;
        response.serialize_field("commit_hash", &self.commit_hash)?;
        response.serialize_field("identity_pubkey", &self.identity_pubkey)?;
        response.serialize_field("num_active_channels", &self.num_active_channels)?;
        response.serialize_field("num_inactive_channels", &self.num_inactive_channels)?;
        response.serialize_field("num_peers", &self.num_peers)?;
        response.serialize_field("features", &self.features)?;
        response.serialize_field("uris", &self.uris)?;
        response.serialize_field("require_htlc_interceptor", &self.require_htlc_interceptor)?;
        response.serialize_field(
            "store_final_htlc_resolutions",
            &self.store_final_htlc_resolutions,
        )?;
        response.end()
    }
}

impl Responder for ReturnHTTPResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub async fn getinfo(node_name: web::Path<String>) -> impl Responder {
    return get_lnd_info(node_name).await;
}

async fn get_lnd_info(node_name: web::Path<String>) -> ReturnHTTPResponse {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let lnd_response = node_connection
        .client
        .lightning()
        .get_info(GetInfoRequest {})
        .await
        .expect("failed to get info");
    let lnd_response: GetInfoResponse = lnd_response.into_inner();

    return ReturnHTTPResponse {
        message: lnd_response,
    };
}
