pub mod inbound;
mod outbound;

const MSG: &[u8] = "MSG".as_bytes();

use inbound::*;
use async_trait::async_trait;
use crate::display::Display;
use std::fmt;

#[async_trait]
pub trait Data {
    async fn process(&self);
}

#[async_trait]
impl Data for ChatMessage {
    async fn process(&self) {
        self.display().await;
    }
}

pub fn retrieve(bytes: &[u8]) -> Result<impl Data> {
    let (t, d) = seperate(bytes);
    match t {
        MSG => {
            let r: std::result::Result<ChatMessage, _> = serde_json::from_slice(d);
            match r {
                Ok(cm) => Ok(cm),
                Err(_) => Err(RetrieveError)
            }
        },
        _ => Err(RetrieveError)
    }
}

pub async fn process(bytes: &[u8]) {
    let r = retrieve(bytes);
    match r {
        Ok(d) => d.process().await,
        Err(_) => ()
    }
}

pub async fn process_data(data: &impl Data) {
    data.process().await;
}

fn seperate(bytes: &[u8]) -> (&[u8], &[u8]) {
    let s = first_word(bytes);
    bytes.split_at(s)
}

fn first_word(bytes: &[u8]) -> usize {
    let space = b' ';
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == space {
            return i;
        }
    }

    bytes.len()
}

type Result<T> = std::result::Result<T, RetrieveError>;

#[derive(Debug)]
pub struct RetrieveError;

impl fmt::Display for RetrieveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to retrieve data")
    }
}