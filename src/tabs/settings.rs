use crate::icons::Icon;
use crate::tabs::Tab;
use crate::theme::Theme;
use crate::Message;
use iced::{Column, Command, Element, Length, Radio, Scrollable, Text};

#[derive(Debug, Clone)]
pub enum SettingsMessage {}

#[derive(Debug)]
pub struct SettingsTab {
    scroll: iced::scrollable::State,
}

impl SettingsTab {
    pub fn new() -> Self {
        //TODO
        SettingsTab {
            scroll: Default::default(),
        }
    }

    pub fn update(&mut self, _message: SettingsMessage) -> Command<Message> {
        todo!()
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> (Option<Icon>, String) {
        (Icon::Settings.into(), "Settings".to_string())
    }

    fn content(&mut self, theme: &Theme) -> Element<'_, Self::Message> {
        //TODO
        let mut column = Column::new()
            .spacing(10)
            .push(Text::new("Theme").size(26).color(theme.text_color()));

        for t in Theme::ALL.iter() {
            column = column.push(
                Radio::new(*t, &format!("{:?}", t), Some(*theme), Message::ThemeChanged)
                    .style(theme.radio_button_theme()),
            );
        }

        Scrollable::new(&mut self.scroll)
            .push(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
