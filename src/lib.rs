use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use wg_2024::network::NodeId;

mod node;
pub mod node_event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub source_id: NodeId,
    pub session_id: u64,
    pub content: MessageType,
}

pub trait TestTrait: Serialize + DeserializeOwned + Send {
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
pub enum MessageType {
    Request(RequestType),
    Response(ResponseType),
}

impl TestTrait for MessageType { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestType {
    TextRequest(TextRequest),
    MediaRequest(MediaRequest),
    ChatRequest(ChatRequest),
}

impl TestTrait for RequestType { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseType {
    TextResponse(TextResponse),
    MediaResponse(MediaResponse),
    ChatResponse(ChatResponse),
}

impl TestTrait for ResponseType { }

// ReqServerType,
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextRequest {
    TextList,
    Text(u64),
}

impl TestTrait for TextRequest { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaRequest {
    MediaList,
    Media(u64),
}

impl TestTrait for MediaRequest { }

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

impl TestTrait for ChatRequest { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextResponse {
    TextList(Vec<u64>),
    Text(String),
    NotFound,
}

impl TestTrait for TextResponse { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaResponse {
    MediaList(Vec<u64>),
    Media(Vec<u8>), // should we use some other type?
}

impl TestTrait for MediaResponse { }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatResponse {
    ClientList(Vec<NodeId>),
    MessageFrom { from: NodeId, message: Vec<u8> },
    MessageSent,
}

impl TestTrait for ChatResponse { }