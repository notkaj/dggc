use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub nick: String,
    features: Vec<String>,
    timestamp: usize,
    pub data: String,
}