use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct User_Auth_Table{
    pub username: String,
    pub password_hash: String,
    pub password_salt: String
}