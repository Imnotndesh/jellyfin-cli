use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JellyfinConfig {
    pub access_token: Option<String>,
    pub server: Option<String>,
    pub user_id: Option<String>,
}

impl ::std::default::Default for JellyfinConfig {
    fn default() -> Self {
        Self{
            access_token: None,
            server : None,
            user_id : None,
        }
    }
}

pub fn save_config(config: &JellyfinConfig) -> Result<(), confy::ConfyError> {
    confy::store("jellyfin-cli", "config",config)
}
pub fn load_config() -> Result<JellyfinConfig, confy::ConfyError> {
    confy::load("jellyfin-cli", "config")
}
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