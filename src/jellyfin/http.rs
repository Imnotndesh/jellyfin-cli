use reqwest::{Client, RequestBuilder};
use crate::jellyfin::config::load_config;
use once_cell::sync::Lazy;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("jellyfin-cli")
        .build()
        .expect("Failed to build HTTP client")
});

pub fn get_client() -> &'static Client {
    &HTTP_CLIENT
}

pub async fn authed_get_json<T: serde::de::DeserializeOwned>(
    url: &str,
) -> Result<T, String> {
    let builder = authed_get_request(get_client(), url)?;
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
