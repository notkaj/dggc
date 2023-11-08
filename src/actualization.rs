use crate::chat::{Message, User};
use crate::data::inbound::{MessageData, UserData, UsersData};
use crate::flairs::{color, flairs};
use colors_transform::{Color, Rgb};

// pub enum Actual {
//     Join(User),
//     Quit(User),
//     Update(User),
//     Users(Vec<User>),
//     Message(Message),
//     Error,
// }

// pub async fn actualize(data: Data) -> Actual {
//     match data {
//         Data::Join(ud) => Actual::Join(actualize_user(ud).await),
//         Data::Message(md) => Actual::Message(actualize_message(md).await),
//         _ => Actual::Error,
//     }
// }

pub async fn actualize_user(user_data: UserData) -> User {
    let flairs = flairs().await;
    let user_flairs = user_data
        .features
        .iter()
        .filter_map(|f| flairs.get(f))
        .collect();
    let hex_color = color(&user_flairs);
    let (r, g, b) = match Rgb::from_hex_str(hex_color) {
        Ok(rgb) => rgb.as_tuple(),
        Err(_) => (0.0, 0.0, 0.0),
    };
    let labels = user_flairs.iter().map(|&f| f.label.as_str()).collect();
    let icons = user_data
        .features
        .iter()
        .filter_map(|f| flairs.get_icon(f))
        .collect();
    User {
        name: user_data.nick,
        color: (r as u8, g as u8, b as u8),
        labels,
        icons,
    }
}

pub fn actualize_message(message_data: MessageData) -> Message {
    Message {
        // user: chat.find_user(&self.nick).ok_or(ConversionError)?,
        username: message_data.nick,
        body: message_data.data,
        timestamp: message_data.timestamp,
    }
}

pub fn actualize_messages(message_data: Vec<MessageData>) -> Vec<Message> {
    let mut messages = Vec::new();
    for data in message_data {
        let message = actualize_message(data);
        messages.push(message);
    }
    messages
}

pub async fn actualize_users(users_data: UsersData) -> Vec<User> {
    let mut users = Vec::new();
    for data in users_data.users {
        let user = actualize_user(data).await;
        users.push(user);
    }
    users
}
