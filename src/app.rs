use std::error::Error;

use crate::{
    chat::{Chat, Message, User},
    connection::Connection,
    update::update,
};

pub struct App {
    title: String,
    pub chat: Chat,
    enhanced_graphics: bool,
    pub do_close: bool,
    pub connection: Connection,
    errors: Vec<Box<dyn Error>>,
}

impl App {
    pub async fn new(title: String, enhanced_graphics: bool) -> App {
        App {
            title,
            chat: Chat::new().await,
            enhanced_graphics,
            do_close: false,
            connection: Connection::new().await,
            errors: Vec::new(),
        }
    }

    pub fn new_sync(title: String, enhanced_graphics: bool) -> App {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { Self::new(title, enhanced_graphics).await })
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.do_close = true,
            _ => (),
        }
    }

    pub async fn on_tick(&mut self) {
        update(self).await;
    }

    pub fn log_error(&mut self, err: impl Error + 'static) {
        self.errors.push(Box::new(err))
    }
}

pub trait Pushable {
    fn push(self, chat: &mut Chat);
}

impl Pushable for Message {
    fn push(self, chat: &mut Chat) {
        chat.push_message(self);
    }
}

impl Pushable for User {
    fn push(self, chat: &mut Chat) {
        chat.add_user(self);
    }
}
