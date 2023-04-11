// use derive_builder::Builder;
use serde::Deserialize;
use serde_derive::Serialize;
use std::fs;

#[derive(Default, Clone, PartialEq, Deserialize)]
pub struct NodeConfigurations {
    pub nodes: Vec<Node>,
}

impl NodeConfigurations {
    pub fn new(config_file: &str) -> NodeConfigurations {
        let mut node_configurations: NodeConfigurations = {
            let node_configurations =
                std::fs::read_to_string(&config_file).expect("JSON was not well-formatted");
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

    return hex_str;
}
// #[derive(Default, Builder, Clone, Debug)]
// #[builder(setter(into))]
// pub struct ClientConfig {
//     pub cert: String,
//     pub macaroon: String,
//     pub socket: String,
// }

// impl ClientConfigBuilder {
//     pub fn set_cert(&mut self, cert: &str) -> &mut Self {
//         let cert_bytes = fs::read(cert).expect("FailedToReadTlsCertFile");
//         self.cert = Some(buffer_as_hex(cert_bytes));
//         self
//     }
//     pub fn set_mac(&mut self, mac: &str) -> &mut Self {
//         let mac_bytes = fs::read(mac).expect("FailedToReadMacaroonFile");
//         self.macaroon = Some(buffer_as_hex(mac_bytes));
//         self
//     }
// }
