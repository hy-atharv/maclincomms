use std::collections::HashMap;

use reqwest::{Client, Response, Error};
use serde::Deserialize;
use serde_json::json;

use crate::user_model::{ChatData, DmUser_Data, StatusTypes};

#[derive(Debug)]
pub enum UploadDmChatsResponseResult {
    DMS_DATA_UPLOADED,
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR,
    INVALID_DATA
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadDmChatsResponseData {
    pub status_type: StatusTypes,
    pub message: String
}



pub async fn upload_dm_chats(
    token: String,
    upload_dm_chats_endpoint: &'static str,
    chats_data: ChatData
) -> UploadDmChatsResponseResult {

    let url = upload_dm_chats_endpoint.to_string();
    let client = Client::new();


    if let Ok(json_data) = serde_json::to_value(chats_data){
        let response = client
            .post(url)
            .json(&json_data)
            .header("Authorization", token)
            .send()
            .await;


        match response {
            Ok(data) => {
                
                let res_data = data.json::<UploadDmChatsResponseData>().await.unwrap();

                match res_data.status_type {
                    StatusTypes::DMS_DATA_FETCHED => {
                        return UploadDmChatsResponseResult::DMS_DATA_UPLOADED;
                    }
                    StatusTypes::DATABASE_ERROR => {
                        return UploadDmChatsResponseResult::DATABASE_ERROR;
                    }
                    _ => {
                        return UploadDmChatsResponseResult::UNKNOWN_ERROR;
                    }
                }

            },
            Err(err) => {
                return UploadDmChatsResponseResult::REQUEST_ERROR;
            }
        }
    }
    else{
        return UploadDmChatsResponseResult::INVALID_DATA;
    }
}