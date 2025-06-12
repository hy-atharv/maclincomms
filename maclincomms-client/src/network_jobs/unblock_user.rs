use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::user_model::StatusTypes;

#[derive(Debug)]
pub enum UnblockUserResponseResult {
    USER_UNBLOCKED,
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnblockUserResponseData {
    pub status_type: StatusTypes,
    pub message: String
}




pub async fn unblock_user(
    token: String,
    username: String,
    unblock_user_endpoint: &'static str
) -> UnblockUserResponseResult {

    let url = unblock_user_endpoint.to_string();
    let client = Client::new();

    let data = json!({
        "username_to_unblock": username,
    });

    let response = client
        .post(url)
        .json(&data)
        .header("Authorization", token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<UnblockUserResponseData>().await.unwrap();

            match res_data.status_type {
                StatusTypes::USER_UNBLOCKED => {
                    return UnblockUserResponseResult::USER_UNBLOCKED;
                }
                StatusTypes::DATABASE_ERROR => {
                    return UnblockUserResponseResult::DATABASE_ERROR;
                }
                _ => {
                    return UnblockUserResponseResult::UNKNOWN_ERROR;
                }
            }

        },
        Err(err) => {
            return UnblockUserResponseResult::REQUEST_ERROR;
        }
    }
}