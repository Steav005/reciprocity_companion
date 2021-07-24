use log::{warn, error, debug, info};

#[derive(Clone, Debug)]
pub enum LogMessage{
    Error(String),
    Warn(String),
    Debug(String),
    Info(String),
}

impl LogMessage{
    pub fn log(&self){
        match self {
            LogMessage::Error(m) => error!("{}", m),
            LogMessage::Warn(m) => warn!("{}", m),
            LogMessage::Debug(m) => debug!("{}", m),
            LogMessage::Info(m) => info!("{}", m),
        }
    }
}