use crate::actualization::actualize_messages;
use crate::chat::Message;
use crate::data::serialize_messages;
use reqwest::get;
use std::error::Error;

pub async fn get_history() -> Result<Vec<Message>, Box<dyn Error>> {
    let url = "https://www.destiny.gg/api/chat/history";
    let data = get(url).await?.text().await?;
    let message_data = serialize_messages(data.as_bytes())?;
    let messages = actualize_messages(message_data);
    Ok(messages)
}
