use crate::theme::Theme;
use reciprocity_communication::client::Config as ComConfig;
use reciprocity_communication::messages::oauth2::RefreshToken;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_com")]
    pub com: ComConfig,
    #[serde(default)]
    pub refresh_token: Option<RefreshToken>,
    #[serde(default = "default_bot_link")]
    pub bot_link: String,
    #[serde(default)]
    pub theme: Theme,
}

fn default_com() -> ComConfig {
    ComConfig {
        client_id: "815279319513432134".to_string(),
        auth_url: "https://discord.com/api/oauth2/authorize".to_string(),
        redirect_url: "localhost:1887".to_string(),
    }
}

fn default_bot_link() -> String {
    "ws://autumnal.de:1337".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            com: default_com(),
            refresh_token: None,
            bot_link: default_bot_link(),
            theme: Default::default(),
        }
    }
}

impl Config {
    pub fn update(&self, path: PathBuf) {
        //TODO Error Handling
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(serde_yaml::to_vec(self).unwrap().as_slice())
            .unwrap();
    }
}
