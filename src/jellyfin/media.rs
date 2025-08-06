use reqwest::{Client};
use crate::jellyfin::models::{ItemsResponse, MediaItem, MediaType,SearchResponse};
use crate::jellyfin::config::load_config;
use crate::jellyfin::http::authed_get_json;
use crate::utils::select::choose_item;

pub async fn list_items(base_url: &str, media_type: MediaType) -> Result<Vec<MediaItem>, String> {
    let config = load_config().map_err(|_| "Failed to load config")?;
    let user_id = config
        .user_id
        .ok_or("Missing user ID in saved config")?;

    let client = Client::new();

    let url = format!(
        "{}/Users/{}/Items?IncludeItemTypes={}&Recursive=true&SortBy=SortName",
        base_url.trim_end_matches('/'),
        user_id,
        media_type.as_str()
    );

    let res: ItemsResponse = authed_get_json(&client, &url).await?;
    Ok(res.items)
}

pub async fn search_items(base_url: &str, query: &str) -> Result<Vec<MediaItem>, reqwest::Error> {
    let url = format!(
        "{}/Search/Hints?SearchTerm={}",
        base_url.trim_end_matches('/'),
        query
    );

    let client = Client::new();
    let response: Result<SearchResponse, String> = authed_get_json(&client, &url).await;
    Ok(response.unwrap().search_hints)
}
