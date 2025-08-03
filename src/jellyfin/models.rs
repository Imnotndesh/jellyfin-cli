use serde::{ Serialize, Deserialize };
// request payload
#[derive(Serialize)]
pub struct LoginRequest {
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Pw")]
    pub password: String,
}

#[derive(Deserialize,Debug)]
pub struct JellyfinUser {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
}
// response from new login
#[derive(Deserialize,Debug)]
pub struct LoginResponse {
    #[serde(rename = "User")]
    pub user: JellyfinUser,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}