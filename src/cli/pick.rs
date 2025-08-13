use crate::jellyfin::config::{get_token, resolve_server, get_default_player};
use crate::jellyfin::media::search_items;
use crate::jellyfin::models::PlayerType;
use crate::utils::select::choose_item;
use crate::utils::{ffplay, mpv};

pub async fn handle_pick(base_url: Option<String>, query: &str) {
    let server = match resolve_server(base_url) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
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
                println!("Playing: {} [{}]", item.name, item.media_type);
                let stream_url = format!(
                    "{}/Videos/{}/main.m3u8?api_key={}",
                    server.trim_end_matches('/'),
                    item.id,
                    get_token().unwrap()
                );

                // Pick the player type from config
                match get_default_player() {
                    Ok(PlayerType::FFMPEG) => {
                        ffplay::print_ffplay_controls(None, None);
                        if let Some(child) = ffplay::play_in_background(&stream_url) {
                            ffplay::show_playback_menu(child);
                        }
                    }
                    Ok(PlayerType::MPV) => {
                        if let Some(child) = mpv::play_with_controls(&stream_url) {
                            mpv::show_playback_menu(child);
                        }
                    }
                    Ok(PlayerType::VLC) => {
                        eprintln!("VLC support is not yet implemented.");
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
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
