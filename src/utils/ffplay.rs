use std::process::{Command, Child};
use inquire::Select;

pub fn show_playback_menu(mut child: Child) {
    let options = vec!["Quit playback"];
    let choice = Select::new("Quick Action", options).prompt();

    match choice {
        Ok("Quit playback") => {
            let _ = child.kill();
            println!("Playback stopped.");
        }
        _ => {
            println!("❌ Unknown option or cancelled.");
            let _ = child.kill();
        }
    }
}
pub fn play_in_background(stream_url: &str) -> Option<Child> {
    match Command::new("ffplay")
        .arg("-autoexit")
        .arg("-loglevel")
        .arg("quiet")
        .arg(stream_url)
        .spawn()
    {
        Ok(child) => Some(child),
        Err(e) => {
            eprintln!("❌ Failed to launch ffplay: {}", e);
            None
        }
    }
}
// Print ffplay controls
pub fn print_ffplay_controls(
    title: Option<&str>,
    controls: Option<&[(&str, &str)]>,
) {
    let title = title.unwrap_or("ffplay Controls");

    let default_controls = [
        ("q", "Quit playback"),
        ("p / space", "Pause / Play toggle"),
        ("← / →", "Seek backward / forward"),
        ("↑ / ↓", "Volume up / down"),
        ("m", "Mute / Unmute"),
    ];

    let controls = controls.unwrap_or(&default_controls);

    println!("\n+{:-^54}+", "");
    println!("|{:^54}|", title);
    println!("+{:-^19}+{:-^34}+", "", "");

    println!("| {:<17} | {:<32} |", "Key", "Action");
    println!("+{:-^19}+{:-^34}+", "", "");

    for (key, action) in controls {
        println!("| {:<17} | {:<32} |", key, action);
    }

    println!("+{:-^19}+{:-^34}+\n", "", "");
}


