use std::fmt::Debug;
use wg_2024::network::NodeId;

mod node;
pub mod node_event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub source_id: NodeId,
    pub session_id: u64,
    pub content: MessageType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Request(RequestType),
    Response(ResponseType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestType {
    TextRequest(TextRequest),
    MediaRequest(MediaRequest),
    ChatRequest(ChatRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseType {
    TextResponse(TextResponse),
    MediaResponse(MediaResponse),
    ChatResponse(ChatResponse),
}

// ReqServerType,
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextRequest {
    TextList,
    Text(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaRequest {
    MediaList,
    Media(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatRequest {
    ClientList,
    Register(NodeId),
    SendMessage {
        from: NodeId,
        to: NodeId,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextResponse {
    TextList(Vec<u64>),
    Text(String),
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaResponse {
    MediaList(Vec<u64>),
    Media(Vec<u8>), // should we use some other type?
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatResponse {
    ClientList(Vec<NodeId>),
    MessageFrom { from: NodeId, message: Vec<u8> },
    MessageSent,
}