use crate::jellyfin::config::{get_token, resolve_server};
use crate::jellyfin::media::search_items;
use crate::utils::select::choose_item;
use crate::utils::ffplay::play_in_background;

pub async fn handle_pick(base_url: Option<String>, query: &str) {
    let server = match resolve_server(base_url) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {}", e);
            return;
        }
    };
    match search_items(&server, query).await {
        Ok(items) => {
            if items.is_empty() {
                println!("No results found for '{}'", query);
                return;
            }
            if let Some(item) = choose_item(&items) {
                println!("You picked: {}", item.name);
                println!("Media Type: {}", item.media_type);
                let stream_url = format!(
                    "{}/Videos/{}/main.m3u8?api_key={}",
                    server.trim_end_matches('/'),
                    item.id,
                    get_token().unwrap()
                );

                println!("Launching ffplay...");
                play_in_background(stream_url, item.name.clone());
            } else {
                println!("No selection made.");
            }
        }
        Err(err) => {
            eprintln!("Search failed: {}", err);
        }
    }
}