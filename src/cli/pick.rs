use crate::jellyfin::config::{get_token, resolve_server, get_default_player};
use crate::jellyfin::media::{get_episodes, get_seasons, search_items};
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
    // Plays a given Jellyfin item
    fn play_item(base_url: &str, id: &str) {
        let stream_url = format!(
            "{}/Videos/{}/main.m3u8?api_key={}",
            base_url.trim_end_matches('/'),
            id,
            get_token().unwrap()
        );

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
            }
        }
    }

    match search_items(&server, query).await {
        Ok(items) => {
            if items.is_empty() {
                println!("No results found for '{}'", query);
                return;
            }

            if let Some(item) = choose_item(&items, |i| format!("{} [{}]", i.name, i.media_type)) {
                match item.media_type.as_str() {
                    "Movie" => {
                        println!("Playing movie: {}", item.name);
                        play_item(&server, &item.id);
                    }
                    "Series" => {
                        println!("Series selected: {}", item.name);

                        if let Ok(seasons) = get_seasons(&server, &item.id).await {
                            if let Some(season) = choose_item(&seasons, |s| format!("{} [{}]", s.name, s.id)) {
                                if let Ok(episodes) = get_episodes(&server, &season.id).await {
                                    if let Some(episode) = choose_item(&episodes, |e| format!("{} [{}]", e.name, e.id)) {
                                        play_item(&server, &episode.id);
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Unsupported media type: {}", item.media_type);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Search failed: {}", err);
        }
    }

}
