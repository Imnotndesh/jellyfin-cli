use inquire::Select;
use crate::jellyfin::models::MediaItem;
// Will Use inquire::Select to show a list of selectable options
pub fn choose_item(items: &[MediaItem]) -> Option<&MediaItem> {
    if items.is_empty() {
        println!("No items available.");
        return None;
    }
    let options: Vec<String> = items
        .iter()
        .map(|item| format!("{} [{}]", item.name, item.media_type))
        .collect();

    // Show selection prompt
    let selection = Select::new("Choose an item:", options)
        .prompt();

    match selection {
        Ok(choice) => {
            let index = items.iter().position(|item| {
                format!("{} [{}]", item.name, item.media_type) == choice
            })?;
            Some(&items[index])
        }
        Err(_) => None,
    }
}
