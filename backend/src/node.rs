use serde::Deserialize;
use crate::node_message::NodeMessage;

pub type NodeId = usize;

#[derive(Deserialize, Debug)]
pub struct NodeDetails {
    pub name: Box<str>,
    pub implementation: Box<str>,
    pub version: Box<str>,
}

pub struct Node {
    details: NodeDetails,
}

impl Node {
    pub fn new(details: NodeDetails) -> Self {
        Node {
            details,
        }
    }

    pub fn name(&self) -> &str {
        &self.details.name
    }

    pub fn update(&mut self, chain: &str, msg: NodeMessage) {
        // info!("[{}] [{}] {:?}", chain, self.name(), msg);
    }
}