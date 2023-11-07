use std::collections::HashMap;

use crate::history;

pub struct Chat {
    pub should_quit: bool,
    pub messages: Vec<Message>,
    // pub users: Vec<User>,
    pub users_by_name: HashMap<String, User>,
}

impl Chat {
    pub async fn new() -> Chat {
        let messages = match history::get_history().await {
            Ok(ms) => ms,
            Err(_) => Vec::new(),
        };
        Chat {
            should_quit: false,
            messages,
            // users: Vec::new(),
            users_by_name: HashMap::new(),
        }
    }

    pub fn find_user(&self, nick: &str) -> Option<&User> {
        // self.users.iter().find(|&p| p.name == nick)
        self.users_by_name.get(&nick.to_string())
    }

    pub fn add_user(&mut self, user: User) {
        self.users_by_name.insert(user.name.clone(), user);
    }

    pub fn remove_user(&mut self, user: User) {
        // let index = self.users.
        self.users_by_name.remove(&user.name);
    }

    pub fn update_user(&mut self, user: User) {
        self.add_user(user);
    }

    pub fn add_users(&mut self, users: Vec<User>) {
        for user in users {
            self.add_user(user);
        }
    }

    pub fn users<'a>(&'a self) -> Vec<&'a User> {
        let v = self.users_by_name.values().collect::<Vec<_>>();
        v
    }

    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn push_messages(&mut self, messages: Vec<Message>) {
        self.messages.extend(messages.into_iter());
    }
}

// static MESSAGES: Mutex<Vec<Message>> = Mutex::const_new(Vec::new());

// static USERS: Mutex<Vec<User>> = Mutex::const_new(Vec::new());

pub struct Message {
    // pub nick: &'a User<'a>,
    pub username: String,
    pub body: String,
    pub timestamp: usize,
}

pub struct User {
    pub name: String,
    pub color: &'static str,
    pub labels: Vec<&'static str>,
    pub icons: Vec<&'static str>,
}
