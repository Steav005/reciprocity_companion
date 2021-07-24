#![allow(dead_code)]

pub mod config;
mod connection;
mod executor;
mod footer;
pub mod icons;
mod player_control;
mod states;
mod tabs;
mod theme;
pub mod util;
mod log;

use crate::config::Config;
use crate::connection::{Connection, ConnectionError};
use crate::footer::{FooterMessage, PlayerFooter};
use crate::player_control::{PlayerControl, PlayerControlMessage};
use crate::tabs::history::{HistoryMessage, HistoryTab};
use crate::tabs::playlist::{PlaylistMessage, PlaylistTab};
use crate::tabs::search::{SearchMessage, SearchTab};
use crate::tabs::settings::{SettingsMessage, SettingsTab};
use crate::tabs::{Tab, Tabs};
use crate::theme::Theme;
use iced::{Application, Clipboard, Column, Command, Container, Element, Length, Row};
use reciprocity_communication::client::{get_auth_code, OAuthError};
use reciprocity_communication::messages::oauth2::{AuthorizationCode, RefreshToken};
use reciprocity_communication::messages::{Auth, User, PlayerControlResult};
use reciprocity_communication::messages::{Message as ComMessage, PlayerState, State};
use std::ops::Deref;
use std::path::PathBuf;
use std::time::Duration;
use crate::log::LogMessage;

pub const MAX_DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(300);

#[derive(Debug, Clone)]
pub enum Message {
    None(),
    GotAuth(Result<AuthorizationCode, OAuthError>),
    GotConnection(Result<(Connection, (User, RefreshToken)), ConnectionError>),
    ReceiveBotMessage(Result<ComMessage, ConnectionError>),

    PlayerControl(PlayerControlMessage),
    Footer(FooterMessage),

    Playlist(PlaylistMessage),
    History(HistoryMessage),
    Search(SearchMessage),
    Settings(SettingsMessage),

    ThemeChanged(Theme),
    TabSelected(usize),
}

#[derive(Debug)]
pub struct Companion {
    cfg: Config,
    cfg_path: PathBuf,
    theme: theme::Theme,
    connection: Option<Connection>,
    player_state: Option<PlayerState>,

    app_log: Vec<LogMessage>,
    control_log: Vec<PlayerControlResult>,

    player_control: PlayerControl,
    footer: PlayerFooter,

    tabs: Tabs<Message, 4>,
    playlist_tab: PlaylistTab,
    history_tab: HistoryTab,
    search_tab: SearchTab,
    settings_tab: SettingsTab,
}

impl Application for Companion {
    type Executor = executor::TokioExecutor;
    type Message = Message;
    type Flags = (Config, PathBuf);

    fn new((cfg, config_path): (Config, PathBuf)) -> (Self, Command<Self::Message>) {
        println!(
            "Config refresh token: {:?}",
            cfg.refresh_token.as_ref().map(|t| t.secret().clone())
        );
        let command = if let Some(token) = cfg.refresh_token.as_ref() {
            println!("Performing Connect with Refresh Token");
            Command::perform(
                Connection::new(Auth::Token(token.clone()), cfg.bot_link.clone()),
                Message::GotConnection,
            )
        } else {
            println!("Getting New Auth Code: Main");
            Command::perform(get_auth_code(cfg.com.clone()), Message::GotAuth)
        };

        (
            Companion {
                cfg: cfg.clone(),
                cfg_path: config_path,
                theme: cfg.theme,
                connection: None,
                player_state: None,
                app_log: Vec::default(),
                control_log: Vec::default(),
                player_control: PlayerControl::new(),
                footer: PlayerFooter::new(),
                tabs: Tabs::new(0, Message::TabSelected),
                playlist_tab: PlaylistTab::new(),
                history_tab: HistoryTab::new(),
                search_tab: SearchTab::new(),
                settings_tab: SettingsTab::new(),
            },
            command,
        )
    }

    fn title(&self) -> String {
        "Reciprocity Companion".to_string()
    }

