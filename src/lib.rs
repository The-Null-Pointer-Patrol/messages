use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use wg_2024::network::NodeId;

pub mod node;
pub mod node_event;

pub trait MessageUtilities: Serialize + DeserializeOwned + Send {
    fn stringify(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// # Errors
    /// Will return an error in the case deserialization failed.
    fn from_string(raw: String) -> Result<Self, String>
    where
        Self: Sized
    {
        serde_json::from_str(raw.as_str()).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub source: NodeId,
    pub destination: NodeId,
    pub session_id: u64,
    pub content: MessageType,
}

impl MessageUtilities for Message { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Request(RequestType),
    Response(ResponseType),
    Error(ErrorType),
}

impl MessageUtilities for MessageType { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestType {
    TextRequest(TextRequest),
    MediaRequest(MediaRequest),
    ChatRequest(ChatRequest),
    DiscoveryRequest(()),
}

impl MessageUtilities for RequestType { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseType {
    TextResponse(TextResponse),
    MediaResponse(MediaResponse),
    ChatResponse(ChatResponse),
    DiscoveryResponse(ServerType),
}

impl MessageUtilities for ResponseType { }

// ReqServerType,
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextRequest {
    TextList,
    Text(u64),
}

impl MessageUtilities for TextRequest { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaRequest {
    MediaList,
    Media(u64),
}

impl MessageUtilities for MediaRequest { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatRequest {
    ClientList,
    Register(NodeId),
    SendMessage {
        from: NodeId,
        to: NodeId,
        message: String,
    },
}

impl MessageUtilities for ChatRequest { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextResponse {
    TextList(Vec<String>),
    Text(String),
    NotFound,
}

impl MessageUtilities for TextResponse { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaResponse {
    MediaList(Vec<String>),
    Media(Vec<u8>),
    NotFound,
}

impl MessageUtilities for MediaResponse { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatResponse {
    ClientList(Vec<NodeId>),
    MessageFrom { from: NodeId, message: String },
    MessageSent,
}

impl MessageUtilities for ChatResponse { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerType {
    CommunicationServer,
    ContentServer,
}

impl MessageUtilities for ServerType { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorType {
    Unsupported(RequestType),
    Unexpected(ResponseType),
    Unregistered(NodeId),
}

impl MessageUtilities for ErrorType { }
