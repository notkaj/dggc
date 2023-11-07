pub mod inbound;
pub mod outbound;

// pub const MSG: &[u8] = b"MSG";
pub const NAMES: &[u8] = "NAMES".as_bytes();
pub const JOIN: &[u8] = "JOIN".as_bytes();
pub const QUIT: &[u8] = "QUIT".as_bytes();
pub const UPDATEUSER: &[u8] = "UPDATEUSER".as_bytes();

// use crate::conversion::Convertable;
use crate::data::MessageData;
use inbound::*;
use std::error::Error;
use std::fmt;

// pub enum Data {
//     Join(User),
//     Quit(User),
//     Message(Message),
//     Err,
//     Users(Vec<User>),
//     Update(User),
// }

// pub fn serialize(bytes: Vec<u8>) -> Data {
//     let (t, d) = seperate(bytes.as_slice());
//     match t {
//         MSG => match serialize_message(d) {
//             Ok(md) => Data::Message(md),
//             Err(e) => Data::Err(e),
//         },
//         JOIN => match serialize_user(d) {
//             Ok(ud) => Data::Join(ud),
//             Err(e) => Data::Err(e),
//         },
//         QUIT => match serialize_user(d) {
//             Ok(ud) => Data::Quit(ud),
//             Err(e) => Data::Err(e),
//         },
//         NAMES => match serialize_users(d) {
//             Ok(uds) => Data::Users(uds),
//             Err(e) => Data::Err(e),
//         },
//         UPDATEUSER => match serialize_user(d) {
//             Ok(uds) => Data::Update(uds),
//             Err(e) => Data::Err(e),
//         },
//         _ => Data::Err(SerializeError),
//     }
// }

pub fn serialize_message(bytes: &[u8]) -> SerializeResult<MessageData> {
    let result = serde_json::from_slice::<MessageData>(bytes);
    match result {
        Ok(md) => Ok(md),
        Err(_) => Err(SerializeError),
    }
}

pub fn serialize_messages(bytes: &[u8]) -> Result<Vec<MessageData>, Box<dyn Error>> {
    let strings = serde_json::from_slice::<Vec<String>>(bytes)?;
    let message_data = strings
        .iter()
        .filter_map(|s| {
            let (_, md) = seperate(s.as_bytes());
            serde_json::from_slice::<MessageData>(md).ok()
        })
        .collect();
    Ok(message_data)
}

pub fn serialize_user(bytes: &[u8]) -> SerializeResult<UserData> {
    let result = serde_json::from_slice::<UserData>(bytes);
    match result {
        Ok(ud) => Ok(ud),
        Err(_) => Err(SerializeError),
    }
}

pub fn serialize_users(bytes: &[u8]) -> SerializeResult<UsersData> {
    let result = serde_json::from_slice::<UsersData>(bytes);
    match result {
        Ok(uds) => Ok(uds),
        Err(_) => Err(SerializeError),
    }
}

// pub async fn process(bytes: &[u8]) {
//     let r = retrieve(bytes);
//     match r {
//         Ok(d) => d.process().await,
//         Err(_) => ()
//     }
// }

// pub async fn process_data(data: &impl Convertable) {
//     data.process().await;
// }

pub fn seperate(bytes: &[u8]) -> (&[u8], &[u8]) {
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

pub type SerializeResult<T> = Result<T, SerializeError>;

#[derive(Debug, Clone)]
pub struct SerializeError;

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to retrieve data")
    }
}

impl std::error::Error for SerializeError {}
