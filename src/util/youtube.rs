use reciprocity_communication::messages::oauth2::url::ParseError;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum SearchError {
    UrlParse(Arc<ParseError>),
    Reqwest(Arc<reqwest::Error>),
    Serde(Arc<serde_json::Error>),
}

impl From<ParseError> for SearchError {
    fn from(e: ParseError) -> Self {
        SearchError::UrlParse(Arc::new(e))
    }
}

impl From<reqwest::Error> for SearchError {
    fn from(e: reqwest::Error) -> Self {
        SearchError::Reqwest(Arc::new(e))
    }
}

impl From<serde_json::Error> for SearchError {
    fn from(e: serde_json::Error) -> Self {
        SearchError::Serde(Arc::new(e))
    }
}

pub async fn search(q: String) -> Result<(String, Vec<Video>), SearchError> {
    let base = "http://youtube-scrape.herokuapp.com/api/search?page=1".to_string();
    let url = reqwest::Url::parse_with_params(&base, &[("q", q.clone())])?;
    let res = reqwest::get(url).await.unwrap().text().await?;
    let mut res: SearchResult = serde_json::from_str(&res)?;
    let results: Vec<_> = res.results.drain(..).map(|r| r.video).flatten().collect();

    Ok((q, results))
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    results: Vec<VideoResult>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VideoResult {
    video: Option<Video>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub url: String,
    pub duration: String,
    pub snippet: String,
    pub upload_date: String,
    pub thumbnail_src: String,
    pub views: String,
}
