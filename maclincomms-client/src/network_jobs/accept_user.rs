use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::user_model::{AcceptanceStatus, StatusTypes};

#[derive(Debug)]
pub enum AcceptUserResponseResult {
    USER_ADDED,
    USER_IGNORED,
    NOTIFICATIONS_ERROR,
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Clone, Deserialize)]
pub struct AcceptUserResponseData {
    pub status_type: StatusTypes,
    pub message: String
}




pub async fn accept_user(
    token: String,
    username: String,
    status: AcceptanceStatus,
    accept_user_endpoint: &'static str
) -> AcceptUserResponseResult {

    let url = accept_user_endpoint.to_string();
    let client = Client::new();

    let data = json!({
        "username_to_accept": username,
        "acceptance_status": status
    });

    let response = client
        .post(url)
        .json(&data)
        .header("Authorization", token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<AcceptUserResponseData>().await.unwrap();

            match res_data.status_type {
                StatusTypes::USER_ADDED => {
                    return AcceptUserResponseResult::USER_ADDED;
                }
                StatusTypes::USER_IGNORED => {
                    return AcceptUserResponseResult::USER_IGNORED;
                }
                StatusTypes::NOTIFICATIONS_ERROR => {
                    return AcceptUserResponseResult::NOTIFICATIONS_ERROR;
                }
                StatusTypes::DATABASE_ERROR => {
                    return AcceptUserResponseResult::DATABASE_ERROR;
                }
                _ => {
                    return AcceptUserResponseResult::UNKNOWN_ERROR;
                }
            }

        },
        Err(err) => {
            return AcceptUserResponseResult::REQUEST_ERROR;
        }
    }
}