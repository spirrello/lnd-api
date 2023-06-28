use super::super::httpresponse::LNDHTTPResponse;
use super::super::setuplndclient::NodeConnection;
use actix_web::{post, web, HttpResponse};
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Builder, Debug, Clone, Serialize, Deserialize)]
pub struct LndGridAddInvoiceRequest {
    node_name: String,
    memo: String,
    amount_paid: i64,
    expiry: u64,
    amp: i64,
    r_preimage: Vec<u8>,
    value_msat: i64,
    description_hash: Vec<u8>,
    fallback_addr: String,
    cltv_expiry: u64,
    route_hints: Vec<String>,
    private: bool,
    is_amp: bool,
}

#[post("/addinvoice/{node}")]
pub async fn add_invoice(invoice_request: web::Data<AddInvoiceRequest>) -> HttpResponse {
    let mut node_connection = NodeConnection::new(invoice_request.node_name)
        .await
        .unwrap();

    let add_invoice_response = node_connection
        .client
        .lightning()
        .add_invoice(lnd_grpc_rust::lnrpc::AddInvoiceRequest {})
        .await
        .expect("failed to get wallet balance");

    let add_invoice_response = add_invoice_response.into_inner();

    // for x in &wallet_balance_response.account_balance {
    //     account_balance_default.confirmed_balance = x.1.confirmed_balance.clone();
    //     account_balance_default.unconfirmed_balance = x.1.unconfirmed_balance.clone();
    // }

    // let account_balance = AccountBalanceBuilder::default()
    //     .balance_default(account_balance_default)
    //     .build()
    //     .unwrap();

    // let wallet_balance = WalletBalanceBuilder::default()
    //     .account_balance(account_balance)
    //     .total_balance(wallet_balance_response.total_balance.clone())
    //     .confirmed_balance(wallet_balance_response.confirmed_balance.clone())
    //     .unconfirmed_balance(wallet_balance_response.unconfirmed_balance.clone())
    //     .locked_balance(wallet_balance_response.locked_balance.clone())
    //     .reserved_balance_anchor_chan(wallet_balance_response.reserved_balance_anchor_chan.clone())
    //     .build()
    //     .unwrap();

    let response_json = &LNDHTTPResponse {
        status: "success".to_string(),
        message: wallet_balance,
    };
    HttpResponse::Ok().json(response_json)
}
