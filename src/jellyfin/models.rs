use std::fmt;
use serde::{Serialize, Deserialize };
// request payload
#[derive(Serialize)]
pub struct LoginRequest {
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Pw")]
    pub password: String,
}

#[derive(Deserialize,Debug)]
pub struct JellyfinUser {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
}
// response from new login
#[derive(Deserialize,Debug)]
pub struct LoginResponse {
    #[serde(rename = "User")]
    pub user: JellyfinUser,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}
#[derive(Debug,Clone, Deserialize)]
pub struct MediaItem {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Type")]
    pub media_type: String,

    #[serde(rename = "ProductionYear")]
    pub year: Option<i32>,

    #[serde(rename = "Overview")]
    pub overview: Option<String>,

    #[serde(rename = "CommunityRating")]
    pub rating: Option<f64>,

    // #[serde(rename = "RunTimeTicks")]
    // pub runtime_ticks: Option<i64>,
}
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "SearchHints")]
    pub search_hints: Vec<MediaItem>,
}

/// Response wrapper from /Items endpoint
#[derive(Debug, Deserialize)]
pub struct ItemsResponse {
    #[serde(rename = "Items")]
    pub items: Vec<MediaItem>,
}

#[derive(Debug, Clone, Copy)]
pub enum MediaType {
    Movie,
    Series,
    Audio,
}

impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Movie => "Movie",
            MediaType::Series => "Series",
            MediaType::Audio => "Audio",
        }
    }
}
impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the `as_str()` helper for clean string values
        write!(f, "{}", self.as_str())
    }
}
