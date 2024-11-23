use serde::{Deserialize, Serialize};

use super::status_types::StatusTypes;


#[derive(Serialize, Deserialize)]
pub struct LoginResponseData{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponseData{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizationResponseData{
    pub status_type: StatusTypes,
    pub message: String
}