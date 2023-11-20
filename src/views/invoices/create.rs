use crate::{
    json_serialization::invoice_request::InvoiceRequest,
    lnd::{
        lnd_client::lnrpc::AddInvoiceResponse, lnd_client::lnrpc::Invoice,
        node_connect::NodeConnection,
    },
};
use actix_web::web;
use serde_derive::Serialize;

use crate::storage::cache::*;
use base64::encode;
use deadpool_redis::{Config, Connection, Pool, Runtime};
use derive_builder::Builder;

use redis::FromRedisValue;

#[derive(Debug, Serialize)]
pub struct ReturnHTTPResponse {
    pub message: InvoiceHTTPResponse,
}

#[derive(Default, Builder, Debug, Serialize)]
pub struct InvoiceHTTPResponse {
    r_hash: String,
    add_index: u64,
    payment_addr: String,
    payment_request: String,
}

pub async fn create_invoice(
    invoice_request: web::Json<InvoiceRequest>,
    redis_pool: web::Data<Pool>,
) -> web::Json<ReturnHTTPResponse> {
    let mut node_connection = NodeConnection::new(&invoice_request.node_name)
        .await
        .unwrap();

    let mut lnd_invoice_request = Invoice::default();
    lnd_invoice_request.value_msat = invoice_request.millisat.clone();
    lnd_invoice_request.memo = "beer".into();

    let lnd_response = node_connection
        .client
        .lightning()
        .add_invoice(lnd_invoice_request)
        .await
        .expect("failed to get info");
    let lnd_response: AddInvoiceResponse = lnd_response.into_inner();

    let r_hash = encode(&lnd_response.r_hash);
    let payment_addr = encode(&lnd_response.payment_addr);
    let invoice_http_response = InvoiceHTTPResponseBuilder::default()
        .add_index(lnd_response.add_index.clone())
        .payment_addr(payment_addr)
        .payment_request(lnd_response.payment_request.clone())
        .r_hash(r_hash.clone())
        .build()
        .unwrap();

    let cache_payload = CachePayload::new(
        format!("{}:{}", "invoice", r_hash),
        vec![(
            "payment_request".to_string(),
            lnd_response.payment_request.clone(),
        )],
    );
    match set_hash(&redis_pool, cache_payload).await {
        Err(redis_error) => error!("Error persisting to redis: {:?}", redis_error),
        _ => info!("setting to redis worked"),
    }

    web::Json(ReturnHTTPResponse {
        message: invoice_http_response,
    })
}
