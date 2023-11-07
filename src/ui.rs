use crate::app::App;
use crate::chat::{Chat, Message, User};
use colors_transform::Color as TColor;
use colors_transform::Rgb;
use ratatui::style::Stylize;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Corner, Layout, Rect},
    style::{Color, Style},
    // symbols,
    text::{Line, Span, Text},
    // widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Frame, //Terminal,
};
use textwrap::{wrap, Options};

pub fn draw<B: Backend>(app: &App, frame: &mut Frame<B>) {
    let tab_view = TabView::new(vec![String::from("Chat"), String::from("Users")]);
    tab_view.draw(&app.chat, frame, frame.size());
}

// impl<'a, B> Drawable<'a, B> for Ui<'a>
// where
//     B: Backend,
// {
//     fn draw(&self, chat: &Chat, frame: &mut Frame<B>, area: Rect) {
//         self.tab_view.draw(chat, frame, area)
//     }
// }

// pub trait Drawable<'a, B: Backend> {
//     fn draw(&self, chat: &Chat, frame: &mut Frame<B>, area: Rect);
// }

struct TabView {
    titles: Vec<String>,
    index: usize,
    size: usize,
}

impl<'a> TabView {
    fn new(titles: Vec<String>) -> TabView {
        let size = titles.len();
        TabView {
            titles,
            index: 0,
            size,
        }
    }

    fn next(&mut self) {
        self.index = (self.index + 1) % self.size;
    }

    fn prev(&mut self) {
        self.index = (self.index + self.size - 1) % self.size;
    }

    fn draw_chat_tab<B: Backend>(&self, chat: &Chat, frame: &mut Frame<B>, area: Rect) {
        let messages = &chat.messages;
        let rows = messages
            .iter()
            .rev()
            .map(|m| {
                let card = MessageCard::new(m, chat, area.width);
                card.into()
            })
            .collect::<Vec<_>>();
        let message_list = List::new(rows)
            .block(Block::default().borders(Borders::ALL))
            .start_corner(Corner::BottomLeft);
        frame.render_widget(message_list, area)
    }

    fn draw_users_tab<B: Backend>(&self, chat: &Chat, frame: &mut Frame<B>, area: Rect) {
        let users = &chat.users();
        let cards = users
            .iter()
            .rev()
            .map(|u| {
                let card = UserCard::new(u);
                card.into()
            })
            .collect::<Vec<_>>();
        let user_cards_list = List::new(cards)
            .block(Block::default().borders(Borders::ALL))
            .start_corner(Corner::BottomLeft);
        frame.render_widget(user_cards_list, area)
    }

    fn get_tabs(titles: &'a Vec<String>) -> Tabs<'a> {
        let spans = titles
            .iter()
            .map(|t| Line::from(Span::styled(t, Style::default().fg(Color::Green))))
            .collect();

        let tabs = Tabs::new(spans)
            .block(Block::default().borders(Borders::ALL).title("A title"))
            .select(0);
        tabs
    }

    fn draw<B: Backend>(&self, chat: &Chat, frame: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);
        let tabs = Self::get_tabs(&self.titles);
        frame.render_widget(tabs, chunks[0]);
        match self.index {
            0 => self.draw_chat_tab(chat, frame, chunks[1]),
            1 => self.draw_users_tab(chat, frame, chunks[1]),
            _ => (),
        }
    }
}

struct UserText {
    name: String,
    color: Color,
}

impl UserText {
    fn new(user: &User) -> UserText {
        let (r, g, b) = match Rgb::from_hex_str(user.color) {
            Ok(rgb) => rgb.as_tuple(),
            Err(_) => (0.0, 0.0, 0.0),
        };
        let color = Color::Rgb(r as u8, g as u8, b as u8);
        let icon_text = String::from_iter(user.icons.iter().rev().map(|&s| s));
        let full_text = format!(
            "{icons} {name}",
            icons = icon_text,
            name = user.name.clone()
        );
        UserText {
            name: full_text,
            color,
        }
    }

    fn default(name: String) -> UserText {
        let color = Color::DarkGray;
        UserText { name, color }
    }
}

impl From<UserText> for Span<'_> {
    fn from(value: UserText) -> Self {
        let style = Style::default().bold().fg(value.color);
        // let text = format!("{0}({1})", value.name, value.hex_color);
        Self::styled(value.name, style)
    }
}

impl From<UserText> for Line<'_> {
    fn from(value: UserText) -> Self {
        let span = Span::from(value);
        Self::from(span)
    }
}

impl From<UserText> for Text<'_> {
    fn from(value: UserText) -> Self {
        let span = Span::from(value);
        Self::from(span)
    }
}

struct MessageCard {
    // message: &Message,
    user_text: UserText,
    body: String,
    width: u16,
}

impl MessageCard {
    fn new(message: &Message, chat: &Chat, width: u16) -> Self {
        let name = message.username.clone();
        let user = chat.find_user(&name);
        let user_text = match user {
            Some(u) => UserText::new(u),
            None => UserText::default(name),
        };
        MessageCard {
            user_text,
            body: message.body.clone(),
            width,
        }
    }
}

// impl<'a> Listable<'a> for MessageCard {
//     fn itemize(self, chat: &Chat) -> ListItem<'a> {
//         let name = self.username;
//         let user = chat.find_user(&name);
//         let user_text = match user {
//             Some(u) => UserText::new(u),
//             None => UserText::default(name.clone()),
//         };
//         // let body_span = Span::from(self.body);
//         // let body = wrap(&self.body, width as usize);
//         let spans = Line::from(vec![
//             user_text.to_span(),
//             Span::raw(": "), /*body_span*/
//         ]);
//         ListItem::new(spans)
//     }
// }

impl From<MessageCard> for ListItem<'_> {
    fn from(value: MessageCard) -> Self {
        let user_width = value.user_text.name.len() + 1;
        let indent = &String::from_iter((0..=user_width).map(|_| " "));
        let options = Options::new(value.width as usize - user_width).subsequent_indent(indent);
        let wrap = wrap(value.body.as_str(), options);
        // TODO: There must be a better way to do this:
        let body = wrap.iter().map(|c| c.to_string()).collect::<Vec<_>>();
        let (first, rest) = body.split_first().unwrap();
        let mut text = Text::from(Line::from(vec![
            value.user_text.into(),
            Span::from(": "),
            Span::from(first.to_owned()),
        ]));
        text.extend(rest.iter().map(|l| Span::raw(l.to_owned())));

        ListItem::new(text)
    }
}

struct UserCard<'a> {
    user: &'a User,
}

impl<'a> UserCard<'a> {
    fn new(user: &'a User) -> Self {
        UserCard { user }
    }
}

impl<'a> From<UserCard<'a>> for ListItem<'a> {
    fn from(value: UserCard) -> Self {
        let user_text = UserText::new(value.user);
        let labels = &value.user.labels;
        let spans = labels
            .iter()
            .map(|&l| Span::raw(format!("   {}", l)))
            .collect::<Vec<_>>();
        let label_spans = Line::from(spans);
        let spans = Line::from(user_text);
        ListItem::new(vec![spans, label_spans])
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn new_fill(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn new() -> StatefulList<T> {
        Self::new_fill(Vec::new())
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
