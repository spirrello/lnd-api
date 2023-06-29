use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::{body::BoxBody, HttpResponse, Responder};
use serde_derive::Serialize;

// use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Debug)]
pub struct ReturnHTTPResponse<T: serde::Serialize> {
    pub message: T,
}

impl<T: serde::Serialize> Responder for ReturnHTTPResponse<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
