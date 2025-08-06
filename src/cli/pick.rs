use crate::jellyfin::config::{get_token, resolve_server};
use crate::jellyfin::media::search_items;
use crate::utils::select::choose_item;
use crate::utils::ffplay::{play_in_background, print_ffplay_controls, show_playback_menu};

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
                println!("Playing: {} [{}]", item.name,item.media_type);
                let stream_url = format!(
                    "{}/Videos/{}/main.m3u8?api_key={}",
                    server.trim_end_matches('/'),
                    item.id,
                    get_token().unwrap()
                );
                print_ffplay_controls(None,None);
                if let Some(child) = play_in_background(&stream_url) {
                    show_playback_menu(child);
                }
            } else {
                println!("No selection made.");
            }
        }
        Err(err) => {
            eprintln!("Search failed: {}", err);
        }
    }
}