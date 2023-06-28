// use super::super::common::features::Features;
use super::super::httpresponse::LNDHTTPResponse;
use super::super::setuplndclient::NodeConnection;
use actix_web::{get, web, HttpResponse};
use derive_builder::Builder;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub last_update: u32,
    pub pub_key: String,
    pub alias: String,
    pub addresses: Vec<Addresses>,
    pub color: String,
    pub custom_records: HashMap<u64, Vec<u8>>,
}

#[derive(Default, Clone, Builder, Debug, PartialEq, Serialize, Deserialize)]
pub struct Addresses {
    pub network: String,
    pub addr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomRecords {}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub channel_id: u64,
    pub chan_point: String,
    pub node1_pub: String,
    pub node2_pub: String,
    pub capacity: i64,
    pub node1_policy: Node1Policy,
    pub node2_policy: Node2Policy,
    // pub custom_records: HashMap<u64, Vec<u8>>,
}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node1Policy {
    pub time_lock_delta: u32,
    pub min_htlc: i64,
    pub fee_base_msat: i64,
    pub fee_rate_milli_msat: i64,
    pub disabled: bool,
    pub max_htlc_msat: u64,
    pub last_update: u32,
    pub custom_records: HashMap<u64, Vec<u8>>,
}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node2Policy {
    pub time_lock_delta: u32,
    pub min_htlc: i64,
    pub fee_base_msat: i64,
    pub fee_rate_milli_msat: i64,
    pub disabled: bool,
    pub max_htlc_msat: u64,
    pub last_update: u32,
    pub custom_records: HashMap<u64, Vec<u8>>,
}

#[get("/describegraph/{node}")]
pub async fn describe_graph(node_name: web::Path<String>) -> HttpResponse {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let include_unannounced = true;
    let describe_graph_response = node_connection
        .client
        .lightning()
        .describe_graph(lnd_grpc_rust::lnrpc::ChannelGraphRequest {
            include_unannounced,
        })
        .await
        .expect("failed to get info");

    let describe_graph_response = describe_graph_response.into_inner();

    let mut describe_graph = DescribeGraph::default();

    for node in describe_graph_response.nodes.iter() {
        let addresses: Vec<Addresses> = node
            .addresses
            .iter()
            .map(|a| {
                let node_address = AddressesBuilder::default()
                    .addr(a.addr.clone())
                    .network(a.network.clone())
                    .build()
                    .unwrap();
                node_address
            })
            .collect();

        let n = NodeBuilder::default()
            .last_update(node.last_update.clone())
            .pub_key(node.pub_key.clone())
            .alias(node.alias.clone())
            .color(node.color.clone())
            .custom_records(node.custom_records.clone())
            .addresses(addresses)
            .build()
            .unwrap();
        describe_graph.nodes.push(n);
    }

    for edge in describe_graph_response.edges.iter() {
        let mut node1_policy = Node1Policy::default();
        let mut node2_policy = Node2Policy::default();
        for node_policy in edge.node1_policy.iter() {
            let node_policy = Node1PolicyBuilder::default()
                .time_lock_delta(node_policy.time_lock_delta.clone())
                .min_htlc(node_policy.min_htlc.clone())
                .fee_base_msat(node_policy.fee_base_msat.clone())
                .fee_rate_milli_msat(node_policy.fee_base_msat.clone())
                .disabled(node_policy.disabled.clone())
                .max_htlc_msat(node_policy.max_htlc_msat.clone())
                .last_update(node_policy.last_update.clone())
                .custom_records(node_policy.custom_records.clone())
                .build()
                .unwrap();
            node1_policy = node_policy;
        }
        for node_policy in edge.node2_policy.iter() {
            let node_policy = Node2PolicyBuilder::default()
                .time_lock_delta(node_policy.time_lock_delta.clone())
                .min_htlc(node_policy.min_htlc.clone())
                .fee_base_msat(node_policy.fee_base_msat.clone())
                .fee_rate_milli_msat(node_policy.fee_base_msat.clone())
                .disabled(node_policy.disabled.clone())
                .max_htlc_msat(node_policy.max_htlc_msat.clone())
                .last_update(node_policy.last_update.clone())
                .custom_records(node_policy.custom_records.clone())
                .build()
                .unwrap();
            node2_policy = node_policy;
        }

        let e = EdgeBuilder::default()
            .channel_id(edge.channel_id.clone())
            .chan_point(edge.chan_point.clone())
            .node1_pub(edge.node1_pub.clone())
            .node2_pub(edge.node2_pub.clone())
            .capacity(edge.capacity.clone())
            .node1_policy(node1_policy)
            .node2_policy(node2_policy)
            .build()
            .unwrap();

        describe_graph.edges.push(e);
    }

    let response_json = &LNDHTTPResponse {
        status: "success".to_string(),
        message: describe_graph,
    };
    HttpResponse::Ok().json(response_json)
}
