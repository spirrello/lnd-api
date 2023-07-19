use crate::{
    json_serialization::invoice_request::InvoiceRequest,
    lnd::{
        lnd_client::lnrpc::AddInvoiceResponse, lnd_client::lnrpc::Invoice,
        node_connect::NodeConnection,
    },
};
use actix_web::web;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct ReturnHTTPResponse {
    pub message: AddInvoiceResponse,
}

impl Serialize for AddInvoiceResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut thing = serializer.serialize_struct("AddInvoiceResponse", 1)?;
        thing.serialize_field("r_hash", &self.r_hash)?;
        thing.serialize_field("add_index", &self.add_index)?;
        thing.serialize_field("payment_addr", &self.payment_addr)?;
        thing.serialize_field("payment_request", &self.payment_request)?;
        thing.end()
    }
}

pub async fn create_invoice(
    invoice_request: web::Json<InvoiceRequest>,
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

    web::Json(ReturnHTTPResponse {
        message: lnd_response,
    })
}
