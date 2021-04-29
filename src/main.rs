#![windows_subsystem = "windows"]

use directories::ProjectDirs;
use iced::{Application, Settings};
use reciprocity_companion::config::Config;
use reciprocity_companion::Companion;
use std::fs::File;
use std::io::Write;

fn main() -> iced::Result {
    let mut config = Config::default();
    let config_path =
        if let Some(proj_dir) = ProjectDirs::from("de", "Autumnal", "Reciprocity Companion") {
            let config_dir = proj_dir.config_dir();
            std::fs::create_dir_all(config_dir).expect("Could not get or create config path");
            let config_path = config_dir.with_file_name("config.yml");
            if config_path.exists() {
                //Load Config
                config = serde_yaml::from_reader(
                    File::open(config_path.clone()).expect("Could not open File"),
                )
                .expect("Error Parsing Config");
            } else {
                //Create Default Config
                File::create(config_path.clone())
                    .expect("Could not create config")
                    .write_all(
                        serde_yaml::to_vec(&config)
                            .expect("Could not parse default config")
                            .as_slice(),
                    )
                    .expect("Could not write default config");
            }
            config_path
        } else {
            panic!("Could not get Project Dir")
        };

    Companion::run(Settings {
        window: iced::window::Settings {
            size: (1280, 750),
            min_size: Some((600, 360)),
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        flags: (config, config_path),
        default_font: Some(include_bytes!("./fonts/NotoSansSC-Medium.otf")),
        default_text_size: 22,
        exit_on_close_request: true,
        antialiasing: true,
    })
}
