use crate::jellyfin::models::{ItemsResponse, MediaItem, MediaType,SearchResponse,SeasonsResponse,EpisodesResponse,Episode,Season};
use crate::jellyfin::config::{get_token, load_config};
use crate::jellyfin::http::authed_get_json;
// list all items in jellyfin
pub async fn list_items(base_url: &str, media_type: MediaType) -> Result<Vec<MediaItem>, String> {
    let config = load_config().map_err(|_| "Failed to load config")?;
    let user_id = config
        .user_id
        .ok_or("Missing user ID in saved config")?;

    let url = format!(
        "{}/Users/{}/Items?IncludeItemTypes={}&Recursive=true&SortBy=SortName",
        base_url.trim_end_matches('/'),
        user_id,
        media_type.as_str()
    );

    let res: ItemsResponse = authed_get_json(&url).await?;
    Ok(res.items)
}
// Search for any item in jellyfin
pub async fn search_items(base_url: &str, query: &str) -> Result<Vec<MediaItem>, reqwest::Error> {
    let url = format!(
        "{}/Search/Hints?SearchTerm={}",
        base_url.trim_end_matches('/'),
        query
    );
    let response: Result<SearchResponse, String> = authed_get_json(&url).await;
    Ok(response.unwrap().search_hints)
}
pub async fn get_seasons(base_url: &str, series_id: &str) -> Result<Vec<Season>, String> {
    let url = format!(
        "{}/Shows/{}/Seasons?api_key={}",
        base_url.trim_end_matches('/'),
        series_id,
        get_token().map_err(|_| "Missing token")?
    );
    let res: Result<SeasonsResponse, String> = authed_get_json(&url).await;
    res.map(|r| r.items)
}

// Fetch episodes for a given season ID
pub async fn get_episodes(base_url: &str, season_id: &str) -> Result<Vec<Episode>, String> {
    let url = format!(
        "{}/Shows/{}/Episodes?api_key={}",
        base_url.trim_end_matches('/'),
        season_id,
        get_token().map_err(|_| "Missing token")?
    );
    let res: Result<EpisodesResponse, String> = authed_get_json(&url).await;
    res.map(|r| r.items)
}