    fn update(&mut self, message: Self::Message, _c: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::None() => Command::none(),
            Message::GotAuth(res) => match res {
                Ok(code) => Command::perform(
                    Connection::new(Auth::Code(code), self.cfg.bot_link.clone()),
                    Message::GotConnection,
                ),
                Err(e) => panic!("{:?}", e),
            },
            Message::PlayerControl(message) => {
                self.player_control.update(&self.connection, message)
            }
            Message::Footer(message) => self.footer.update(message),
            Message::Playlist(message) => self.playlist_tab.update(&self.connection, message),
            Message::History(message) => self.history_tab.update(&self.connection, message),
            Message::Search(message) => self.search_tab.update(&self.connection, message),
            Message::Settings(message) => self.settings_tab.update(message),
            Message::TabSelected(selected) => self.tabs.update(selected),
            Message::ReceiveBotMessage(res) => {
                println!("{:?}", res);
                let msg = match res {
                    Ok(msg) => msg,
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                };
                let mut commands = Vec::new();
                match &msg {
                    ComMessage::PlayerState(state) => {
                        if let Some(state) = state {
                            match state {
                                State::FullState(full) => {
                                    self.player_state = Some(full.deref().clone())
                                }
                                State::UpdateState(patch) => {
                                    println!("Patch Length: {} Bytes", patch.len());
                                    msg.patch_player_state(
                                        &mut self.player_state.as_mut().unwrap(),
                                    )
                                    .ok();
                                }
                                State::EmptyState() => {
                                    self.player_state = None;
                                }
                            }
                        } else {
                            self.player_state = None;
                        }

                        commands.push(self.player_control.update(
                            &self.connection,
                            PlayerControlMessage::PlayerStateChanged(self.player_state.clone()),
                        ));
                        commands.push(self.playlist_tab.update(
                            &self.connection,
                            PlaylistMessage::PlayerStateChanged(self.player_state.clone()),
                        ));
                        commands.push(self.history_tab.update(
                            &self.connection,
                            HistoryMessage::PlayerStateChanged(self.player_state.clone()),
                        ));
                        //TODO Send to all who are interested
                    }
                    ComMessage::UserVoiceState(voice) => {
                        //TODO Send to all who are interested

                        commands.push(self.footer.update(FooterMessage::UpdateChannel(
                            voice.as_ref().map(|v| v.channel_name.clone()),
                        )))
                    }
                    _ => {}
                }
                commands.push(
                    self.connection
                        .as_ref()
                        .expect("Connection is empty")
                        .get_rec_cmd(),
                );
                Command::batch(commands)
            }
            Message::GotConnection(res) => {
                match res {
                    Ok((con, (user, token))) => {
                        self.connection = Some(con);
                        self.cfg.refresh_token = Some(token);
                        self.cfg.update(self.cfg_path.clone());
                        let footer_cmd = self
                            .footer
                            .update(FooterMessage::UpdateUser(Some(user.username)));
                        //Start Receive Chain
                        let rec_cmd = self.connection.as_ref().unwrap().get_rec_cmd();
                        Command::batch(vec![footer_cmd, rec_cmd])
                    }
                    Err(e) => {
                        match e {
                            ConnectionError::NonAuthMessage(e) => {
                                println!("Auth Error {:?}", e);
                                //Clear Token in Config
                                self.cfg.refresh_token = None;
                                self.cfg.update(self.cfg_path.clone());
                                //Attempt getting new Token
                                println!("Getting New Auth Code: Got Connection");
                                Command::perform(
                                    get_auth_code(self.cfg.com.clone()),
                                    Message::GotAuth,
                                )
                            }
                            _ => {
                                panic!("Error connecting: {:?}", e);
                            }
                        }
                    }
                }
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                self.cfg.theme = theme;
                self.cfg.update(self.cfg_path.clone());

                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        //Container::new(Text::new(""));
        //Column::new();
        //TODO

        let (tabs, tab_view) = self.tabs.view(
            [
                self.playlist_tab.borrowed(),
                self.history_tab.borrowed(),
                self.search_tab.borrowed(),
                self.settings_tab.borrowed(),
            ],
            &self.theme,
        );
        let (song_picture, control) = self.player_control.view(&self.theme);
        let control = Container::new(control).style(self.theme.control_container_theme());

        let tabs_song_column = Column::new()
            .push(tabs)
            .push(song_picture)
            .height(Length::Fill)
            .width(Length::Units(200));
        let tabs_song_column = Container::new(tabs_song_column)
            .height(Length::Fill)
            .width(Length::Units(200))
            .style(self.theme.tab_container_theme());
        let footer = Container::new(self.footer.view(&self.theme))
            .width(Length::Fill)
            .style(self.theme.footer_container_theme());

        let combined_row = Row::new()
            .push(tabs_song_column)
            //.push(Rule::vertical(1))
            .push(tab_view)
            .width(Length::Fill)
            .height(Length::Fill);

        let col = Column::new()
            .push(combined_row)
            //.push(Rule::horizontal(1))
            .push(control)
            //.push(Rule::horizontal(1))
            .push(footer);

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
