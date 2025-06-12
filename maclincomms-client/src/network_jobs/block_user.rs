use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::user_model::StatusTypes;

#[derive(Debug)]
pub enum BlockUserResponseResult {
    USER_BLOCKED,
    USER_ALREADY_BLOCKED,
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockUserResponseData {
    pub status_type: StatusTypes,
    pub message: String
}




pub async fn block_user(
    token: String,
    username: String,
    block_user_endpoint: &'static str
) -> BlockUserResponseResult {

    let url = block_user_endpoint.to_string();
    let client = Client::new();

    let data = json!({
        "username_to_block": username,
    });

    let response = client
        .post(url)
        .json(&data)
        .header("Authorization", token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<BlockUserResponseData>().await.unwrap();

            match res_data.status_type {
                StatusTypes::USER_BLOCKED => {
                    return BlockUserResponseResult::USER_BLOCKED;
                }
                StatusTypes::USER_ALREADY_BLOCKED => {
                    return BlockUserResponseResult::USER_ALREADY_BLOCKED;
                }
                StatusTypes::DATABASE_ERROR => {
                    return BlockUserResponseResult::DATABASE_ERROR;
                }
                _ => {
                    return BlockUserResponseResult::UNKNOWN_ERROR;
                }
            }

        },
        Err(err) => {
            return BlockUserResponseResult::REQUEST_ERROR;
        }
    }
}