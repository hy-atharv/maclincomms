use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct UserClaims{
    pub username: String,
    pub exp: i64
 // pub exp: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthenticationToken {
    pub username: String,
}


