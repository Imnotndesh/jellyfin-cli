use inquire::Select;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use std::path::Path;

/// Check if mpv is installed
pub fn is_mpv_installed() -> bool {
    let mpv_cmd = if cfg!(target_os = "windows") { "mpv.exe" } else { "mpv" };
    which::which(mpv_cmd).is_ok() || Path::new(mpv_cmd).exists()
}

/// Start mpv in stdin control mode
pub fn play_with_controls(stream_url: &str) -> Option<Child> {
    if !is_mpv_installed() {
        eprintln!("❌ mpv not found in PATH.");
        return None;
    }

    let mpv_cmd = if cfg!(target_os = "windows") { "mpv.exe" } else { "mpv" };

    match Command::new(mpv_cmd)
        .arg("--quiet")
        .arg("--no-terminal")
        .arg("--input-terminal=yes")
        .arg("--idle=yes") // keep running after pause
        .arg(stream_url)
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(child) => Some(child),
        Err(e) => {
            eprintln!("❌ Failed to start mpv: {}", e);
            None
        }
    }
}

/// Interactive playback menu
pub fn show_playback_menu(mut child: Child) {
    let stdin = child.stdin.as_mut().expect("Failed to open mpv stdin");

    loop {
        let options = vec![
            "▶ Play",
            "⏸ Pause",
            "⏹ Stop & Exit",
        ];

        match Select::new("Playback Control", options.clone()).prompt() {
            Ok(choice) => match choice {
                "▶ Play" => {
                    let _ = stdin.write_all(b"set pause no\n");
                }
                "⏸ Pause" => {
                    let _ = stdin.write_all(b"set pause yes\n");
                }
                "⏹ Stop & Exit" => {
                    let _ = stdin.write_all(b"quit\n");
                    break;
                }
                _ => {}
            },
            Err(_) => {
                let _ = stdin.write_all(b"quit\n");
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
