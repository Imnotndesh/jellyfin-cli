use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "jellyfin-cli")]
#[command(version = "0.1")]
#[command(about = "Interact with your Jellyfin server from the terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Login {
        #[arg(short, long)]
        server: String,

        #[arg(short, long)]
        username: String,

        #[arg(short, long)]
        password: String,
    },

    List {
        #[arg(short = 'm', long)]
        media_type: String,

        #[arg(short, long)]
        server: Option<String>,
    },
    Search {
        /// Query string to search for
        #[arg(short, long)]
        query: String,

        /// Jellyfin server base URL
        #[arg(short, long)]
        server: Option<String>,
    },
    Watch {
        /// Query string to search for
        #[arg(short, long)]
        query: String,

        /// Jellyfin server base URL
        #[arg(short, long)]
        server: Option<String>,
    },
}
