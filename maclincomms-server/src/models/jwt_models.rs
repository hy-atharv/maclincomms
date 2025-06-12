use serde::{Deserialize, Serialize};

//User Authentication Claims for Public Chat
#[derive(Serialize, Deserialize, Debug)]
pub struct UserClaims{
    pub username: String,
    pub exp: i64
 // pub exp: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthenticationTokenPayload {
    pub username: String,
}




//User Authentication Claims for Private Rooms
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoomClaims{
    pub username: String,
    pub exp: i64,
    pub room_name: String,
    pub room_key: String,
    pub role: RoomRoles
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoomAuthenticationTokenPayload {
    pub username: String,
    pub room_name: String,
    pub room_key: String,
    pub role: RoomRoles
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomRoles{
    OWNER,
    MEMBER
}




