use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct Register_User{
    pub username: String,
    pub password: String,
    pub public_identity_key: String
}

