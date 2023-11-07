use crate::actualization::{actualize_message, actualize_user, actualize_users};
use crate::app::App;
use crate::data::{seperate, serialize_message, serialize_user, serialize_users};

// pub enum Retrieval {
//     Join(User),
//     Quit(User),
//     Message(Message),
//     Err(Box<dyn Error>),
//     Users(Vec<User>),
//     Update(User),
// }

//VERY tightly coupled code SWEATSTINY
pub async fn update(app: &mut App) {
    let captures = app.connection.capture().await;
    for capture in captures {
        let (label, data) = seperate(capture.as_slice());
        match label {
            b"MSG" => match serialize_message(data) {
                Ok(md) => app.chat.push_message(actualize_message(md)),
                Err(e) => app.log_error(e),
            },
            b"JOIN" => match serialize_user(data) {
                Ok(ud) => app.chat.add_user(actualize_user(ud).await),
                Err(e) => app.log_error(e),
            },
            b"QUIT" => match serialize_user(data) {
                Ok(ud) => app.chat.remove_user(actualize_user(ud).await),
                Err(e) => app.log_error(e),
            },
            b"NAMES" => match serialize_users(data) {
                Ok(uds) => app.chat.add_users(actualize_users(uds).await),
                Err(e) => app.log_error(e),
            },
            b"UPDATEUSER" => match serialize_user(data) {
                Ok(uds) => app.chat.update_user(actualize_user(uds).await),
                Err(e) => app.log_error(e),
            },
            _ => app.log_error(UpdateError),
        };
    }
}

use std::fmt;

pub type UpdateResult<T> = Result<T, UpdateError>;

#[derive(Debug, Clone)]
pub struct UpdateError;

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to retrieve data")
    }
}

impl std::error::Error for UpdateError {}
