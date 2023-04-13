use serde_derive::Serialize;

#[derive(Serialize)]
pub struct LNDHTTPResponse<T> {
    pub status: String,
    pub message: T,
}
