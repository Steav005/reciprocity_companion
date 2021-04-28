use crate::theme::Theme;
use iced::svg::Handle;
use iced::Svg;
use reciprocity_communication::messages::PlayMode;

pub enum Icon {
    Eject,
    Pause,
    Play,
    PlaylistAdd,
    QueueMusic,
    History,
    Repeat,
    RepeatOne,
    RepeatActive,
    RepeatOneActive,
    SkipNext,
    SkipPrevious,
    Search,
    Error,
    Warning,
    Settings,
}

impl Icon {
    pub fn get_svg_bytes(&self, theme: &Theme) -> &[u8] {
        match theme {
            Theme::Light => match self {
                Icon::Eject => include_bytes!("google/black/eject.svg"),
                Icon::Pause => include_bytes!("google/black/pause.svg"),
                Icon::Play => include_bytes!("google/black/play.svg"),
                Icon::PlaylistAdd => include_bytes!("google/black/playlist-add.svg"),
                Icon::QueueMusic => include_bytes!("google/black/queue-music.svg"),
                Icon::Repeat => include_bytes!("google/black/repeat.svg"),
                Icon::RepeatOne => include_bytes!("google/black/repeat-one.svg"),
                Icon::SkipNext => include_bytes!("google/black/skip-next.svg"),
                Icon::SkipPrevious => include_bytes!("google/black/skip-previous.svg"),
                Icon::RepeatActive => include_bytes!("google/black/repeat-on.svg"),
                Icon::RepeatOneActive => include_bytes!("google/black/repeat-one-on.svg"),
                Icon::Error => include_bytes!("google/black/error.svg"),
                Icon::Warning => include_bytes!("google/black/warning.svg"),
                Icon::History => include_bytes!("google/black/history.svg"),
                Icon::Search => include_bytes!("google/black/search.svg"),
                Icon::Settings => include_bytes!("google/black/settings.svg"),
            },
            Theme::Dark => match self {
                Icon::Eject => include_bytes!("google/white/eject.svg"),
                Icon::Pause => include_bytes!("google/white/pause.svg"),
                Icon::Play => include_bytes!("google/white/play.svg"),
                Icon::PlaylistAdd => include_bytes!("google/white/playlist-add.svg"),
                Icon::QueueMusic => include_bytes!("google/white/queue-music.svg"),
                Icon::Repeat => include_bytes!("google/white/repeat.svg"),
                Icon::RepeatOne => include_bytes!("google/white/repeat-one.svg"),
                Icon::SkipNext => include_bytes!("google/white/skip-next.svg"),
                Icon::SkipPrevious => include_bytes!("google/white/skip-previous.svg"),
                Icon::RepeatActive => include_bytes!("google/white/repeat-on.svg"),
                Icon::RepeatOneActive => include_bytes!("google/white/repeat-one-on.svg"),
                Icon::Error => include_bytes!("google/white/error.svg"),
                Icon::Warning => include_bytes!("google/white/warning.svg"),
                Icon::History => include_bytes!("google/white/history.svg"),
                Icon::Search => include_bytes!("google/white/search.svg"),
                Icon::Settings => include_bytes!("google/white/settings.svg"),
            },
        }
    }

    pub fn get_svg(&self, theme: &Theme) -> Svg {
        Svg::new(Handle::from_memory(self.get_svg_bytes(theme)))
    }
}

/// For Option of paused bool
impl From<Option<bool>> for Icon {
    fn from(state: Option<bool>) -> Self {
        if let Some(true) = state {
            return Icon::Pause;
        }
        Icon::Play
    }
}

impl From<Option<PlayMode>> for Icon {
    fn from(mode: Option<PlayMode>) -> Self {
        if let Some(mode) = mode {
            match mode {
                PlayMode::Normal => Icon::Repeat,
                PlayMode::LoopAll => Icon::RepeatActive,
                PlayMode::LoopOne => Icon::RepeatOneActive,
            }
        } else {
            Icon::Repeat
        }
    }
}
