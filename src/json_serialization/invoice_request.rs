use serde::Deserialize;

#[derive(Deserialize)]
pub struct InvoiceRequest {
    pub memo: String,
    pub millisat: i64,
    pub node_name: String,
}
