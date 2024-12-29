use std::sync::{Arc, RwLock};
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};
use crate::{DroneSend, Message};

pub enum NodeEvent<M: DroneSend> {
    PacketSent(Packet),
    PacketReceived(Packet),
    StartingMessageTransmission(Message<M>),
    MessageReceived(Message<M>),
    KnownNetworkGraph(NetworkGraph)
}

#[derive(Debug)]
pub struct NetworkGraph {
    pub nodes: RwLock<Vec<Arc<RwLock<NetworkNode>>>>
}

#[derive(Debug)]
pub struct NetworkNode {
    pub node_id: NodeId,
    pub node_type: NodeType,
    pub num_of_dropped_packets: u64, // TODO: does this need to be communicated to SC?
    pub neighbors: RwLock<Vec<NodeId>>
}