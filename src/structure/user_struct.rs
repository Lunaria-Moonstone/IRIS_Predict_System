use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserLoginInfo {
    pub username: String,
    pub password: String,
}