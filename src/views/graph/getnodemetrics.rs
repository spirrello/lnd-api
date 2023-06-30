use crate::lnd::lnd_client::lnrpc::*;
use crate::lnd::node_connect::NodeConnection;
use actix_web::web;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_derive::Serialize;
// use std::collections::HashMap;

// use serde_with::serde_as;
// #[serde_as]
#[derive(Serialize)]
pub struct ReturnHTTPResponse {
    pub message: NodeMetricsResponse,
}

impl Serialize for FloatMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("FloatMetric", 1)?;
        response.serialize_field("normalized_value", &self.normalized_value)?;
        response.end()
    }
}

impl Serialize for NodeMetricsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("NodeMetricsResponse", 1)?;
        response.serialize_field("peers", &self.betweenness_centrality)?;
        response.end()
    }
}

pub async fn get_getnodemetrics(node_name: web::Path<String>) -> web::Json<ReturnHTTPResponse> {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let types: Vec<i32> = vec![1];
    let lnd_response = node_connection
        .client
        .lightning()
        .get_node_metrics(NodeMetricsRequest { types })
        .await
        .expect("failed to get info");

    let lnd_response: NodeMetricsResponse = lnd_response.into_inner();

    web::Json(ReturnHTTPResponse {
        message: lnd_response,
    })
}
