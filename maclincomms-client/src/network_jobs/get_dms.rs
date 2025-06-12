use reqwest::{Client, Response, Error};
use serde::Deserialize;
use serde_json::json;

use crate::user_model::{DmUser_Data, StatusTypes};

#[derive(Debug)]
pub enum GetDmsResponseResult {
    DMS_DATA_FETCHED(Vec<DmUser_Data>),
    DATABASE_ERROR,
    REQUEST_ERROR,
    UNKNOWN_ERROR
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDmsResponseData {
    pub status_type: StatusTypes,
    pub data: Vec<DmUser_Data>,
    pub message: String
}




pub async fn get_dms(
    token: String,
    get_dms_endpoint: &'static str
) -> GetDmsResponseResult {

    let url = get_dms_endpoint.to_string();
    let client = Client::new();


    let response = client
        .get(url)
        .header("Authorization", token)
        .send()
        .await;


    match response {
        Ok(data) => {
            
            let res_data = data.json::<GetDmsResponseData>().await.unwrap();

            match res_data.status_type {
                StatusTypes::DMS_DATA_FETCHED => {
                    return GetDmsResponseResult::DMS_DATA_FETCHED(res_data.data);
                }
                StatusTypes::DATABASE_ERROR => {
                    return GetDmsResponseResult::DATABASE_ERROR;
                }
                _ => {
                    return GetDmsResponseResult::UNKNOWN_ERROR;
                }
            }

        },
        Err(err) => {
            return GetDmsResponseResult::REQUEST_ERROR;
        }
    }
}