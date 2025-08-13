use crate::jellyfin::auth::login;
use crate::jellyfin::config::{save_config, JellyfinConfig};
use crate::jellyfin::models::PlayerType;

pub async fn handle_login(server: &str, username: &str, password: &str) {
    match login(server, username, password).await {
        Ok(response) => {
            println!("Login successful as '{}'", response.user.name);

            let token = response.access_token.clone();
            let user_id = response.user.id.clone();

            let config = JellyfinConfig {
                server: Some(server.to_string()),
                access_token: Some(token),
                user_id: Some(user_id),
                default_player: Some(PlayerType::FFMPEG.to_string())
            };

            match save_config(&config) {
                Ok(_) => println!("Access token saved."),
                Err(e) => eprintln!("Failed to save config: {}", e),
            }
        }
        Err(err) => {
            eprintln!("Login failed: {}", err);
        }
    }
}
