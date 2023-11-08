use crate::data::inbound::FlairData;
use reqwest::get;
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::OnceCell;

const DEFAULT_COLOR: &str = "#DCDCDC";

const GYM_ICON: &str = "󰇦";
const NFL_ICON: &str = "󰉝";
const BROADCASTER_ICON: &str = "󱜡";
const SUBSCRIBER_ICON: &str = "[D]";
const YOUTUBE_ICON: &str = "󰗃";
const BIRTHDAY_ICON: &str = "";
const BOT_ICON: &str = "󰚩";
const LAWYER_ICON: &str = "󰊛";

static FLAIRS: OnceCell<Flairs> = OnceCell::const_new();

pub async fn flairs() -> &'static Flairs {
    FLAIRS.get_or_init(Flairs::new).await
}

pub struct Flairs {
    flairs_by_label: HashMap<String, Flair>,
    icons_by_name: HashMap<&'static str, &'static str>,
}

impl Flairs {
    async fn new() -> Flairs {
        Flairs {
            flairs_by_label: retrieve_flairs().await.expect("Retrieval of Flairs failed"),
            icons_by_name: init_icon_table(),
        }
    }

    pub fn get(&self, name: &String) -> Option<&Flair> {
        self.flairs_by_label.get(name)
    }

    pub fn get_icon(&self, name: &str) -> Option<&str> {
        Some(*self.icons_by_name.get(name)?)
        // match self.icons_by_name.get(name) {
        //     Some(&s) => Some(s),
        //     None => None,
        // }
    }
}

pub async fn retrieve_flairs() -> Result<HashMap<String, Flair>, Box<dyn Error>> {
    let url = "https://cdn.destiny.gg/2.83.1/flairs/flairs.json";
    let data = get(url).await?.text().await?;
    let entities: Vec<FlairData> = serde_json::from_slice(data.as_bytes())?;
    let map = entities
        .into_iter()
        .map(|e| {
            (
                e.name,
                Flair::new(e.label, e.color, e.rainbow_color, e.priority),
            )
        })
        .collect();
    Ok(map)
}

pub fn init_icon_table() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("subscriber", SUBSCRIBER_ICON),
        ("flair7", NFL_ICON),
        ("flair12", BROADCASTER_ICON),
        ("bot", BOT_ICON),
        ("flair29", GYM_ICON),
        ("flair28", LAWYER_ICON),
        ("flair25", YOUTUBE_ICON),
        ("flair21", YOUTUBE_ICON),
        ("flair15", BIRTHDAY_ICON),
        ("flair15", BIRTHDAY_ICON),
    ])
}

pub struct Flair {
    pub label: String,
    color: String,
    rainbow: bool,
    priority: i32,
}

impl Flair {
    pub fn new(label: String, color: String, rainbow: bool, priority: i32) -> Flair {
        Flair {
            label,
            color,
            rainbow,
            priority,
        }
    }
}

pub fn color<'a>(flairs: &Vec<&'a Flair>) -> &'a str {
    let flair = flairs
        .iter()
        .filter(|f| !f.color.is_empty())
        .max_by(|a, b| b.priority.cmp(&a.priority));
    match flair {
        Some(f) => &f.color,
        None => DEFAULT_COLOR,
    }
}
