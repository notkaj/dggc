use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::data::inbound::*;
use async_trait::async_trait;

#[async_trait]
pub trait Display {
    async fn display(&self);
}

#[async_trait]
impl Display for ChatMessage {
    async fn display(&self) {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(self.nick.as_bytes()).await.unwrap();
        stdout.write_all(": ".as_bytes()).await.unwrap();
        stdout.write_all(self.data.as_bytes()).await.unwrap();
        stdout.write_u8(b'\n').await.unwrap();
    }
}
