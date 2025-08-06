use reqwest::{Client, RequestBuilder};
use crate::jellyfin::config::load_config;

pub async fn authed_get_json<T: serde::de::DeserializeOwned>(
    client: &Client,
    url: &str,
) -> Result<T, String> {
    let builder = authed_get_request(client, url)?;
    let res = builder
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    res.json::<T>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}
pub fn authed_get_request(client: &Client, url: &str) -> Result<RequestBuilder, String> {
    let config = load_config().map_err(|_| "Failed to load config")?;
    let token = config
        .access_token
        .ok_or("Missing access token in config")?;

    Ok(client
        .get(url)
        .header("X-Emby-Token", token))
}
