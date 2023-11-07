use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageData {
    pub nick: String,
    pub features: Vec<String>,
    pub timestamp: usize,
    pub data: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlairData {
    pub label: String,
    pub name: String,
    pub hidden: bool,
    pub priority: i32,
    pub color: String,
    pub rainbow_color: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub nick: String,
    pub created_date: String,
    pub features: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UsersData {
    pub users: Vec<UserData>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceData {
    pub nick: String,
    pub created_date: String,
    pub features: Vec<String>,
    pub timestamp: usize,
}

pub struct Err {
    // typically will say banned if banned
    pub description: String,
}