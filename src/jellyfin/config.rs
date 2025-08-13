use std::fmt::Display;
use std::str::FromStr;
pub(crate) use crate::jellyfin::models::{JellyfinConfig, PlayerType};



impl Default for JellyfinConfig {
    fn default() -> Self {
        Self {
            access_token: None,
            server: None,
            user_id: None,
            default_player: Some(PlayerType::MPV.to_string()),
        }
    }
}
impl Display for PlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PlayerType::MPV => "mpv",
            PlayerType::VLC => "vlc",
            PlayerType::FFMPEG => "ffmpeg",
        };
        write!(f, "{}", name)
    }
}

// Save to config
pub fn save_config(config: &JellyfinConfig) -> Result<(), confy::ConfyError> {
    confy::store("jellyfin-cli", "config", config)
}

// Get whole config
pub fn load_config() -> Result<JellyfinConfig, confy::ConfyError> {
    confy::load("jellyfin-cli", "config")
}

// Get token from config
pub fn get_token() -> Result<String, String> {
    let config = load_config().map_err(|_| "❌ Failed to load config")?;
    config
        .access_token
        .ok_or_else(|| "❌ Access token not found in saved config.".to_string())
}

// If no server passed pick default saved
pub fn resolve_server(cli_input: Option<String>) -> Result<String, String> {
    if let Some(server) = cli_input {
        Ok(server)
    } else {
        let config = load_config().map_err(|_| "❌ Failed to load config")?;
        config
            .server
            .ok_or_else(|| "❌ No server URL provided and none saved in config.".to_string())
    }
}

// Set default player
pub fn set_default_player(player: &Option<String>) -> Result<(), String> {
    let new_player = match player {
        Some(p) => PlayerType::from_str(p.as_str())?,
        None => return Err("No player specified.".to_string()),
    };

    let mut config = load_config().map_err(|_| "Failed to load config")?;
    config.default_player = Some(new_player.to_string());
    save_config(&config).map_err(|_| "Failed to save config".to_string())
}


// Get default player
pub fn get_default_player() -> Result<PlayerType, String> {
    let config = load_config().map_err(|_| "Failed to load config")?;

    if let Some(player_str) = config.default_player {
        PlayerType::from_str(&player_str)
            .map_err(|e| format!("Invalid default player in config: {}", e))
    } else {
        Err("No default player set in config".to_string())
    }
}
