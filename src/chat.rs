use crate::history;
use bounded_vec_deque::BoundedVecDeque;
use std::collections::HashMap;

//Keep this to one less than a power of 2
//for maximum efficiency
const MESSAGE_COUNT_BOUND: usize = 511;

pub struct Chat {
    pub should_quit: bool,
    pub messages: BoundedVecDeque<Message>,
    // pub users: Vec<User>,
    pub users_by_name: HashMap<String, User>,
}

impl Chat {
    pub async fn new() -> Chat {
        let history = match history::get_history().await {
            Ok(ms) => ms,
            Err(_) => Vec::new(),
        };
        let messages = BoundedVecDeque::from_iter(history.into_iter(), MESSAGE_COUNT_BOUND);
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
        self.messages.push_back(message);
    }

    pub fn push_messages(&mut self, messages: Vec<Message>) {
        //I don't actually know if this works, the ui updates too fast
        //but I think the logic makes sense
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
    pub color: (u8, u8, u8),
    pub labels: Vec<&'static str>,
    pub icons: Vec<&'static str>,
}
