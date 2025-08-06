use crate::jellyfin::config::resolve_server;
use crate::jellyfin::media::search_items;
use crate::utils::select::choose_item;

/// Interactive search and pick function
pub async fn handle_pick(base_url: Option<String>, query: &str) {
    let server = match resolve_server(base_url) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {}", e);
            return;
        }
    };

    // Perform the search
    match search_items(&server, query).await {
        Ok(items) => {
            if items.is_empty() {
                println!("⚠️ No results found for '{}'", query);
                return;
            }

            // Let user choose one
            if let Some(item) = choose_item(&items) {
                println!("You picked: {}", item.name);
                println!("ID: {}", item.id);
            } else {
                println!("No selection made.");
            }
        }
        Err(err) => {
            eprintln!("Search failed: {}", err);
        }
    }
}
