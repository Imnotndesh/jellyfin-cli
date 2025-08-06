use crate::jellyfin::media::search_items;
use crate::jellyfin::models::MediaItem;
use crate::jellyfin::config::resolve_server;

pub async fn handle_search(base_url: Option<String>, query: &str) {
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
            } else {
                println!("Found {} result(s) for '{}':", items.len(), query);
                for item in items {
                    print_media_item(&item);
                }
            }
        }
        Err(err) => {
            eprintln!("Search failed: {}", err);
        }
    }
}

fn print_media_item(item: &MediaItem) {
    println!("🎬 {} [{}]", item.name, item.media_type);

    if let Some(year) = item.year {
        println!("Year: {}", year);
    }

    if let Some(overview) = &item.overview {
        println!("Overview: {}", overview);
    }

    if let Some(rating) = item.rating {
        println!("Rating: {:.1}", rating);
    }

    println!("---");
}