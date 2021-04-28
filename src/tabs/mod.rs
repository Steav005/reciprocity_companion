use crate::icons::Icon;
use crate::theme::Theme;
use iced::{Align, Button, Column, Command, Container, Element, Length, Row, Space, Text};
use std::borrow::BorrowMut;
use std::cmp::min;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};

pub mod history;
pub mod playlist;
pub mod search;
pub mod settings;

pub const HEADER_SIZE: u16 = 32;
pub const TAB_PADDING: u16 = 16;

pub struct Tabs<M, const SIZE: usize>
where
    M: Clone + Debug,
{
    cur_tab: usize,
    label: [iced::button::State; SIZE],
    on_select: Box<dyn Fn(usize) -> M>,
}

impl<M, const SIZE: usize> Debug for Tabs<M, SIZE>
where
    M: Clone + Debug,
{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<M, const SIZE: usize> Tabs<M, SIZE>
where
    M: Clone + Debug,
{
    pub fn new(cur_tab: usize, on_select: impl Fn(usize) -> M + 'static) -> Tabs<M, SIZE> {
        let label: [iced::button::State; SIZE] = (0..SIZE)
            .map(|_| iced::button::State::new())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Could build Button State Array");

        Tabs {
            cur_tab,
            label,
            on_select: Box::new(on_select),
        }
    }

    pub fn update(&mut self, cur: usize) -> Command<M> {
        self.cur_tab = min(SIZE, cur);
        Command::none()
    }

    pub fn view<'a>(
        &'a mut self,
        tabs: [&'a mut dyn Tab<Message = M>; SIZE],
        theme: &Theme,
    ) -> (Element<'a, M>, Element<'a, M>) {
        let mut tabs_column = Column::new();
        for (i, (tab, label_state)) in tabs.iter().zip(self.label.iter_mut()).enumerate() {
            let (icon, label_text) = tab.tab_label();
            let mut label = Row::new();
            if let Some(icon) = icon {
                label = label
                    .push(icon.get_svg(theme).height(Length::Units(22)))
                    .push(Space::new(Length::Units(5), Length::Shrink));
            }
            label = label.push(Text::new(label_text));
            let mut tab_button = Button::new(label_state, label)
                .width(Length::Fill)
                .on_press((self.on_select)(i))
                .style(theme.tab_button_theme());
            if self.cur_tab == i {
                tab_button = tab_button.style(theme.selected_tab_button_theme())
            }

            let container = Container::new(tab_button).width(Length::Fill);
            tabs_column = tabs_column.push(container);
        }

        let cur_tab_view = tabs[self.cur_tab].view(theme);
        //let column = Column::new()
        //    .push(tabs_row)
        //    .push(cur_tab_view);
        (
            Container::new(tabs_column)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            Container::new(cur_tab_view)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        )
    }
}

pub trait Tab: Debug {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> (Option<Icon>, String);

    fn borrowed(&mut self) -> &mut dyn Tab<Message = Self::Message>
    where
        Self: Sized,
    {
        self.borrow_mut()
    }

    fn view(&mut self, theme: &Theme) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(10)
            .push(
                Text::new(self.title())
                    .size(HEADER_SIZE)
                    .color(theme.text_color()),
            )
            .push(self.content(theme));

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .padding(TAB_PADDING)
            .style(theme.tab_view_container_theme())
            .into()
    }

    fn content(&mut self, theme: &Theme) -> Element<'_, Self::Message>;
}
