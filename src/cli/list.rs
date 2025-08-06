use crate::jellyfin::media::list_items;
use crate::jellyfin::models::MediaType;
use crate::jellyfin::config::resolve_server;


pub async fn handle_list(media_type: &str, server_url: Option<String>) {
    let media_type_enum = match media_type.to_lowercase().as_str() {
        "movie" => MediaType::Movie,
        "series" => MediaType::Series,
        "audio" => MediaType::Audio,
        _ => {
            eprintln!("Invalid media type '{}'. Use Movie, Series, or Audio.", media_type);
            return;
        }
    };
    let server = match resolve_server(server_url) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}",e);
            return;
        }
    };

    match list_items(&server, media_type_enum).await {
        Ok(items) => {
            println!("Found {} {}(s):", items.len(), media_type_enum);
            for item in items {
                println!("{}", item.name);
                println!("Year: {}", item.year.map_or("N/A".to_string(), |y| y.to_string()));
                println!("ID: {}", item.id);
                println!("Rating: {}", item.rating.map_or("N/A".to_string(), |r| format!("{:.1}", r)));
                if let Some(desc) = &item.overview {
                println!("Overview: {}", desc);
            }
            println!("---");
        }
        }
        Err(err) => {
            eprintln!("Failed to fetch items: {}", err);
        }
    }
}
