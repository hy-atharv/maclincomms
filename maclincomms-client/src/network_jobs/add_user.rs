use reqwest::{Client, Response, Error};
use serde::Deserialize;
use serde_json::json;

use crate::user_model::StatusTypes;

#[derive(Debug)]
pub enum AddUserResponseResult {
    ADD_REQUEST_SENT,
    USER_NOT_FOUND,
    NOTIFICATIONS_ERROR,
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddUserResponseData {
    pub status_type: StatusTypes,
    pub message: String
}




pub async fn add_user(
    token: String,
    username: String,
    message: String,
    add_user_endpoint: &'static str
) -> AddUserResponseResult {

    let url = add_user_endpoint.to_string();
    let client = Client::new();

    let data = json!({
        "username_to_add": username,
        "message": message
    });

    let response = client
        .post(url)
        .json(&data)
        .header("Authorization", token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<AddUserResponseData>().await.unwrap();

            match res_data.status_type {
                StatusTypes::ADD_REQUEST_SENT => {
                    return AddUserResponseResult::ADD_REQUEST_SENT;
                }
                StatusTypes::USER_NOT_FOUND => {
                    return AddUserResponseResult::USER_NOT_FOUND;
                }
                StatusTypes::DATABASE_ERROR => {
                    return AddUserResponseResult::DATABASE_ERROR;
                }
                StatusTypes::NOTIFICATIONS_ERROR => {
                    return AddUserResponseResult::NOTIFICATIONS_ERROR;
                }
                _ => {
                    return AddUserResponseResult::UNKNOWN_ERROR;
                }
            }

        },
        Err(err) => {
            return AddUserResponseResult::REQUEST_ERROR;
        }
    }
}