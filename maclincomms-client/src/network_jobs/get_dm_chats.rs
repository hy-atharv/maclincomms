use std::collections::HashMap;

use reqwest::{Client, Response, Error};
use serde::Deserialize;
use serde_json::json;

use crate::user_model::{ChatData, StatusTypes};

#[derive(Debug)]
pub enum GetDmChatsResponseResult {
    DMS_DATA_FETCHED(ChatData),
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Deserialize)]
pub struct GetDmChatsResponseData {
    pub status_type: StatusTypes,
    pub data: ChatData,
    pub message: String
}




pub async fn get_dm_chats(
    token: String,
    get_dm_chats_endpoint: &'static str
) -> GetDmChatsResponseResult {

    let url = get_dm_chats_endpoint.to_string();
    let client = Client::new();


    let response = client
        .get(url)
        .header("Authorization", token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<GetDmChatsResponseData>().await.unwrap();

            match res_data.status_type {
                StatusTypes::DMS_DATA_FETCHED => {
                    return GetDmChatsResponseResult::DMS_DATA_FETCHED(res_data.data);
                }
                StatusTypes::DATABASE_ERROR => {
                    return GetDmChatsResponseResult::DATABASE_ERROR;
                }
                _ => {
                    return GetDmChatsResponseResult::UNKNOWN_ERROR;
                }
            }

        },
        Err(err) => {
            return GetDmChatsResponseResult::REQUEST_ERROR;
        }
    }
}