use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::dms_data::ChatData;

use super::{dms_data::DmUser_Data, room_data::Rooms_Table, status_types::StatusTypes};


#[derive(Serialize, Deserialize)]
pub struct RegisterResponseData{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
    pub message: String
}


#[derive(Serialize, Deserialize)]
pub struct LoginResponseData{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct RequestNewTokenResponseData{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
    pub message: String
}


//-----ROOMS---------------------

#[derive(Serialize, Deserialize)]
pub struct CreateRoomResponseData{
    pub status_type: StatusTypes,
    pub room_token: String,
    pub room_name: String,
    pub room_key: String,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomResponseData{
    pub status_type: StatusTypes,
    pub room_token: String,
    pub room_name: String,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct RoomDataResponseData{
    pub status_type: StatusTypes,
    pub room_data: Rooms_Table,
    pub message: String
}


//-----------------DMS------------------

#[derive(Serialize, Deserialize)]
pub struct AddUserResponseData{
    pub status_type: StatusTypes,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct BlockUserResponseData{
    pub status_type: StatusTypes,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct AcceptUserResponseData{
    pub status_type: StatusTypes,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct UnblockUserResponseData{
    pub status_type: StatusTypes,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct GetDmsResponseData{
    pub status_type: StatusTypes,
    pub data: Vec<DmUser_Data>,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct GetDmChatsResponseData{
    pub status_type: StatusTypes,
    pub data: ChatData,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct UploadDmChatsResponseData{
    pub status_type: StatusTypes,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct QueuedNotificationsReponseData{
    pub status_type: StatusTypes,
    pub data: HashMap<String, Vec<String>>,
    pub message: String
}
