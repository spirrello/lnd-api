use crate::lnd::{lnd_client::lnrpc::*, node_connect::NodeConnection};
use actix_web::web;
use serde_derive::Serialize;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Debug)]
pub struct ReturnHTTPResponse {
    pub message: WalletBalanceResponse,
}

impl Serialize for WalletAccountBalance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("WalletAccountBalance", 1)?;
        response.serialize_field("confirmed_balance", &self.confirmed_balance)?;
        response.serialize_field("unconfirmed_balance", &self.unconfirmed_balance)?;

        response.end()
    }
}

impl Serialize for WalletBalanceResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response = serializer.serialize_struct("WalletBalanceResponse", 1)?;
        response.serialize_field("confirmed_balance", &self.confirmed_balance)?;
        response.serialize_field("locked_balance", &self.locked_balance)?;
        response.serialize_field(
            "reserved_balance_anchor_chan",
            &self.reserved_balance_anchor_chan,
        )?;
        response.serialize_field("total_balance", &self.total_balance)?;
        response.serialize_field("unconfirmed_balance", &self.unconfirmed_balance)?;
        response.serialize_field("account_balance", &self.account_balance)?;

        response.end()
    }
}

pub async fn get_walletbalance(node_name: web::Path<String>) -> web::Json<ReturnHTTPResponse> {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let lnd_response = node_connection
        .client
        .lightning()
        .wallet_balance(WalletBalanceRequest {})
        .await
        .expect("failed to get info");
    let lnd_response = lnd_response.into_inner();

    web::Json(ReturnHTTPResponse {
        message: lnd_response,
    })
}
