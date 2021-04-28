use crate::theme::Theme;
use crate::Message;
use iced::{Command, Container, Element, Row, Rule, Text};

#[derive(Debug, Clone)]
pub enum FooterMessage {
    UpdateUser(Option<String>),
    UpdateChannel(Option<String>),
}

#[derive(Debug)]
pub struct PlayerFooter {
    user: Option<String>,
    voice_channel: Option<String>,
}

impl PlayerFooter {
    pub fn new() -> Self {
        //TODO
        PlayerFooter {
            user: None,
            voice_channel: None,
        }
    }

    pub fn update(&mut self, message: FooterMessage) -> Command<Message> {
        match message {
            FooterMessage::UpdateUser(u) => self.user = u,
            FooterMessage::UpdateChannel(c) => self.voice_channel = c,
        }

        Command::none()
    }

    pub fn view(&mut self, theme: &Theme) -> Element<'_, Message> {
        let mut row = Row::new();

        let con = match self.user.is_some() {
            true => "Connected",
            false => "Not Connected",
        };

        let user = match &self.user {
            None => Text::new("User: None").size(12).color(theme.text_color()),
            Some(user) => Text::new(format!("User: {}", user))
                .size(12)
                .color(theme.text_color()),
        };

        let channel = match &self.voice_channel {
            None => String::from("Channel: Not Connected"),
            Some(ch) => format!("Channel: {}", ch),
        };

        row = row
            .push(Container::new(
                Text::new(con).size(12).color(theme.text_color()),
            ))
            .push(Rule::vertical(10))
            .push(user)
            .push(Rule::vertical(10))
            .push(Container::new(
                Text::new(channel).size(12).color(theme.text_color()),
            ))
            .max_height(12);
        Container::new(row).padding(3).into()
    }
}
