use crate::connection::Connection;
use crate::icons::Icon;
use crate::tabs::Tab;
use crate::theme::Theme;
use crate::util::youtube::{search, Video};
use crate::util::{get_image, get_image_uri_from_yt_id};
use crate::{Message, MAX_DOUBLE_CLICK_INTERVAL};
use iced::{
    Button, Column, Command, Element, Image, Length, Row, Scrollable, Space, Text, TextInput,
    Tooltip,
};
use reciprocity_communication::messages::PlayerControl;
use reqwest::Url;
use std::time::{Duration, Instant};

const TOOLTIP_DURATION: Duration = Duration::from_secs(2);

#[derive(Debug, Clone)]
pub enum SearchMessage {
    SearchClick(usize),
    SearchImage(String, iced::image::Handle),
    SearchResult(String, Vec<Video>),
    InputChanged(String),
    InputSubmit(),
    UpdateUi(),
}

#[derive(Debug)]
pub struct SearchTab {
    scroll: iced::scrollable::State,
    search_input: iced::text_input::State,
    search_input_value: String,

    tooltip: (String, Instant),

    search: String,
    results: Vec<(Option<iced::image::Handle>, Video)>,

    last_click: (usize, Instant),
    btn_states: Vec<iced::button::State>,
}

impl SearchTab {
    pub fn new() -> Self {
        //TODO
        SearchTab {
            scroll: Default::default(),
            search_input: Default::default(),
            search_input_value: "".to_string(),
            tooltip: ("".to_string(), Instant::now()),
            search: "".to_string(),
            results: Vec::new(),
            last_click: (0, Instant::now()),
            btn_states: Vec::new(),
        }
    }

    pub fn update(&mut self, con: &Option<Connection>, message: SearchMessage) -> Command<Message> {
        match message {
            SearchMessage::InputChanged(i) => self.search_input_value = i,
            SearchMessage::InputSubmit() => {
                self.search = self.search_input_value.clone();
                return Command::perform(search(self.search.clone()), move |res| match res {
                    Ok((query, videos)) => {
                        Message::Search(SearchMessage::SearchResult(query, videos))
                    }
                    Err(e) => panic!("Search Error: {:?}", e),
                });
            }
            SearchMessage::SearchResult(q, v) => {
                if self.search.eq(&q) {
                    self.results = v.iter().cloned().map(|v| (None, v)).collect();

                    let mut commands = Vec::new();
                    for (_, video) in self.results.iter() {
                        let id = video.id.clone();
                        let img_url =
                            get_image_uri_from_yt_id(id.clone()).expect("Error creating Image Uri");
                        println!("{:?}", id);
                        commands.push(Command::perform(
                            async move {
                                let res = get_image(img_url).await;
                                (id, res)
                            },
                            |(id, res)| match res {
                                Ok(img) => Message::Search(SearchMessage::SearchImage(id, img)),
                                Err(e) => {
                                    println!("{}", e);
                                    Message::None()
                                }
                            },
                        ))
                    }
                    return Command::batch(commands);
                }
            }
            SearchMessage::SearchImage(id, img) => self
                .results
                .iter_mut()
                .filter(|(_, v)| v.id.eq(&id))
                .map(|(handle, _)| *handle = img.clone().into())
                .collect(),
            SearchMessage::SearchClick(i) => {
                if self.last_click.0 == i
                    && self.last_click.1.elapsed() <= MAX_DOUBLE_CLICK_INTERVAL
                {
                    self.last_click = (0, Instant::now());
                    if let Some(con) = con {
                        if let Some((_, song)) = self.results.get(i - 1) {
                            let url = Url::parse(&song.url).expect("Error Parsing Url");
                            self.tooltip = (song.title.clone(), Instant::now());
                            return Command::batch(vec![
                                con.control_request(PlayerControl::Enqueue(url)),
                                Command::perform(
                                    async {
                                        tokio::time::sleep(
                                            TOOLTIP_DURATION + Duration::from_millis(200),
                                        )
                                        .await
                                    },
                                    |_| Message::Search(SearchMessage::UpdateUi()),
                                ),
                            ]);
                        }
                    }
                }
                self.last_click = (i, Instant::now());
            }
            SearchMessage::UpdateUi() => {
                //Ignore
            }
        }

        Command::none()
    }
}

impl Tab for SearchTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Search")
    }

    fn tab_label(&self) -> (Option<Icon>, String) {
        (Icon::Search.into(), "Search".to_string())
    }

    fn content(&mut self, theme: &Theme) -> Element<'_, Self::Message> {
        //TODO styling
        let mut column = Column::new().height(Length::Fill);

        while self.btn_states.len() <= self.results.len() {
            self.btn_states.push(Default::default());
        }

        let search_input = TextInput::new(
            &mut self.search_input,
            "Youtube Search...",
            &self.search_input_value,
            |i| Message::Search(SearchMessage::InputChanged(i)),
        )
        .on_submit(Message::Search(SearchMessage::InputSubmit()))
        .style(theme.search_input_theme());
        column = column.push(search_input);

        let mut results_column = Scrollable::new(&mut self.scroll).height(Length::Fill).width(Length::Fill);
        for (i, ((img, video), btn_state)) in self
            .results
            .iter()
            .zip(self.btn_states.iter_mut())
            .enumerate()
        {
            let i = i + 1;
            let mut row = Row::new().width(Length::Fill).height(Length::Units(73));
            if let Some(img) = img.as_ref() {
                let img = Image::new(img.clone()).width(Length::Units(130));

                row = row
                    .push(img)
                    .push(Space::new(Length::Units(10), Length::Shrink));
            } else {
                row = row.push(Space::new(Length::Units(140), Length::Shrink))
            }
            row = row.push(
                Column::new()
                    .push(
                        Text::new(format!("{} - {}", video.title, video.duration))
                            .width(Length::Fill),
                    )
                    .push(
                        Text::new(format!("{}, {}", video.views, video.upload_date))
                            .width(Length::Fill),
                    ),
            );

            let btn = Button::new(btn_state, row)
                .on_press(Message::Search(SearchMessage::SearchClick(i)))
                .style(theme.tab_button_theme())
                .width(Length::Fill);
            let btn_row = Row::new()
                .push(btn)
                .push(Space::new(Length::Units(10), Length::Shrink));

            results_column = results_column.push(btn_row);
        }

        let mut view = column.push(results_column).into();
        if self.tooltip.1.elapsed() < TOOLTIP_DURATION && !self.tooltip.0.is_empty() {
            view = Tooltip::new(
                view,
                format!("Added: {}", self.tooltip.0),
                iced::tooltip::Position::Top,
            )
            .style(theme.tooltip_container_theme())
            .gap(10)
            .into()
        }
        view
    }
}
