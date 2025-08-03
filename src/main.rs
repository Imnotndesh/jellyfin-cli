use crate::jellyfin::auth::login;

mod jellyfin;
#[tokio::main]
async fn main() {
    let base_url = "http://192.168.1.70:8096";
    let username = "brian";
    let password = "l1sa0s0s0";
    match login(base_url, username, password).await {
        Ok(response) => {
            println!("Login was successful with access token {}", response.access_token);
            println!("Welcome, {} (ID: {})", response.user.name, response.user.id);
        }
        Err(e) => {
            println!("Login failed: {}", e);
        }
    }
}
