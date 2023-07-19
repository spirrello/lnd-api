use crate::lnd::lnd_client::lnrpc::*;
use crate::lnd::node_connect::NodeConnection;
use actix_web::web;
use serde_derive::Serialize;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Debug)]
pub struct ReturnHTTPResponse {
    pub message: ChannelGraph,
}

impl Serialize for RoutingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("RoutingPolicy", 1)?;
        response.serialize_field("custom_records", &self.custom_records)?;
        response.serialize_field("disabled", &self.disabled)?;
        response.serialize_field("fee_base_msat", &self.fee_base_msat)?;
        response.serialize_field("fee_rate_milli_msat", &self.fee_rate_milli_msat)?;
        response.serialize_field("last_update", &self.last_update)?;
        response.serialize_field("max_htlc_msat", &self.max_htlc_msat)?;
        response.serialize_field("min_htlc", &self.min_htlc)?;
        response.serialize_field("time_lock_delta", &self.time_lock_delta)?;

        response.end()
    }
}
impl Serialize for ChannelEdge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("ChannelEdge", 1)?;
        response.serialize_field("capacity", &self.capacity)?;
        response.serialize_field("chan_point", &self.chan_point)?;
        response.serialize_field("channel_id", &self.channel_id)?;
        response.serialize_field("custom_records", &self.custom_records)?;
        response.serialize_field("node1_policy", &self.node1_policy)?;
        response.serialize_field("node1_pub", &self.node1_pub)?;
        response.serialize_field("node2_policy", &self.node2_policy)?;
        response.serialize_field("node2_pub", &self.node2_pub)?;

        response.end()
    }
}

impl Serialize for NodeAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("NodeAddress", 1)?;
        response.serialize_field("addr", &self.addr)?;
        response.serialize_field("network", &self.network)?;

        response.end()
    }
}

impl Serialize for LightningNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("LightningNode", 1)?;
        response.serialize_field("alias", &self.alias)?;
        response.serialize_field("addresses", &self.addresses)?;
        response.serialize_field("color", &self.color)?;
        response.serialize_field("custom_records", &self.custom_records)?;
        response.serialize_field("features", &self.features)?;
        response.serialize_field("last_update", &self.last_update)?;
        response.serialize_field("pub_key", &self.pub_key)?;

        response.end()
    }
}

impl Serialize for ChannelGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("ChannelGraph", 1)?;
        response.serialize_field("edges", &self.edges)?;
        response.serialize_field("nodes", &self.nodes)?;

        response.end()
    }
}

pub async fn get_describegraph(node_name: web::Path<String>) -> web::Json<ReturnHTTPResponse> {
    let mut node_connection = NodeConnection::new(&node_name).await.unwrap();

    let include_unannounced = true;
    let lnd_response = node_connection
        .client
        .lightning()
        .describe_graph(ChannelGraphRequest {
            include_unannounced,
        })
        .await
        .expect("failed to get info");

    let lnd_response: ChannelGraph = lnd_response.into_inner();

    web::Json(ReturnHTTPResponse {
        message: lnd_response,
    })
}
