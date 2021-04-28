use crate::connection::Connection;
use crate::icons::Icon;
use crate::theme::Theme;
use crate::util::{duration_fmt, CompTrack};
use crate::Message;
use iced::{Align, Button, Command, Container, Element, Image, Length, Row, Slider, Space, Text};
use reciprocity_communication::messages::PlayerControl as ControlRequest;
use reciprocity_communication::messages::{PlayMode, PlayerState};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum PlayerControlMessage {
    SongImageUpdated(Option<CompTrack>),
    PlayerStateChanged(Option<PlayerState>),
    ButtonPressed(ButtonEvent),
    PosSliderChanged(f32),
    PosSliderReleased(),
    ContinuousPosUpdate(Instant),
}

#[derive(Debug, Clone)]
pub enum ButtonEvent {
    Prev,
    PlayPause,
    Next,
    Repeat,
}

#[derive(Debug)]
pub struct PlayerControl {
    cur_song: Option<CompTrack>,
    player_state: Option<PlayerState>,
    user_sliding: bool,
    slider_pos: f32,

    song_pos_slider: iced::slider::State,
    prev_button_state: iced::button::State,
    play_pause_button_state: iced::button::State,
    next_button_state: iced::button::State,
    repeat_button_state: iced::button::State,
}

impl PlayerControl {
    pub fn new() -> Self {
        //TODO
        PlayerControl {
            cur_song: None,
            player_state: None,
            user_sliding: false,
            slider_pos: 0.0,
            song_pos_slider: Default::default(),
            prev_button_state: Default::default(),
            play_pause_button_state: Default::default(),
            next_button_state: Default::default(),
            repeat_button_state: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        con: &Option<Connection>,
        message: PlayerControlMessage,
    ) -> Command<Message> {
        match message {
            PlayerControlMessage::PosSliderChanged(x) => {
                if self.cur_song.is_some() {
                    self.user_sliding = true;
                    self.slider_pos = x;
                }
            }
            PlayerControlMessage::PosSliderReleased() => {
                if self.cur_song.is_some() {
                    let _target = self.slider_pos;
                    self.user_sliding = false;
                }

                if let Some(con) = con {
                    if let Some(track) = self.cur_song.as_ref() {
                        return con.control_request(
                            reciprocity_communication::messages::PlayerControl::SetTime(
                                track.pos(self.slider_pos),
                            ),
                        );
                    }
                }
            }
            PlayerControlMessage::PlayerStateChanged(state) => {
                self.player_state = state;

                let new = self
                    .player_state
                    .as_ref()
                    .map(|s| s.current.clone())
                    .flatten();
                //If bot are Some
                if let (Some(new), Some(old)) = (new.clone(), self.cur_song.as_mut()) {
                    if new.uri.eq(&old.track.uri) {
                        old.track = new;
                        old.updated = Instant::now();
                    } else {
                        self.cur_song = CompTrack {
                            track: new,
                            updated: Instant::now(),
                            image: None,
                        }
                        .into();
                    }
                } else if let Some(new) = new {
                    self.cur_song = CompTrack {
                        track: new,
                        updated: Instant::now(),
                        image: None,
                    }
                    .into();
                } else {
                    self.cur_song = None;
                }

                if !self.user_sliding {
                    self.slider_pos = self
                        .cur_song
                        .as_ref()
                        .map(|t| t.pos_percentage())
                        .unwrap_or(0.0);
                }
                if let Some(n) = self.cur_song.as_ref() {
                    let update_cmd = get_update_command(n.updated);

                    if n.image.is_none() {
                        let download_cmd = Command::perform(n.clone().download_image(), |res| {
                            Message::PlayerControl(PlayerControlMessage::SongImageUpdated(
                                res.map(Some).map_err(|e| println!("{}", e)).unwrap_or(None),
                            ))
                        });
                        return Command::batch(vec![update_cmd, download_cmd]);
                    }
                    return update_cmd;
                }
            }
            PlayerControlMessage::ButtonPressed(b) => {
                if let Some(state) = self.player_state.as_ref() {
                    if let Some(con) = con {
                        let req = match b {
                            ButtonEvent::Prev => ControlRequest::BackSkip(1),
                            ButtonEvent::PlayPause => match state.paused {
                                true => ControlRequest::Resume(),
                                false => ControlRequest::Pause(),
                            },
                            ButtonEvent::Next => ControlRequest::Skip(1),
                            ButtonEvent::Repeat => match state.mode {
                                PlayMode::Normal => ControlRequest::PlayMode(PlayMode::LoopAll),
                                PlayMode::LoopAll => ControlRequest::PlayMode(PlayMode::LoopOne),
                                PlayMode::LoopOne => ControlRequest::PlayMode(PlayMode::Normal),
                            },
                        };
                        return con.control_request(req);
                    }
                }
            }
            PlayerControlMessage::SongImageUpdated(t) => {
                if let (Some(img_track), Some(cur_track)) = (t.as_ref(), self.cur_song.as_mut()) {
                    if img_track.track.uri.eq(&cur_track.track.uri) {
                        cur_track.image = img_track.image.clone();
                    }
                }
            }
            PlayerControlMessage::ContinuousPosUpdate(when) => {
                if let Some(state) = self.player_state.as_ref() {
                    if !state.paused {
                        if let Some(song) = self.cur_song.as_ref() {
                            if song.updated.eq(&when) {
                                if !self.user_sliding {
                                    self.slider_pos = self
                                        .cur_song
                                        .as_ref()
                                        .map(|t| t.pos_percentage())
                                        .unwrap_or(0.0);
                                }

                                return get_update_command(when);
                            }
                        }
                    }
                }
            }
        };

        Command::none()
    }

    pub fn view(&mut self, theme: &Theme) -> (Element<'_, Message>, Element<'_, Message>) {
        let img = self
            .cur_song
            .as_ref()
            .map(|t| t.image.clone())
            .flatten()
            .map(|h| Image::new(h).width(Length::Units(200)))
            .map(Element::new)
            .unwrap_or_else(|| Element::new(Space::new(Length::Fill, Length::Shrink)));
        let song_picture = Container::new(img)
            .align_x(Align::Center)
            .align_y(Align::End)
            .max_height(150)
            .width(Length::Fill);

        let song_title = self
            .cur_song
            .as_ref()
            .map(|t| t.track.title.as_str())
            .unwrap_or("No Song");
        let song_pos = self
            .cur_song
            .as_ref()
            .map(|t| duration_fmt(&t.pos(self.slider_pos)))
            .unwrap_or_else(|| String::from("-:--"));
        let song_len = self
            .cur_song
            .as_ref()
            .map(|t| duration_fmt(&t.track.len))
            .unwrap_or_else(|| String::from("-:--"));

        let prev_btn = Button::new(
            &mut self.prev_button_state,
            Icon::SkipPrevious.get_svg(theme),
        )
        .on_press(Message::PlayerControl(PlayerControlMessage::ButtonPressed(
            ButtonEvent::Prev,
        )))
        .style(theme.control_button_theme());
        let play_pause_icon: Icon = self.player_state.as_ref().map(|s| !s.paused).into();
        let play_pause_btn = Button::new(
            &mut self.play_pause_button_state,
            play_pause_icon.get_svg(theme),
        )
        .on_press(Message::PlayerControl(PlayerControlMessage::ButtonPressed(
            ButtonEvent::PlayPause,
        )))
        .style(theme.control_button_theme());
        let next_btn = Button::new(&mut self.next_button_state, Icon::SkipNext.get_svg(theme))
            .on_press(Message::PlayerControl(PlayerControlMessage::ButtonPressed(
                ButtonEvent::Next,
            )))
            .style(theme.control_button_theme());
        let repeat_icon: Icon = self.player_state.as_ref().map(|s| s.mode.clone()).into();
        let repeat_btn = Button::new(&mut self.repeat_button_state, repeat_icon.get_svg(theme))
            .on_press(Message::PlayerControl(PlayerControlMessage::ButtonPressed(
                ButtonEvent::Repeat,
            )))
            .style(theme.control_button_theme());

        let mut row = Row::new();
        row = row
            .push(
                Container::new(Text::new(song_title).size(16).color(theme.text_color()))
                    .width(Length::Units(190)),
            )
            .push(prev_btn)
            .push(play_pause_btn)
            .push(next_btn)
            .push(repeat_btn)
            //.push(Space::new(Length::Units(15), Length::Fill))
            .push(
                Container::new(Text::new(song_pos).size(16).color(theme.text_color()))
                    .width(Length::Units(70))
                    .align_x(Align::End),
            )
            .push(Space::new(Length::Units(10), Length::Fill))
            .height(Length::Units(35));

        let slid = Slider::new(
            &mut self.song_pos_slider,
            0.0..=100.0,
            self.slider_pos,
            |x| Message::PlayerControl(PlayerControlMessage::PosSliderChanged(x)),
        )
        .step(0.1)
        .on_release(Message::PlayerControl(
            PlayerControlMessage::PosSliderReleased(),
        ))
        .style(theme.song_slider_theme());
        row = row
            .push(slid)
            .push(Space::new(Length::Units(10), Length::Fill))
            .push(
                Container::new(Text::new(song_len).size(16).color(theme.text_color()))
                    .width(Length::Units(70))
                    .align_x(Align::Start),
            )
            .align_items(Align::Center);
        (song_picture.into(), Container::new(row).padding(5).into())
    }
}

fn get_update_command(when: Instant) -> Command<Message> {
    Command::perform(
        async { tokio::time::sleep(Duration::from_millis(500)).await },
        move |_| Message::PlayerControl(PlayerControlMessage::ContinuousPosUpdate(when)),
    )
}
