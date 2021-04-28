use crate::icons::Icon;
use crate::tabs::Tab;
use crate::theme::Theme;
use crate::util::duration_fmt;
use crate::Connection;
use crate::{Message, MAX_DOUBLE_CLICK_INTERVAL};
use iced::{
    Button, Column, Command, Element, HorizontalAlignment, Length, Row, Scrollable, Space, Text,
};
use reciprocity_communication::messages::{PlayerControl, PlayerState, Track};
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum PlaylistMessage {
    PlayerStateChanged(Option<PlayerState>),
    SongClicked(usize),
}

#[derive(Debug)]
pub struct PlaylistTab {
    playlist: Vec<Track>,
    scroll: iced::scrollable::State,
    last_click: (usize, Instant),
    btn_states: Vec<iced::button::State>,
}

impl PlaylistTab {
    pub fn new() -> Self {
        //TODO
        PlaylistTab {
            playlist: Vec::new(),
            scroll: Default::default(),
            last_click: (0, Instant::now()),
            btn_states: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        con: &Option<Connection>,
        message: PlaylistMessage,
    ) -> Command<Message> {
        match message {
            PlaylistMessage::PlayerStateChanged(state) => {
                self.playlist = state.map(|s| s.queue).unwrap_or_default();
            }
            PlaylistMessage::SongClicked(i) => {
                if self.last_click.0 == i
                    && self.last_click.1.elapsed() <= MAX_DOUBLE_CLICK_INTERVAL
                {
                    self.last_click = (0, Instant::now());
                    if let Some(con) = con {
                        return con.control_request(PlayerControl::Skip(i));
                    }
                }
                self.last_click = (i, Instant::now());
            }
        }

        Command::none()
    }
}

impl Tab for PlaylistTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Playlist")
    }

    fn tab_label(&self) -> (Option<Icon>, String) {
        (Icon::QueueMusic.into(), "Playlist".to_string())
    }

    fn content(&mut self, theme: &Theme) -> Element<'_, Self::Message> {
        let mut column = Column::new().width(Length::Fill);

        while self.btn_states.len() <= self.playlist.len() {
            self.btn_states.push(Default::default());
        }

        for (i, (track, btn_state)) in self
            .playlist
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
                .on_press(Message::Playlist(PlaylistMessage::SongClicked(i)))
                .style(theme.tab_button_theme());

            column = column.push(btn);
        }

        Scrollable::new(&mut self.scroll)
            .push(column)
            .height(Length::Fill)
            .into()
    }
}
