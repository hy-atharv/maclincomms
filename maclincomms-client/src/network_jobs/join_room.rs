use reqwest::{Client, Response, Error};
use serde::Deserialize;
use serde_json::json;

use crate::user_model::StatusTypes;

#[derive(Debug)]
pub enum JoinRoomResponseResult {
    ROOM(JoinRoomResponseData),
    ROOM_NOT_FOUND,
    INVALID_CREDENTIALS,
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Clone, Deserialize)]
pub struct JoinRoomResponseData {
    pub status_type: StatusTypes,
    pub room_token: String,
    pub room_name: String,
    pub message: String
}


pub async fn join_room(
    token: String,
    room_name: String,
    room_key: String,
    join_room_endpoint: &'static str
) -> JoinRoomResponseResult {

    let url = join_room_endpoint.to_string();
    let client = Client::new();

    let room = json!({
        "room_name": room_name,
        "room_key": room_key
    });

    let response = client
        .post(url)
        .json(&room)
        .header("Authorization", token)
        .send()
        .await;

    match response {
        Ok(data) => {
            let res_data = data.json::<JoinRoomResponseData>().await.unwrap();
    
            match res_data.status_type {
                StatusTypes::ROOM_AUTHORIZATION_SUCCESSFUL => {
                    return JoinRoomResponseResult::ROOM(res_data);
                }
                StatusTypes::ROOM_NOT_FOUND => {
                    return JoinRoomResponseResult::ROOM_NOT_FOUND;
                }
                StatusTypes::INVALID_CREDENTIALS => {
                    return JoinRoomResponseResult::INVALID_CREDENTIALS;
                }
                StatusTypes::DATABASE_ERROR => {
                    return JoinRoomResponseResult::DATABASE_ERROR;
                }
                _ => {
                    return JoinRoomResponseResult::UNKNOWN_ERROR;
                }
            }
    
        },
        Err(err) => {
            return JoinRoomResponseResult::REQUEST_ERROR;
        }
    }
}