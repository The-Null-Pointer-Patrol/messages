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
pub(super) struct NetworkGraph {
    nodes: RwLock<Vec<Arc<RwLock<NetworkNode>>>>
}

#[derive(Debug)]
pub(super) struct NetworkNode {
    pub(super) node_id: NodeId,
    pub(super) node_type: NodeType,
    pub(super) num_of_dropped_packets: u64, // TODO: does this need to be communicated to SC?
    pub(super) neighbors: RwLock<Vec<NodeId>>
}