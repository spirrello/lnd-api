use crate::lnd_grpc_rust::LndClient;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::error;
use std::fs;

use std::clone;

use std::rc::Rc;

#[derive(Default, Builder, Clone, Debug)]
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
pub struct LndConn {
    pub client_config: ClientConfig,
    pub lnd_client: LndClient,
}

// pub struct CloneAbleLndConn(pub LndConn);

impl LndConn {
    pub async fn new(cc: ClientConfig) -> Result<Self, Box<dyn error::Error>> {
        let lnd_client =
            crate::lnd_grpc_rust::connect(cc.cert.clone(), cc.macaroon.clone(), cc.socket.clone())
                .await?;

        Ok(LndConn {
            client_config: cc,
            lnd_client,
        })
    }
}
