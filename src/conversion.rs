use crate::{
    app::Pushable,
    chat::{Message, User},
    data::inbound::{MessageData, UserData},
    flairs::{color, flairs},
};
use async_trait::async_trait;
use std::fmt;

#[async_trait]
pub trait Convertable {
    type Conversion: Pushable;
    #[deprecated(
        since = "testing",
        note = "Probably works now, but I've moved to actualization"
    )]
    async fn convert(self) -> Self::Conversion;
}

#[async_trait]
impl Convertable for MessageData {
    type Conversion = Message;
    async fn convert(self) -> Self::Conversion {
        let message = Message {
            // user: chat.find_user(&self.nick).ok_or(ConversionError)?,
            username: self.nick,
            body: self.data,
            timestamp: self.timestamp,
        };
        message
    }
}

#[async_trait]
impl Convertable for UserData {
    type Conversion = User;
    async fn convert(self) -> Self::Conversion {
        let flairs = flairs().await;
        let user_flairs = self.features.iter().filter_map(|f| flairs.get(f)).collect();
        let color = color(&user_flairs);
        let labels = user_flairs.iter().map(|&f| f.label.as_str()).collect();
        let user = User {
            name: self.nick,
            color,
            labels,
            icons: Vec::new(),
        };
        user
    }
}

pub type ConversionResult<T> = std::result::Result<T, ConversionError>;

#[derive(Debug)]
pub struct ConversionError;

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Converstion failed")
    }
}
