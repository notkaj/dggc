use crate::data::inbound::MessageData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub trait Display {
    async fn display(&self);
}

impl Display for MessageData {
    async fn display(&self) {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(self.nick.as_bytes()).await.unwrap();
        stdout.write_all(": ".as_bytes()).await.unwrap();
        stdout.write_all(self.data.as_bytes()).await.unwrap();
        stdout.write_u8(b'\n').await.unwrap();
    }
}
