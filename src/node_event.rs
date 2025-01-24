use crate::Message;
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};

#[derive(Debug, Clone)]
pub enum NodeEvent {
    PacketSent(Packet),
    StartingMessageTransmission(Message),
    MessageSentSuccessfully(Message),
    MessageReceived(Message),
    // NodeId field is used to know the id of event sender
    KnownNetworkGraph {
        source: NodeId,
        graph: EventNetworkGraph,
    },
    //UpdateDroppedPackets {
    //    node: NodeId,
    //    num_of_dropped_packets: u64,
    //},
}

#[derive(Debug, Clone)]
pub struct EventNetworkGraph {
    pub nodes: Vec<EventNetworkNode>,
}

#[derive(Debug, Clone)]
pub struct EventNetworkNode {
    pub node_id: NodeId,
    pub node_type: NodeType,
    //pub num_of_dropped_packets: u64,
    pub neighbors: Vec<NodeId>,
}
