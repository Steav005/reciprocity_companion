#![windows_subsystem = "windows"]

use log::info;
use directories::ProjectDirs;
use iced::{Application, Settings};
use reciprocity_companion::config::Config;
use reciprocity_companion::Companion;
use std::fs::File;
use std::io::Write;
use image::ImageFormat;
use iced::window::Icon;
use std::path::PathBuf;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Root};
use log::LevelFilter;
use chrono::{DateTime, Local};

fn main() -> iced::Result {
    let icon = image::load_from_memory_with_format(include_bytes!("./icons/google/twotone_settings_remote_white_48dp.png"), ImageFormat::Png).unwrap();
    let icon = icon.to_rgba8();
    let icon = Icon::from_rgba(icon.pixels().map(|rgba| rgba.0.iter()).flatten().cloned().collect(), 96, 96).ok();

    let mut config = Config::default();
    let config_path =
        if let Some(proj_dir) = ProjectDirs::from("de", "Autumnal", "Reciprocity Companion") {
            let config_dir = proj_dir.config_dir();
            let log_dir = proj_dir.data_dir();
            std::fs::create_dir_all(config_dir).expect("Could not get or create config path");
            std::fs::create_dir_all(log_dir).expect("Could not get or create log path");
            let config_path = config_dir.join("config.yml");
            let log_path = log_dir.join(Local::now().format("%Y-%m-%d %H_%M_%S.log").to_string());

            let logfile = FileAppender::builder()
                .build(log_path)
                .expect("Could not create Log File");

            let log_config = log4rs::config::Config::builder()
                .appender(Appender::builder().build("logfile", Box::new(logfile)))
                .build(Root::builder()
                    .appender("logfile")
                    .build(LevelFilter::Info))
                .expect("Could not create Log Config");

            log4rs::init_config(log_config).expect("Could not init Log");

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
            icon,
        },
        flags: (config, config_path),
        default_font: Some(include_bytes!("./fonts/NotoSansSC-Medium.otf")),
        default_text_size: 22,
        exit_on_close_request: true,
        antialiasing: true,
    })
}
