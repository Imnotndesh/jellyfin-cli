use reqwest::dns::Resolve;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JellyfinConfig {
    pub access_token: Option<String>,
}

impl ::std::default::Default for JellyfinConfig {
    fn default() -> Self {
        Self{
            access_token: None,
        }
    }
}

pub fn save_config(config: &JellyfinConfig) -> Result<(), confy::ConfyError> {
    confy::store("jellyfin-cli", "config",config)
}
pub fn load_config() -> Result<JellyfinConfig, confy::ConfyError> {
    confy::load("jellyfin-cli", "config")
}