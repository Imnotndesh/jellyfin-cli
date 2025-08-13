use inquire::Select;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

/// Path to MPV's IPC socket
const MPV_SOCKET: &str = "/tmp/mpv_socket";

/// Check if mpv is installed
pub fn is_mpv_installed() -> bool {
    let mpv_cmd = if cfg!(target_os = "windows") { "mpv.exe" } else { "mpv" };
    which::which(mpv_cmd).is_ok() || Path::new(mpv_cmd).exists()
}

/// Start mpv with IPC socket enabled
pub fn play_with_controls(stream_url: &str) -> Option<Child> {
    if !is_mpv_installed() {
        eprintln!("mpv not found in PATH.");
        return None;
    }
    let mpv_cmd = if cfg!(target_os = "windows") { "mpv.exe" } else { "mpv" };
    match Command::new(mpv_cmd)
        .arg("--quiet")
        .arg("--no-terminal")
        .arg(format!("--input-ipc-server={}", MPV_SOCKET))
        .arg("--idle=yes")
        .arg(stream_url)
        .spawn()
    {
        Ok(child) => Some(child),
        Err(e) => {
            eprintln!("❌ Failed to start mpv: {}", e);
            None
        }
    }
}

/// Send a JSON IPC command to mpv
fn send_mpv_command(json_cmd: &str) -> std::io::Result<()> {
    let mut stream = UnixStream::connect(MPV_SOCKET)?;
    stream.write_all(json_cmd.as_bytes())?;
    Ok(())
}

/// Interactive playback menu
pub fn show_playback_menu(_child: Child) {
    loop {
        let options = vec![
            "▶ Play",
            "⏸ Pause",
            "⏹ Stop & Exit",
        ];

        match Select::new("Playback Control", options.clone()).prompt() {
            Ok(choice) => match choice {
                "▶ Play" => {
                    let _ = send_mpv_command("{\"command\": [\"set_property\", \"pause\", false]}\n");
                }
                "⏸ Pause" => {
                    let _ = send_mpv_command("{\"command\": [\"set_property\", \"pause\", true]}\n");
                }
                "⏹ Stop & Exit" => {
                    let _ = send_mpv_command("{\"command\": [\"quit\"]}\n");
                    break;
                }
                _ => {}
            },
            Err(_) => {
                let _ = send_mpv_command("{\"command\": [\"quit\"]}\n");
                break;
            }
        }
        sleep(Duration::from_millis(100));
    }
}
