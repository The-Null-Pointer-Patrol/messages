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

impl NodeEvent {
    /// returns the `source` of the event, which is the node that sent the event.
    /// Returns `None` only in the case of `PacketSent(packet)` when the packet is malformed,
    /// which means that the previous_hop can't be calculated
    pub fn source(&self) -> Option<NodeId> {
        match self {
            NodeEvent::PacketSent(packet) => packet.routing_header.previous_hop(),
            NodeEvent::StartingMessageTransmission(message)
            | NodeEvent::MessageSentSuccessfully(message)
            | NodeEvent::MessageReceived(message) => Some(message.source_id),
            NodeEvent::KnownNetworkGraph { source, graph: _ } => Some(*source),
        }
    }
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
