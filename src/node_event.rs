use std::sync::{Arc, RwLock};
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};
use crate::{DroneSend, Message};

pub enum NodeEvent<M: DroneSend> {
    PacketSent(Packet),
    PacketReceived(Packet),
    StartingMessageTransmission(Message<M>),
    MessageSentSuccessfully(Message<M>),
    MessageReceived(Message<M>),
    KnownNetworkGraph(EventNetworkGraph)
}

#[derive(Debug)]
pub struct EventNetworkGraph {
    pub nodes: Vec<EventNetworkNode>
}

#[derive(Debug)]
pub struct EventNetworkNode {
    pub node_id: NodeId,
    pub node_type: NodeType,
    pub num_of_dropped_packets: u64,
    pub neighbors: Vec<NodeId>,
}