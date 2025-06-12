use reqwest::{Client, Response, Error};
use serde::Deserialize;
use serde_json::json;

use crate::user_model::StatusTypes;

#[derive(Debug)]
pub enum CreateRoomResponseResult {
    ROOM(CreateRoomResponseData),
    ROOM_ALREADY_EXISTS,
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoomResponseData {
    pub status_type: StatusTypes,
    pub room_token: String,
    pub room_name: String,
    pub room_key: String,
    pub message: String
}





pub async fn create_room(
    token: String,
    room_name: String,
    public_key: String,
    create_room_endpoint: &'static str
) -> CreateRoomResponseResult {

    let url = create_room_endpoint.to_string();
    let client = Client::new();

    let room = json!({
        "room_name": room_name,
        "owner_key": public_key
    });

    let response = client
        .post(url)
        .json(&room)
        .header("Authorization", token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<CreateRoomResponseData>().await.unwrap();

            match res_data.status_type {
                StatusTypes::ROOM_CREATION_SUCCESSFUL => {
                    return CreateRoomResponseResult::ROOM(res_data);
                }
                StatusTypes::ROOM_ALREADY_EXISTS => {
                    return CreateRoomResponseResult::ROOM_ALREADY_EXISTS;
                }
                StatusTypes::DATABASE_ERROR => {
                    return CreateRoomResponseResult::DATABASE_ERROR;
                }
                _ => {
                    return CreateRoomResponseResult::UNKNOWN_ERROR;
                }
            }

        },
        Err(err) => {
            return CreateRoomResponseResult::REQUEST_ERROR;
        }
    }
}