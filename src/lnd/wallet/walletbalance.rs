use super::super::httpresponse::LNDHTTPResponse;
use super::super::setuplndclient::NodeConnection;
use actix_web::{get, web, HttpResponse};
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Builder, Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    pub total_balance: i64,
    pub confirmed_balance: i64,
    pub unconfirmed_balance: i64,
    pub locked_balance: i64,
    pub reserved_balance_anchor_chan: i64,
    pub account_balance: AccountBalance,
}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalance {
    pub balance_default: AccountBalanceDefault,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalanceDefault {
    pub confirmed_balance: i64,
    pub unconfirmed_balance: i64,
}

#[get("/walletbalance/{node}")]
pub async fn walelt_balance(node_name: web::Path<String>) -> HttpResponse {
    let mut node_connection = NodeConnection::new(node_name.to_string()).await.unwrap();

    let wallet_balance_response = node_connection
        .client
        .lightning()
        .wallet_balance(lnd_grpc_rust::lnrpc::WalletBalanceRequest {})
        .await
        .expect("failed to get wallet balance");

    let wallet_balance_response = wallet_balance_response.into_inner();

    let mut account_balance_default = AccountBalanceDefault::default();
    for x in &wallet_balance_response.account_balance {
        account_balance_default.confirmed_balance = x.1.confirmed_balance.clone();
        account_balance_default.unconfirmed_balance = x.1.unconfirmed_balance.clone();
    }

    let account_balance = AccountBalanceBuilder::default()
        .balance_default(account_balance_default)
        .build()
        .unwrap();

    let wallet_balance = WalletBalanceBuilder::default()
        .account_balance(account_balance)
        .total_balance(wallet_balance_response.total_balance.clone())
        .confirmed_balance(wallet_balance_response.confirmed_balance.clone())
        .unconfirmed_balance(wallet_balance_response.unconfirmed_balance.clone())
        .locked_balance(wallet_balance_response.locked_balance.clone())
        .reserved_balance_anchor_chan(wallet_balance_response.reserved_balance_anchor_chan.clone())
        .build()
        .unwrap();

    let response_json = &LNDHTTPResponse {
        status: "success".to_string(),
        message: wallet_balance,
    };
    HttpResponse::Ok().json(response_json)
}
