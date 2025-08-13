use crate::jellyfin::config::set_default_player;

pub fn handle_change_player(new_player: &Option<String>){
    match set_default_player(new_player) {
        Ok(_) => {
            println!("New player set to default");
        }
        Err(e) => {
            println!("Error setting default player: {}", e);
        }
    }
}