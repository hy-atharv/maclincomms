use std::collections::HashMap;

use reqwest::{Client};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::user_model::{Rooms_Table, StatusTypes};

#[derive(Serialize, Deserialize)]
pub struct GetRoomDataResponseData{
    pub status_type: StatusTypes,
    pub room_data: Rooms_Table,
    pub message: String
}

pub async fn get_room_data(
    room_token: String,
    get_room_data_endpoint: &'static str
) -> Option<Rooms_Table> {

    let url = get_room_data_endpoint.to_string();
    let client = Client::new();


    let response = client
        .get(url)
        .header("Authorization", room_token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<GetRoomDataResponseData>().await.unwrap();
               
            match res_data.status_type {
                StatusTypes::ROOM_FOUND => {
                    return Some(res_data.room_data);
                }
                StatusTypes::DATABASE_ERROR => {
                    //database error
                    return None;
                }
                _ => {
                    //other error
                    return None;
                }
            }

        },
        Err(err) => {
            //Error in request
            return None;
        }
    }
}