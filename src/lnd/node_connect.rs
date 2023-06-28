use serde::Deserialize;
use serde_derive::Serialize;
use std::error;
use std::{env, fs};

use crate::lnd::lnd_client::LndClient;

#[derive(Default, Clone, PartialEq, Deserialize)]
pub struct NodeConfigurations {
    pub nodes: Vec<Node>,
}

impl NodeConfigurations {
    pub fn new(config_file: String) -> NodeConfigurations {
        let mut node_configurations: NodeConfigurations = {
            let node_configurations =
                std::fs::read_to_string(config_file).expect("JSON was not well-formatted");
            serde_json::from_str::<NodeConfigurations>(&node_configurations).unwrap()
        };

        for n in &mut node_configurations.nodes {
            let cert_bytes = fs::read(&n.cert_path.clone()[..]).expect("FailedToReadTlsCertFile");
            n.cert = Some(buffer_as_hex(cert_bytes));

            let macaroon_bytes =
                fs::read(&n.macaroon_path.clone()[..]).expect("FailedToReadTlsCertFile");
            n.macaroon = Some(buffer_as_hex(macaroon_bytes));
        }

        node_configurations
    }

    pub fn get_node_index(&self, node_name: String) -> usize {
        self.nodes.iter().position(|r| r.name == node_name).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub cert: Option<String>,
    pub macaroon: Option<String>,
    pub cert_path: String,
    pub macaroon_path: String,
    pub socket: String,
}
// buffer_as_hex is a utility function to convert the bytes to a hex string
fn buffer_as_hex(bytes: Vec<u8>) -> String {
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    hex_str
}

pub struct NodeConnection {
    pub client: LndClient,
}

impl NodeConnection {
    pub async fn new(node_name: String) -> Result<NodeConnection, Box<dyn error::Error>> {
        let node_config_file = env::var("NODE_CONFIG_FILE").expect("NODE_CONFIG_FILE not set");
        let node_configurations = NodeConfigurations::new(node_config_file);
        let node_index = node_configurations.get_node_index(node_name.to_string());

        let client = crate::lnd::lnd_client::connect(
            node_configurations.nodes[node_index].cert.clone().unwrap(),
            node_configurations.nodes[node_index]
                .macaroon
                .clone()
                .unwrap(),
            node_configurations.nodes[node_index].socket.clone(),
        )
        .await
        .expect("failed to connect");

        Ok(NodeConnection { client })
    }
}
