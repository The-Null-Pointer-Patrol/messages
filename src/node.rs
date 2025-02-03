use crossbeam_channel::{Receiver, Sender};
use std::{collections::HashMap, fmt};
use wg_2024::{
    config::{Client, Drone, Server},
    controller::{DroneCommand, DroneEvent},
    network::NodeId,
    packet::Packet,
};

#[derive(Debug, Clone)]
pub struct NodeOptions {
    pub kind: NodeKind,
    pub id: NodeId,
    pub event_send: Sender<DroneEvent>,
    pub command_recv: Receiver<DroneCommand>,
    pub packet_send: HashMap<NodeId, Sender<Packet>>,
    pub packet_recv: Receiver<Packet>,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Drone { pdr: f32 },
    Server,
    Client,
}
impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeKind::Drone { pdr: _ } => write!(f, "Drone"),
            NodeKind::Server => write!(f, "Server"),
            NodeKind::Client => write!(f, "Client"),
        }
    }
}

pub struct Node {
    pub kind: NodeKind,
    pub id: NodeId,
    pub neighbors: Vec<NodeId>,
}
impl From<Drone> for Node {
    fn from(value: Drone) -> Self {
        Node {
            kind: NodeKind::Drone { pdr: value.pdr },
            id: value.id,
            neighbors: value.connected_node_ids,
        }
    }
}
impl From<Server> for Node {
    fn from(value: Server) -> Self {
        Node {
            kind: NodeKind::Server,
            id: value.id,
            neighbors: value.connected_drone_ids,
        }
    }
}
impl From<Client> for Node {
    fn from(value: Client) -> Self {
        Node {
            kind: NodeKind::Client,
            id: value.id,
            neighbors: value.connected_drone_ids,
        }
    }
}
