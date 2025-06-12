use std::{collections::HashMap};

use disk_persist::DiskPersist;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::crypto::signature_keypair::generate_signature_keypair;

//USER DATA IN APP FOR PERSISTENCE 
#[derive(Deserialize, Serialize, Debug)]
pub struct UserData{ //Normal Data and Auth Token
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_expiry: i64
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UserIdentityKeys{ //Identity Key Pair
    pub public_identity_key: String,
    pub private_identity_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSignatureKeys{ //Signature Key Pair
    pub public_signature_key: [u8;32],
    pub private_signature_key: [u8;32],
}

impl UserSignatureKeys{
    pub fn new() -> Self{
        let (sig_pub, sig_priv) = generate_signature_keypair();
        Self { 
            public_signature_key: sig_pub, 
            private_signature_key: sig_priv 
        }
    }
}


//----------------------------------------------
#[derive(Deserialize, Serialize)]
pub struct UserLoginPayload{
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserRegisterPayload{
    pub username: String,
    pub password: String,
    pub public_identity_key: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterResponse{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResponse{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestNewTokenResponse{
    pub status_type: StatusTypes,
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcceptanceStatus{
    ACCEPTED,
    IGNORED
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rooms_Table{
    pub room_name: String,
    pub room_key: String,
    pub room_owner: String,
    pub room_members: Vec<String>,
    pub members_keys: Vec<Value>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatusTypes{
    LOG_IN_SUCCESSFUL,
    REGISTRATION_SUCCESSFUL,
    TOKEN_GRANTED,
    INVALID_CREDENTIALS,
    INVALID_TOKEN,
    USER_NOT_FOUND,
    ADD_REQUEST_SENT,
    AUTHORIZED,
    DATABASE_ERROR,
    NOTIFICATIONS_FETCHED_SUCCESSFULLY,
    DMS_DATA_FETCHED,
    DMS_DATA_UPLOADED,
    NOTIFICATIONS_ERROR,
    USER_ALREADY_EXISTS,
    USER_ADDED,
    USER_IGNORED,
    USER_ALREADY_BLOCKED,
    USER_BLOCKED,
    USER_UNBLOCKED,
    ROOM_NOT_FOUND,
    ROOM_FOUND,
    ROOM_ALREADY_EXISTS,
    ROOM_CREATION_SUCCESSFUL,
    ROOM_AUTHORIZATION_SUCCESSFUL
}

//-----------Message Types------------------------


#[derive(Clone, Serialize, Deserialize)]
pub enum MessageType {
    WORLD_CHAT(WorldChatMessage),
    ROOM(RoomMessageType),
    DM(DmMessage)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorldChatMessage {
    pub username: String,
    pub content: String,
    pub is_join_leave_msg: bool
}


#[derive(Clone, Serialize, Deserialize)]
pub enum RoomMessageType {
    SENDER(RoomSenderMessage),
    RECEIVER(RoomReceiverMessage)
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


#[derive(Clone, Serialize, Deserialize)]
pub struct DmMessage {
    pub username: String,
    pub content: String,
    pub is_online_offline_msg: bool
}


//-----------SOCKET MESSAGE TYPES-------------

#[derive(Clone)]
pub enum SocketMessage {
    Message(MessageType),
    Join(MessageType),
    Leave(MessageType),
    RoomSenderKey(Vec<u8>),
    Acknowledgement(Vec<u8>),
    Disconnect(DisconnectType),
    // File(FileInfo)
}

#[derive(Clone)]
pub enum DisconnectType {
    WORLD_CHAT,
    ROOM,
    DM
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub filename: String,
    pub filesize: String,
}


//--------------MESSAGE ACKS (Acknowledgements)

#[derive(Clone, Serialize, Deserialize)]
pub enum AckType {
    ServerAck,
    ReceiverAck
}

impl AckType {
    pub fn byte(&self) -> Vec<u8> {
        match self {
            Self::ServerAck => vec![0x01],
            Self::ReceiverAck => vec![0x02],
        }
    }
}


//-----------NOTIFICATIONS---------------------

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct NotificationData {
    pub n_type: NotificationType,
    pub from: String,
    pub to: String,
    pub content: String,
    pub time: String
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum NotificationType {
    MESSAGE,
    ADD_REQUEST,
    ACCEPTED
}


//-------------------DMS LIST--------------------------

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DmsListData {
    pub with_user: String,
    pub latest_msg: String,
    pub time: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DmUser_Data{
    pub username: String,
    pub public_identity_key: String,
}

//DATA LEVEL REPO FOR ALL DMS DATA SUPPLYING UI LEVEL DATA STRUCTS
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DmChats_Warehouse{
    pub dms_session_key: HashMap<String, DmSessionEncryption_Key>, // Username -> DmSession_Key
    pub dms_data: HashMap<String, Vec<(String, String, String, String, bool, String)>>, // Username -> Vec<(Username, His Mesg, UI Time(12 AM), Timestamp, is_join_online_leave_offline_msg, message ack(> or >>))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DmSessionEncryption_Key{
    pub key: [u8;32],
    pub timestamp: String,
    pub nonce: [u8;12]
}

impl DmChats_Warehouse{
    pub fn new() -> Self {
        Self { 
            dms_session_key: HashMap::new(), 
            dms_data: HashMap::new() 
        }
    }
}


//----------DM Double Ratchet Keys-------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DmE2EEncryption_Data{
    pub dms: HashMap<String, DmDoubleRatchet_Keys>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DmDoubleRatchet_Keys{
    pub root_key: [u8;32],
    pub their_old_dh_pub_key: [u8;32],
    pub dh_pub_key: [u8;32],
    pub dh_priv_key: [u8;32],
    pub sending_chain_key: [u8;32],
    pub receiving_chain_key: [u8;32],
}

impl DmE2EEncryption_Data{
    pub fn load() -> Self{
        let disk: DiskPersist<DmE2EEncryption_Data> = DiskPersist::init("persistent-dms-e2e-keys").unwrap(); 
        if let Ok(data_res) = disk.read(){
            match data_res {
                Some(data) => data,
                None => DmE2EEncryption_Data { dms: HashMap::new() }
            }
        }
        else {
            DmE2EEncryption_Data { dms: HashMap::new() }
        }
    }
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
    pub String    //message ack (> or >>)
);
//-------------------------------------------------------------


//----------Room Sender Keys-------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room_Keys{
    pub chain_key: [u8;32],
    pub my_idpriv_key: [u8;32],
    pub my_dh_pub_keys: HashMap<String, Vec<u8>>,
    pub my_sender_key_encryptions: HashMap<String, Vec<u8>>,
    pub their_idpublic_keys: HashMap<String, [u8;32]>,
    pub their_sender_keys: HashMap<String, SenderKey>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SenderKey{
    pub chain_key: [u8;32],
    pub pub_sig_key: [u8;32]
}

impl Room_Keys{
    pub fn new() -> Self{
        Self{
            chain_key: [0u8;32],
            my_idpriv_key: [0u8;32],
            my_dh_pub_keys: HashMap::new(),
            my_sender_key_encryptions: HashMap::new(),
            their_idpublic_keys: HashMap::new(),
            their_sender_keys: HashMap::new()
        }
    }
}