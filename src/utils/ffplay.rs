use std::process::{Command, Child};
use std::thread;
use std::io::{self, Write};

pub fn play_in_background(url: String, item_name: String) {
    println!("Now playing: {}\nType `q` then Enter to quit playback.", item_name);

    // Spawn a thread to run ffplay
    let mut child: Child = Command::new("ffplay")
        .arg("-autoexit")
        .arg("-loglevel")
        .arg("quiet")
        .arg(&url)
        .spawn()
        .expect("Failed to launch ffplay");

    // Spawn another thread to wait for user input (q to quit)
    let _ = thread::spawn(move || {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            input.clear();
            if let Ok(_) = io::stdin().read_line(&mut input) {
                if input.trim() == "q" {
                    println!("Quitting playback...");
                    let _ = child.kill();
                    break;
                } else {
                    println!("Unknown command. Type `q` to quit.");
                }
            }
        }
    });
}