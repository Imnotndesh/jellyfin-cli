mod cli;
mod jellyfin;
mod utils;

use clap::Parser;
use cli::args::{Cli, Commands};
use cli::login::handle_login;
use cli::list::handle_list;
use cli::search::handle_search;
use cli::pick::handle_pick;
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login {
            server,
            username,
            password,
        } => {
            handle_login(&server, &username, &password).await;
        }

        Commands::List {
            media_type,
            server,
        } => {
            handle_list(&media_type, server).await;
        }

        Commands::Search { server, query } => {
            handle_search(server, &query).await;
        }
        Commands::Watch {
            server,query
        } => {
            handle_pick(server, &query).await;
        }
    }
}
