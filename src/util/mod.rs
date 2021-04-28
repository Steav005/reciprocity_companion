use cached::proc_macro::cached;
use reciprocity_communication::messages::oauth2::url::ParseError;
use reciprocity_communication::messages::Track;
use reqwest::Url;
use std::borrow::Cow;
use std::fmt::Write;
use std::time::{Duration, Instant};

pub mod youtube;

pub fn duration_fmt(dur: &'_ Duration) -> String {
    let seconds = dur.as_secs() % 60;
    let minutes = (dur.as_secs() / 60) % 60;
    let hours = (dur.as_secs() / 60) / 60;
    let mut msg = String::from("");
    if hours > 0 {
        write!(msg, "{:02}:", hours).unwrap();
    }
    write!(msg, "{:02}:{:02}", minutes, seconds).unwrap();
    msg
}

#[derive(Debug, Clone)]
pub struct CompTrack {
    pub track: Track,
    pub updated: Instant,
    pub image: Option<iced::image::Handle>,
}

impl CompTrack {
    pub fn new(track: Track) -> Self {
        CompTrack {
            track,
            updated: Instant::now(),
            image: None,
        }
    }

    pub fn pos(&self, pos: f32) -> Duration {
        let factor = pos / 100.0;
        let millis = self.track.len.as_millis() as f32;
        Duration::from_millis((factor * millis) as u64)
    }

    pub fn pos_percentage(&self) -> f32 {
        let elapsed = self.updated.elapsed().as_millis() as f32;
        let track_pos = self.track.pos.as_millis() as f32;
        let track_len = self.track.len.as_millis() as f32;

        let percentage = ((elapsed + track_pos) / track_len) * 100.0;
        if percentage > 100.0 {
            return 100.0;
        }
        percentage
    }

    pub async fn download_image(mut self) -> Result<Self, String> {
        let url = self.track.uri.clone();
        let id = get_yt_identifier(url).map_err(|e| format!("{:?}", e))?;
        let img_url = get_image_uri_from_yt_id(id).map_err(|e| format!("{:?}", e))?;
        let img = get_image(img_url).await?;
        self.image = Some(img);

        Ok(self)
    }
}

#[cached(size = 100)]
pub async fn get_image(url: Url) -> Result<iced::image::Handle, String> {
    let bytes = reqwest::get(url)
        .await
        .map_err(|e| format!("{:?}", e))?
        .bytes()
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(iced::image::Handle::from_memory(bytes.to_vec()))
}

pub fn get_yt_identifier(url: String) -> Result<String, Option<ParseError>> {
    let url = reqwest::Url::parse(&url)?;
    let v = url.query_pairs().find(|(k, _)| Cow::Borrowed("v").eq(k));
    if let Some((_, v)) = v {
        return Ok(v.to_string());
    }
    Err(None)
}

pub fn get_image_uri_from_yt_id(id: String) -> Result<Url, ParseError> {
    //reqwest::Url::parse(&format!("https://img.youtube.com/vi/{}/0.jpg", id)) //480x360
    reqwest::Url::parse(&format!("https://img.youtube.com/vi/{}/mqdefault.jpg", id))
    //320x180
}
