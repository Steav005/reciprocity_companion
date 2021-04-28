mod dark;
mod light;

use iced::{button, container, radio, slider, text_input, Color};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];

    pub fn radio_button_theme(&self) -> Box<dyn radio::StyleSheet> {
        match self {
            Theme::Light => light::RadioButton.into(),
            Theme::Dark => dark::RadioButton.into(),
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            Theme::Light => light::TEXT,
            Theme::Dark => dark::TEXT,
        }
    }

    pub fn tab_view_container_theme(&self) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Light => light::TabViewContainer.into(),
            Theme::Dark => dark::TabViewContainer.into(),
        }
    }

    pub fn search_input_theme(&self) -> Box<dyn text_input::StyleSheet> {
        match self {
            Theme::Light => light::SearchInput.into(),
            Theme::Dark => dark::SearchInput.into(),
        }
    }

    pub fn tab_button_theme(&self) -> Box<dyn button::StyleSheet> {
        match self {
            Theme::Light => light::TabButton.into(),
            Theme::Dark => dark::TabButton.into(),
        }
    }

    pub fn selected_tab_button_theme(&self) -> Box<dyn button::StyleSheet> {
        match self {
            Theme::Light => light::SelectedTabButton.into(),
            Theme::Dark => dark::SelectedTabButton.into(),
        }
    }

    pub fn control_button_theme(&self) -> Box<dyn button::StyleSheet> {
        match self {
            Theme::Light => light::ControlButton.into(),
            Theme::Dark => dark::ControlButton.into(),
        }
    }

    pub fn tab_container_theme(&self) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Light => light::TabsContainer.into(),
            Theme::Dark => dark::TabsContainer.into(),
        }
    }

    pub fn control_container_theme(&self) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Light => light::ControlsContainer.into(),
            Theme::Dark => dark::ControlsContainer.into(),
        }
    }

    pub fn footer_container_theme(&self) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Light => light::FooterContainer.into(),
            Theme::Dark => dark::FooterContainer.into(),
        }
    }

    pub fn song_slider_theme(&self) -> Box<dyn slider::StyleSheet> {
        match self {
            Theme::Light => light::SongPosSlider.into(),
            Theme::Dark => dark::SongPosSlider.into(),
        }
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Dark
    }
}

/*
impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::TextInput.into(),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Button.into(),
            Theme::Dark => dark::Button.into(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Scrollable.into(),
        }
    }
}

impl From<Theme> for Box<dyn slider::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Slider.into(),
        }
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::ProgressBar.into(),
        }
    }
}

impl From<Theme> for Box<dyn checkbox::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Checkbox.into(),
        }
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Rule.into(),
        }
    }
}
*/
