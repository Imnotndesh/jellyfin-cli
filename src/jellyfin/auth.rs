use reqwest::Client;
use crate::jellyfin::models::{LoginRequest,LoginResponse };


// Login to jellyfin server
pub async fn login(base_url: &str,username: &str,password: &str) -> Result<LoginResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/Users/AuthenticateByName", base_url.trim_end_matches('/'));
    let body = LoginRequest{
        username: username.to_string(),
        password: password.to_string(),
    };
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("X-Emby-Authorization", "MediaBrowser Client=\"jellyfin-cli\", Device=\"rust-cli\", DeviceId=\"1234\", Version=\"0.1\"".to_string())
        .json(&body)
        .send()
        .await?
        .json::<LoginResponse>()
        .await?;
    Ok(res)
}