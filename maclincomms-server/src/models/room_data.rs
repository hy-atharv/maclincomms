use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Create_Room{
    pub room_name: String,
    pub owner_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Join_Room{
    pub room_name: String,
    pub room_key: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rooms_Table{
    pub room_name: String,
    pub room_key: String,
    pub room_owner: String,
    pub room_members: Vec<String>,
    pub members_keys: Vec<Value>
}




#[derive(Clone, Serialize, Deserialize)]
pub struct RoomSenderMessage {
    pub username: String,
    pub content: String,
    pub users: Vec<String>,
    pub whisper_mode: WhisperMode,
    pub is_join_leave_msg: bool
}
#[derive(Clone, Serialize, Deserialize)]
pub enum WhisperMode {
    HIDE_FROM,
    SHARE_WITH,
    NONE
}


#[derive(Clone, Serialize, Deserialize)]
pub struct RoomReceiverMessage {
    pub username: String,
    pub content: String,
    pub is_join_leave_msg: bool
}
