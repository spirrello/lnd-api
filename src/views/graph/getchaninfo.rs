use crate::lnd::lnd_client::lnrpc::*;
use crate::lnd::node_connect::NodeConnection;
use actix_web::web;
use serde_derive::Serialize;

// use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Debug)]
pub struct ReturnHTTPResponse {
    pub message: ChannelEdge,
}

pub async fn get_getchaninfo(params: web::Path<(String, String)>) -> web::Json<ReturnHTTPResponse> {
    let (node_name, chan_id) = params.into_inner();
    let mut node_connection = NodeConnection::new(&node_name).await.unwrap();

    let chan_id: u64 = chan_id.parse().unwrap();
    let lnd_response = node_connection
        .client
        .lightning()
        .get_chan_info(ChanInfoRequest { chan_id })
        .await
        .expect("failed to get info");

    let lnd_response: ChannelEdge = lnd_response.into_inner();

    web::Json(ReturnHTTPResponse {
        message: lnd_response,
    })
}
