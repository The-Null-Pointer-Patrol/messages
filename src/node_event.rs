use std::sync::{Arc, RwLock};
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};
use crate::Message;

#[derive(Debug)]
pub enum NodeEvent {
    PacketSent(Packet),
    PacketReceived(Packet),
    StartingMessageTransmission(Message),
    MessageSentSuccessfully(Message),
    MessageReceived(Message),
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
    pub num_of_dropped_packets: u64,
    pub neighbors: RwLock<Vec<NodeId>>
}