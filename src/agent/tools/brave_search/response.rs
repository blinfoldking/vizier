use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BraveResponse {
    pub query: Query,
    pub web: Option<WebResults>,
    pub news: Option<NewsResults>,
    pub videos: Option<VideoResults>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub original: String,
    pub altered: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebResults {
    pub results: Vec<WebResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebResult {
    pub title: String,
    pub url: String,
    pub description: String,
    pub page_age: Option<String>,
    pub profile: Option<Profile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub url: String,
    pub img: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsResults {
    pub results: Vec<NewsResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsResult {
    pub title: String,
    pub url: String,
    pub description: String,
    pub source: String,
    pub age: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoResults {
    pub results: Vec<VideoResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoResult {
    pub title: String,
    pub url: String,
    pub description: String,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumbnail {
    pub src: String,
}
