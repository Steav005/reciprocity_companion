use iced::{button, container, radio, slider, text_input, Background, Color, Vector};

pub const TEXT: Color = Color::from_rgb(
    0x50 as f32 / 255.0,
    0x50 as f32 / 255.0,
    0x50 as f32 / 255.0,
);

const HOVERED_TEXT: Color = Color::from_rgb(
    0x10 as f32 / 255.0,
    0x10 as f32 / 255.0,
    0x10 as f32 / 255.0,
);

const SURFACE: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);

const SURFACE_1: Color = Color::from_rgb(
    0xF2 as f32 / 255.0,
    0xF2 as f32 / 255.0,
    0xF2 as f32 / 255.0,
);

const SURFACE_2: Color = Color::from_rgb(
    0xE5 as f32 / 255.0,
    0xE5 as f32 / 255.0,
    0xE5 as f32 / 255.0,
);

const SURFACE_3: Color = Color::from_rgb(
    0xB0 as f32 / 255.0,
    0xB0 as f32 / 255.0,
    0xB0 as f32 / 255.0,
);

const ACCENT: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x89 as f32 / 255.0,
    0xDA as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.2);

pub struct SearchInput;

impl text_input::StyleSheet for SearchInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(SURFACE_1),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: SURFACE_3,
        }
    }

    fn focused(&self) -> text_input::Style {
        self.active()
    }

    fn placeholder_color(&self) -> Color {
        Color { a: 0.6, ..TEXT }
    }

    fn value_color(&self) -> Color {
        TEXT
    }

    fn selection_color(&self) -> Color {
        Color { a: 0.3, ..TEXT }
    }
}

pub struct SongPosSlider;

impl slider::StyleSheet for SongPosSlider {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (SURFACE_3, Color::TRANSPARENT),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 5.0 },
                color: TEXT,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        slider::Style {
            rail_colors: (ACCENT, Color::TRANSPARENT),
            ..self.active()
        }
    }

    fn dragging(&self) -> slider::Style {
        self.hovered()
    }
}

pub struct TabViewContainer;

impl container::StyleSheet for TabViewContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: SURFACE.into(),
            ..Default::default()
        }
    }
}

pub struct ControlsContainer;

impl container::StyleSheet for ControlsContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: SURFACE_2.into(),
            ..Default::default()
        }
    }
}

pub struct FooterContainer;

impl container::StyleSheet for FooterContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: SURFACE_3.into(),
            ..Default::default()
        }
    }
}

pub struct TooltipContainer;

impl container::StyleSheet for TooltipContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: HOVERED.into(),
            border_radius: 10.0,
            ..Default::default()
        }
    }
}

pub struct TabsContainer;

impl container::StyleSheet for TabsContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: SURFACE_1.into(),
            ..Default::default()
        }
    }
}

pub struct ControlButton;

impl button::StyleSheet for ControlButton {
    fn active(&self) -> button::Style {
        button::Style {
            ..TabButton.active()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: HOVERED.into(),
            ..self.active()
        }
    }
}

pub struct SelectedTabButton;

impl button::StyleSheet for SelectedTabButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: ACCENT,
            ..TabButton.active()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            //text_color: Color::WHITE,
            background: HOVERED.into(),
            shadow_offset: Vector::new(0.0, 0.0), //Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}

pub struct TabButton;

impl button::StyleSheet for TabButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: None, //Color::from_rgb(0.11, 0.42, 0.87).into(),
            border_radius: 12.0,
            border_width: 0.0,
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: TEXT,
            //..button::Style::default()
            border_color: Color::TRANSPARENT,
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            //text_color: Color::WHITE,
            background: HOVERED.into(),
            shadow_offset: Vector::new(0.0, 0.0), //Vector::new(1.0, 2.0),
            text_color: HOVERED_TEXT,
            ..self.active()
        }
    }
}

pub struct RadioButton;

impl radio::StyleSheet for RadioButton {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: Background::Color(SURFACE_1),
            dot_color: TEXT,
            border_width: 1.0,
            border_color: TEXT,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: Background::Color(SURFACE_2),
            ..self.active()
        }
    }
}
