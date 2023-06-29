use crate::lnd::{lnd_client::lnrpc::*, node_connect::NodeConnection};
use actix_web::web;
use serde_derive::Serialize;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Debug)]
pub struct ReturnHTTPResponse {
    pub message: ListPeersResponse,
}
impl Serialize for TimestampedError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut thing = serializer.serialize_struct("TimestampedError", 1)?;
        thing.serialize_field("timestamp", &self.timestamp)?;
        thing.serialize_field("error", &self.error)?;
        thing.end()
    }
}

impl Serialize for Peer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut thing = serializer.serialize_struct("Peer", 1)?;

        thing.serialize_field("address", &self.address)?;
        thing.serialize_field("bytes_recv", &self.bytes_recv)?;
        thing.serialize_field("bytes_sent", &self.bytes_sent)?;
        thing.serialize_field("errors", &self.errors)?;
        thing.serialize_field("features", &self.features)?;
        thing.serialize_field("flap_count", &self.flap_count)?;
        thing.serialize_field("inbound", &self.inbound)?;
        thing.serialize_field("last_flap_ns", &self.last_flap_ns)?;
        thing.serialize_field("last_ping_payload", &self.last_ping_payload)?;
        thing.serialize_field("pub_key", &self.pub_key)?;
        thing.serialize_field("sat_sent", &self.sat_sent)?;
        thing.serialize_field("sync_type", &self.sync_type)?;
        thing.serialize_field("sync_type", &self.sync_type)?;
        thing.end()
    }
}
impl Serialize for ListPeersResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("ListPeersResponse", 1)?;
        response.serialize_field("peers", &self.peers)?;
        response.end()
    }
}

pub async fn get_listpeers(node_name: web::Path<String>) -> web::Json<ReturnHTTPResponse> {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let latest_error = true;
    let lnd_response = node_connection
        .client
        .lightning()
        .list_peers(ListPeersRequest { latest_error })
        .await
        .expect("failed to get info");
    let lnd_response = lnd_response.into_inner();

    web::Json(ReturnHTTPResponse {
        message: lnd_response,
    })
}
