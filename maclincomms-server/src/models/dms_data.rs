use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct Add_User{
    pub username_to_add: String,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accept_User{
    pub username_to_accept: String,
    pub acceptance_status: AcceptanceStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcceptanceStatus{
    ACCEPTED,
    IGNORED
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DmSenderMessage {
    pub username: String,
    pub content: String,
    pub is_online_offline_msg: bool
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Block_User{
    pub username_to_block: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unblock_User{
    pub username_to_unblock: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blocked_List{
    pub blocked_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dms_List{
    pub username: String,
    pub dms_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DmUser_Data{
    pub username: String,
    pub public_identity_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dms_Table{
    pub username: String,
    pub dms_list: Vec<String>,
    pub blocked_list: Vec<String>,
    pub chat_history: Vec<HashMap<String, String>>
}

//--------------ChatsBackupUploadModel----------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatData(pub Vec<ChatEntry>); //Tuple Structs

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatEntry(pub HashMap<String, Vec<Message>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Message(
    pub String,  // sender
    pub String,  // message content
    pub String,  // time string like "12 PM"
    pub String,  // timestamp like "2025-05-14 16:26:00+00"
    pub bool,     // is_online_offline_msg
    pub String    // message ack (> or >>)
);
//-------------------------------------------------------------