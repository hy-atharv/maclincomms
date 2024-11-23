use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Login_User{
    pub username: String,
    pub password: String
}
