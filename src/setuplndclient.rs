use crate::lnd_grpc_rust::lnrpc::GetInfoResponse;
use crate::lnd_grpc_rust::LndClient;
use crate::lnd_grpc_rust::LndClientError;
use crate::lnd_grpc_rust::MyChannel;
use bb8::ManageConnection;
use derive_builder::Builder;
// use lnd_grpc_rust::{LndClient, LndClientError, LndStateClient};
use serde::{Deserialize, Serialize};
use std::clone;
use std::error;
use std::fs;
// use tonic::async_trait;
use tonic::Code::InvalidArgument;

use crate::lnd_grpc_rust::get_channel;
use async_trait::async_trait;
use bb8;

#[derive(Default, Builder, Clone, Debug, Deserialize)]
#[builder(setter(into))]
pub struct ClientConfig {
    pub cert: String,
    pub macaroon: String,
    pub socket: String,
}

impl ClientConfigBuilder {
    pub fn set_cert(&mut self, cert: &str) -> &mut Self {
        let cert_bytes = fs::read(cert).expect("FailedToReadTlsCertFile");
        self.cert = Some(buffer_as_hex(cert_bytes));
        self
    }
    pub fn set_mac(&mut self, mac: &str) -> &mut Self {
        let mac_bytes = fs::read(mac).expect("FailedToReadMacaroonFile");
        self.macaroon = Some(buffer_as_hex(mac_bytes));
        self
    }
}
// buffer_as_hex is a utility function to convert the bytes to a hex string
fn buffer_as_hex(bytes: Vec<u8>) -> String {
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    return hex_str;
}

#[derive(Clone)]
pub struct LndConnectionManager {
    pub client_config: ClientConfig,
}

impl LndConnectionManager {
    pub async fn new(cc: ClientConfig) -> Result<LndConnectionManager, LndClientError> {
        Ok(LndConnectionManager { client_config: cc })
    }
}

#[async_trait]
impl bb8::ManageConnection for LndConnectionManager {
    type Connection = LndClient;
    type Error = LndClientError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let lnd_client = crate::lnd_grpc_rust::connect(
            self.client_config.cert.clone(),
            self.client_config.macaroon.clone(),
            self.client_config.socket.clone(),
        )
        .await;
        Ok(lnd_client.unwrap())
    }

    async fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
