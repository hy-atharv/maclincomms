use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub enum StatusTypes{
    LOG_IN_SUCCESSFUL,
    REGISTRATION_SUCCESSFUL,
    INVALID_CREDENTIALS,
    INVALID_TOKEN,
    AUTHORIZED,
    DATABASE_ERROR,
    USER_ALREADY_EXISTS

}