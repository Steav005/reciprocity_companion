use crate::connection::Connection;
use crate::icons::Icon;
use crate::tabs::Tab;
use crate::theme::Theme;
use crate::util::duration_fmt;
use crate::{Message, MAX_DOUBLE_CLICK_INTERVAL};
use iced::{
    Button, Column, Command, Element, HorizontalAlignment, Length, Row, Scrollable, Space, Text,
};
use reciprocity_communication::messages::{PlayerControl, PlayerState, Track};
use std::time::Instant;
use reqwest::Url;

#[derive(Debug, Clone)]
pub enum HistoryMessage {
    PlayerStateChanged(Option<PlayerState>),
    SongClicked(Track),
}

#[derive(Debug)]
pub struct HistoryTab {
    history: Vec<Track>,
    scroll: iced::scrollable::State,
    last_click: (Track, Instant),
    btn_states: Vec<iced::button::State>,
}

impl HistoryTab {
    pub fn new() -> Self {
        //TODO
        HistoryTab {
            history: Vec::new(),
            scroll: Default::default(),
            last_click: (Track{
                len: Default::default(),
                pos: Default::default(),
                title: "".to_string(),
                uri: "".to_string()
            }, Instant::now()),
            btn_states: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        con: &Option<Connection>,
        message: HistoryMessage,
    ) -> Command<Message> {
        match message {
            HistoryMessage::PlayerStateChanged(state) => {
                self.history = state.map(|s| s.history).unwrap_or_default();
            }
            HistoryMessage::SongClicked(track) => {
                if self.last_click.0.eq(&track)
                    && self.last_click.1.elapsed() <= MAX_DOUBLE_CLICK_INTERVAL
                {
                    self.last_click = (track, Instant::now());
                    if let Some(con) = con {
                        return con.control_request(PlayerControl::Enqueue(Url::parse(self.last_click.0.uri.as_str()).unwrap()));
                    }
                } else {
                    self.last_click = (track, Instant::now());
                }
            }
        }

        Command::none()
    }
}

impl Tab for HistoryTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("History")
    }

    fn tab_label(&self) -> (Option<Icon>, String) {
        (Icon::History.into(), "History".to_string())
    }

    fn content(&mut self, theme: &Theme) -> Element<'_, Self::Message> {
        let mut column = Column::new().width(Length::Fill);

        while self.btn_states.len() <= self.history.len() {
            self.btn_states.push(Default::default());
        }

        for (i, (track, btn_state)) in self
            .history
            .iter()
            .zip(self.btn_states.iter_mut())
            .enumerate()
        {
            let i = i + 1;
            let row = Row::new()
                .push(Text::new(format!("{}.", i)).width(Length::Units(30)))
                .push(Space::new(Length::Units(10), Length::Shrink))
                .push(
                    Text::new(track.title.clone())
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Left),
                )
                .push(Space::new(Length::Units(10), Length::Shrink))
                .push(Text::new(duration_fmt(&track.len)))
                .push(Space::new(Length::Units(15), Length::Shrink))
                .width(Length::Fill);
            let btn = Button::new(btn_state, row)
                .on_press(Message::History(HistoryMessage::SongClicked(track.clone())))
                .style(theme.tab_button_theme());

            column = column.push(btn);
        }

        Scrollable::new(&mut self.scroll)
            .push(column)
            .height(Length::Fill)
            .into()
    }
}
