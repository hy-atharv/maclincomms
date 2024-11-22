use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct User{
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterResponse{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusTypes{
    LOG_IN_SUCCESSFUL,
    REGISTRATION_SUCCESSFUL,
    INVALID_CREDENTIALS,
    INVALID_TOKEN,
    AUTHORIZED,
    DATABASE_ERROR,
    USER_ALREADY_EXISTS

}

