
use actix_web::{http::{self, Error}, web::{self, Bytes}, HttpRequest, HttpResponse, Responder};
use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};
use serde_json::{json, Value};
use crate::{
    core::{encoding_token::{encode_user_room_token, encode_user_token}, hashing_data::{hash_room_password, verify_room_password, verify_user_password}}, 
    database::{
        auth_db::{get_auth_data, insert_auth_data}, dms_db::{get_blocked_list, get_dm_chats_backup_data, get_dms_list, get_dms_list_data, insert_user_to_blocked_list, insert_user_to_dms_list, insert_user_to_dms_table, remove_user_from_blocked_list, upload_dm_chats_backup_data}, redis_db::{publish_notification, queue_notification, retrieve_queued_notifications, subscribe_to_notifications}, rooms_db::{delete_room_data, get_room_data, insert_member_to_room, insert_room_data, remove_member_from_room}
    }, 
    models::{
    ack_model::AckType, dms_data:: {Accept_User, AcceptanceStatus, Add_User, Block_User, ChatData, DmSenderMessage, Dms_Table, Unblock_User}, jwt_models::{RoomRoles, UserAuthenticationTokenPayload, UserClaims, UserRoomAuthenticationTokenPayload, UserRoomClaims}, login_model:: Login_User, notification_data::{NotificationData, NotificationType}, register_model:: Register_User, response_data::{AcceptUserResponseData, AddUserResponseData, BlockUserResponseData, CreateRoomResponseData, GetDmChatsResponseData, GetDmsResponseData, JoinRoomResponseData, LoginResponseData, QueuedNotificationsReponseData, RegisterResponseData, RequestNewTokenResponseData, RoomDataResponseData, UnblockUserResponseData, UploadDmChatsResponseData}, room_data:: {Create_Room, Join_Room, RoomReceiverMessage, RoomSenderMessage, Rooms_Table, WhisperMode}, status_types:: StatusTypes, user_auth::User_Auth_Table
 }
};
use tokio::{sync::mpsc, task::futures};

use crate::core::hashing_data::hash_user_password;

use actix_ws::{CloseCode, CloseReason, Message, Session};
use futures_util::{stream, Stream, StreamExt};

use std::{collections::{HashMap}};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use passwords::PasswordGenerator;



// Simple handler for demonstration
#[actix_web::get("/greet/{user_name}")]
pub async fn greet(user_name: web::Path<String>) -> impl Responder {
    format!("Hello {user_name}")
}



#[actix_web::post("/register_user")]
pub async fn register(
   user: web::Json<Register_User> 
) -> impl Responder {
    
    let passed_user = user.into_inner();
    let user = passed_user.username.clone();
    let identity_key = passed_user.public_identity_key;
    let (salt_bytes, hash_bytes) = hash_user_password(passed_user.password);
    let existing_user = get_auth_data(&passed_user.username).await;
    match existing_user {
        Ok(data) => match data.len() {
            0 => {
                if let Err(err) = insert_auth_data(web::Json(User_Auth_Table {
                    username: passed_user.username.clone(),
                    password_hash: general_purpose::STANDARD.encode(hash_bytes),
                    password_salt: general_purpose::STANDARD.encode(salt_bytes),
                    public_identity_key: identity_key
                }))
                .await
                {
                    return HttpResponse::InternalServerError().json(RegisterResponseData {
                        status_type: StatusTypes::DATABASE_ERROR,
                        exp: 0,
                        access_token: "".to_owned(),
                        refresh_token: "".to_owned(),
                        message: format!("Failed to insert user data: {}", err),
                    });
                }

                //Inserting User to dms table
                if let Err(err) = insert_user_to_dms_table(&passed_user.username).await{
                    return HttpResponse::InternalServerError().json(RegisterResponseData {
                        status_type: StatusTypes::DATABASE_ERROR,
                        exp: 0,
                        access_token: "".to_owned(),
                        refresh_token: "".to_owned(),
                        message: format!("Failed to insert user data to dms table: {}", err),
                    });
                }

                // Assigning a JWT Access Token
                let access_tok = encode_user_token(UserClaims{
                username: user.clone(),
                exp: (Utc::now() + Duration::days(2)).timestamp()
                });
                // Assigning a JWT Refresh Token
                let refresh_tok = encode_user_token(UserClaims {
                username: user,
                exp: (Utc::now() + Duration::days(5)).timestamp(),
                });
                
                HttpResponse::Ok().json(RegisterResponseData {
                    status_type: StatusTypes::REGISTRATION_SUCCESSFUL,
                    exp: (Utc::now() + Duration::hours(47)).timestamp(),
                    access_token: access_tok,
                    refresh_token: refresh_tok,
                    message: "Registration Successful".to_owned(),
                })
            }
            non_zero => HttpResponse::Conflict().json(RegisterResponseData {
                status_type: StatusTypes::USER_ALREADY_EXISTS,
                exp: 0,
                access_token: "".to_owned(),
                refresh_token: "".to_owned(),
                message: "User already exists! Choose a unique username".to_owned(),
            }),
        },
        Err(err) => HttpResponse::InternalServerError().json(RegisterResponseData {
            status_type: StatusTypes::DATABASE_ERROR,
            exp: 0,
            access_token: "".to_owned(),
            refresh_token: "".to_owned(),
            message: format!("Internal server error because of DB error: {}", err),
        })
    }
}






#[actix_web::post("/login_user")]
pub async fn login(
   user: web::Json<Login_User> 
) -> impl Responder {
    
    let passed_user = user.into_inner();
    let user = passed_user.username;

    let found_user = get_auth_data(&user).await;

    match found_user {
        Ok(data) => match data.len() {
            //USER NOT FOUND
            0 => {
                HttpResponse::NotFound().json(LoginResponseData {
                    status_type: StatusTypes::USER_NOT_FOUND,
                    exp: 0,
                    access_token: "".to_owned(),
                    refresh_token: "".to_owned(),
                    message: "User not found".to_owned(),
                })
            },
            non_zero => {
                let salt_bytes = general_purpose::STANDARD.decode(&data[0].password_salt).unwrap();
                let hash_bytes = general_purpose::STANDARD.decode(&data[0].password_hash).unwrap();

                let allow = verify_user_password(passed_user.password, &salt_bytes, &hash_bytes);

                if allow==false {
                    return HttpResponse::Unauthorized().json(LoginResponseData {
                        status_type: StatusTypes::INVALID_CREDENTIALS,
                        exp: 0,
                        access_token: "".to_owned(),
                        refresh_token: "".to_owned(),
                        message: "Invalid Credentials".to_owned(),
                    });
                }

                // Assigning a JWT Access Token
                let access_tok = encode_user_token(UserClaims{
                    username: user.clone(),
                    exp: (Utc::now() + Duration::days(2)).timestamp()
                });
                // Assigning a JWT Refresh Token
                let refresh_tok = encode_user_token(UserClaims {
                    username: user,
                    exp: (Utc::now() + Duration::days(5)).timestamp(),
                });

                HttpResponse::Ok().json(LoginResponseData {
                    status_type: StatusTypes::LOG_IN_SUCCESSFUL,
                    exp: (Utc::now() + Duration::hours(47)).timestamp(),
                    access_token: access_tok,
                    refresh_token: refresh_tok,
                    message: "Logged In Successfully".to_owned(),
                })
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(LoginResponseData {
            status_type: StatusTypes::DATABASE_ERROR,
            exp: 0,
            access_token: "".to_owned(),
            refresh_token: "".to_owned(),
            message: format!("Internal server error because of DB error: {}", err),
        })
    }
}


//Authentication endpoint based on token
#[actix_web::get("/authN_user")]
pub async fn authenticate_user(
    user: UserAuthenticationTokenPayload,
) -> impl Responder {

    return HttpResponse::Ok();
}


//Requesting for new token with refresh token
#[actix_web::get("/new_token")]
pub async fn request_new_token(
    user: UserAuthenticationTokenPayload,
) -> impl Responder {

    let user_name = user.username;

    // Assigning a JWT Access Token
    let access_tok = encode_user_token(UserClaims{
        username: user_name.clone(),
        exp: (Utc::now() + Duration::days(2)).timestamp()
    });
    // Assigning a JWT Refresh Token
    let refresh_tok = encode_user_token(UserClaims {
        username: user_name,
        exp: (Utc::now() + Duration::days(5)).timestamp(),
    });
        
    return HttpResponse::Ok().json(RequestNewTokenResponseData {
            status_type: StatusTypes::TOKEN_GRANTED,
            exp: (Utc::now() + Duration::hours(47)).timestamp(),
            access_token: access_tok,
            refresh_token: refresh_tok,
            message: "New Token granted".to_owned(),
    });
}




// Shared Arc type for managing active WebSocket sessions in Public Chat
type WorldChatSharedState = Arc<Mutex<HashMap<Uuid, Session>>>;


#[actix_web::get("/world_chat")]
pub async fn public_chat(
    user: UserAuthenticationTokenPayload, // Extractor/Kindda Middleware for JWT validation
    req: HttpRequest,
    body: web::Payload,
    shared_state: web::Data<WorldChatSharedState>, // Inject shared state
) -> actix_web::Result<impl Responder> {

    // Initialize WebSocket connection
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;


    let shared_state = shared_state.clone();
    //Session ID is username
    let session_id = Uuid::new_v4();

    // Add this session to the shared state
    {
        let mut sessions = shared_state.lock().unwrap();
        sessions.insert(session_id, session.clone());
    }

    // Spawn an asynchronous task to handle WebSocket messages
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(text) => {

                    //Send server acknowledgment to sender
                    if session.binary(AckType::ServerAck.byte()).await.is_err(){
                        println!("Failed to send server ack to sender.");
                    }

                    // Broadcast to all connected sessions except sender
                    let mut sessions = shared_state.lock().unwrap();
                    for (id, s) in sessions.iter_mut() {
                        // Skip broadcasting to the sender
                        if *id == session_id {
                            // Skip broadcasting to the sender
                            continue;
                        }
                        
                        if s.text(text.clone()).await.is_err() {
                            println!("Failed to send message to a client.");
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => break,
            }
        }

        // Remove session from shared state when connection closes
        {
            let mut sessions = shared_state.lock().unwrap();
            sessions.remove(&session_id);
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}



// Shared Arc type for managing active rooms with active Websocket Sessions
type RoomChatSharedState = Arc<Mutex<HashMap<(String, String), HashMap<String, Session>>>>;



#[actix_web::post("/create_room")]
pub async fn create_room(
    user: UserAuthenticationTokenPayload,
    body: web::Json<Create_Room>,
    rooms_shared_state: web::Data<RoomChatSharedState>
) -> impl Responder {
    
    let user_name = user.username;

    let passed_room = body.into_inner();

    let mut pgi = PasswordGenerator::new().length(9).numbers(true).lowercase_letters(true).uppercase_letters(true).symbols(true).exclude_similar_characters(false).try_iter().unwrap();

    let key = pgi.next().unwrap();

    let key_hash = hash_room_password(key.clone());

    let existing_room = get_room_data(&passed_room.room_name).await;

    match existing_room {
        Ok(data) => match data.len() {
            0 => {
                let mut m_keys = Vec::<Value>::new();
                m_keys.push(json!({ user_name.clone() : passed_room.owner_key }));

                if let Err(err) = insert_room_data(web::Json(Rooms_Table {
                    room_name: passed_room.room_name.clone(),
                    room_key: key_hash.clone(),
                    room_owner: user_name.clone(),
                    room_members: vec![user_name.clone()],
                    members_keys: m_keys
                }))
                .await
                {
                    return HttpResponse::InternalServerError().json(CreateRoomResponseData {
                        status_type: StatusTypes::DATABASE_ERROR,
                        room_token: "".to_owned(),
                        room_name: passed_room.room_name,
                        room_key: "".to_owned(),
                        message: "Room couldnt be created".to_owned(),
                    });
                }
                // Assigning a JWT Room Access Token
                let room_access_tok = encode_user_room_token(UserRoomClaims{
                    username: user_name,
                    exp: (Utc::now() + Duration::minutes(10)).timestamp(),
                    room_name: passed_room.room_name.clone(),
                    room_key: key_hash.clone(),
                    role: RoomRoles::OWNER
                });
            
                // Lock the shared state and insert the new room
                {
                    let mut rooms = rooms_shared_state.lock().unwrap();
                    rooms.insert((passed_room.room_name.clone(), key_hash.clone()), HashMap::new()); // Insert an empty room
                } 
                // Mutex lock is dropped here
            
                HttpResponse::Ok().json(CreateRoomResponseData {
                    status_type: StatusTypes::ROOM_CREATION_SUCCESSFUL,
                    room_token: room_access_tok,
                    room_name: passed_room.room_name,
                    room_key: key,
                    message: "Room Created Successfully".to_owned(),
                })
            }
            non_zero => HttpResponse::Conflict().json(CreateRoomResponseData {
                status_type: StatusTypes::ROOM_ALREADY_EXISTS,
                room_token: "".to_owned(),
                room_name: passed_room.room_name,
                room_key: "".to_owned(),
                message: "Room already exists with that name".to_owned(),
            }),
        },
        Err(err) => HttpResponse::InternalServerError().json(CreateRoomResponseData {
            status_type: StatusTypes::DATABASE_ERROR,
            room_token: "".to_owned(),
            room_name: passed_room.room_name,
            room_key: "".to_owned(),
            message: "Room couldnt be created".to_owned(),
        })
    } 
}


#[actix_web::post("/join_room")]
pub async fn join_room(
    user: UserAuthenticationTokenPayload,
    body: web::Json<Join_Room>
) -> impl Responder {
    
    let user_name = user.username;

    let passed_room = body.into_inner();

    let room_key = passed_room.room_key.clone();


    let existing_room = get_room_data(&passed_room.room_name).await;

    match existing_room {
        Ok(data) => match data.len() {
            0 => {
                HttpResponse::NotFound().json(JoinRoomResponseData {
                    status_type: StatusTypes::ROOM_NOT_FOUND,
                    room_token: "".to_owned(),
                    room_name: passed_room.room_name,
                    message: "Room could not be found".to_owned(),
                })
            },
            non_zero => {
                //Validating Room Key
                let room_key_hash = data[0].room_key.clone();
                let allow = verify_room_password(room_key, room_key_hash.clone());
                //matching hash
                if allow == true {
                    //Inserting member to room
                    if let Err(err) = insert_member_to_room(&user_name, &passed_room.room_name).await
                    {
                        return HttpResponse::InternalServerError().json(JoinRoomResponseData {
                            status_type: StatusTypes::DATABASE_ERROR,
                            room_token: "".to_owned(),
                            room_name: passed_room.room_name,
                            message: "Couldnt join room because of DB Error".to_owned(),
                        });
                    }
                    //Assign room token
                    let room_access_tok = encode_user_room_token(UserRoomClaims{
                        username: user_name,
                        exp: (Utc::now() + Duration::minutes(10)).timestamp(),
                        room_name: passed_room.room_name.clone(),
                        room_key: room_key_hash,
                        role: RoomRoles::MEMBER
                    });
            
                    return HttpResponse::Ok().json(JoinRoomResponseData {
                        status_type: StatusTypes::ROOM_AUTHORIZATION_SUCCESSFUL,
                        room_token: room_access_tok,
                        room_name: passed_room.room_name,
                        message: "Room Joined Successfully".to_owned(),
                    });
                }
                else {
                    return HttpResponse::Unauthorized().json(JoinRoomResponseData {
                        status_type: StatusTypes::INVALID_CREDENTIALS,
                        room_token: "".to_owned(),
                        room_name: passed_room.room_name,
                        message: "Invalid room key".to_owned(),
                    });
                }

            }
        },
        Err(err) => HttpResponse::InternalServerError().json(JoinRoomResponseData {
            status_type: StatusTypes::DATABASE_ERROR,
            room_token: "".to_owned(),
            room_name: passed_room.room_name,
            message: "Couldnt join room because of DB Error".to_owned(),
        })
    }
}

#[actix_web::get("/room_data")]
pub async fn retrieve_room_data(
    user: UserRoomAuthenticationTokenPayload
) -> impl Responder {

    let room_name = user.room_name;

    let existing_room = get_room_data(&room_name).await;

    match existing_room {
        Ok(data) => match data.len() {
            0 => {
                HttpResponse::NotFound().json(RoomDataResponseData {
                    status_type: StatusTypes::ROOM_NOT_FOUND,
                    room_data: Rooms_Table { room_name: "".to_string(), room_key: "".to_string(), room_owner: "".to_string(), room_members: Vec::new(), members_keys: Vec::new() },
                    message: "Room could not be found".to_owned(),
                })
            },
            non_zero => {
                HttpResponse::Ok().json(RoomDataResponseData {
                    status_type: StatusTypes::ROOM_FOUND,
                    room_data: data[0].clone(),
                    message: "Room Data fetched".to_string()
                })
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(RoomDataResponseData {
            status_type: StatusTypes::DATABASE_ERROR,
            room_data: Rooms_Table { room_name: "".to_string(), room_key: "".to_string(), room_owner: "".to_string(), room_members: Vec::new(), members_keys: Vec::new() },
            message: "Couldnt retrieve data from DB".to_owned(),
        })
    }
}


#[actix_web::get("/room_chat")]
pub async fn private_room_chat(
    user: UserRoomAuthenticationTokenPayload, // Extractor/Kindda Middleware for JWT validation
    req: HttpRequest,
    body: web::Payload,
    rooms_shared_state: web::Data<RoomChatSharedState>, // Inject shared state
) -> actix_web::Result<impl Responder> {

    // Initialize WebSocket connection
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let room_name = user.room_name;
    let room_key = user.room_key;
    let username = user.username;
    let role = user.role;

    let shared_state = rooms_shared_state.clone();

    // Add this session to the shared state
    {
        let mut rooms = shared_state.lock().unwrap();

        // Inserting user sesssion into an existing room
        if let Some(room_sessions) = rooms.get_mut(&(room_name.clone(), room_key.clone())){
            room_sessions.insert(username.clone(), session.clone());
        }
        else{
            return Err(actix_web::error::ErrorNotFound("Room not found"));
        }
         
    }

    // Spawn an asynchronous task to handle WebSocket messages
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(text) => {

                    // Broadcast to all connected sessions in a room
                    let rooms = shared_state.lock().unwrap();
                    if let Some(room_sessions) = rooms.get(&(room_name.clone(), room_key.clone())){

                        let sender_session = room_sessions.get(&username);

                        //Parse to Struct
                        let sender_msg_res = serde_json::from_str::<RoomSenderMessage>(&text);

                        let sender_msg = match sender_msg_res{
                            Ok(m) => m,
                            Err(e) => RoomSenderMessage { username: "".to_string(), content: "".to_string(), users: Vec::new(), whisper_mode: WhisperMode::NONE, is_join_leave_msg: false }
                        };

                        //Send server acknowledgment to sender only for normal messages
                        if sender_msg.is_join_leave_msg==false{
                            if sender_session.unwrap().clone().binary(AckType::ServerAck.byte()).await.is_err(){
                                println!("Failed to send server ack to sender.");
                            }
                        }

                        
                        //Parse list of users to send or hide from
                        let users_list = sender_msg.users;
                        
                        //Parse whisper mode
                        let mode = sender_msg.whisper_mode;

                        //Final message to send
                        let msg_struct = RoomReceiverMessage{
                            username: sender_msg.username,
                            content: sender_msg.content,
                            is_join_leave_msg: sender_msg.is_join_leave_msg
                        };
                        let msg = serde_json::to_string(&msg_struct).unwrap();

                        //Check to Send normal message
                        if msg_struct.content!="  ".to_string(){
                            //match for whisper mode to share with or hide from
                            match mode{
                                WhisperMode::HIDE_FROM => {
                                    for (id, s) in room_sessions.iter() {
                                        if users_list.contains(id) || *id==username.clone(){
                                        // Skip broadcasting to the users to hide from
                                            continue;
                                        }
                                        //Sending Message
                                        if s.clone().text(msg.clone()).await.is_err() {
                                            println!("Failed to send message to a client.");
                                        }
                                    }
                                }
                                WhisperMode::SHARE_WITH => {
                                    for (id, s) in room_sessions.iter() {
                                        if !users_list.contains(id) || *id==username.clone() {
                                        // Skip broadcasting to the users not in the list
                                            continue;
                                        }
                                        //Sending Message
                                        if s.clone().text(msg.clone()).await.is_err() {
                                            println!("Failed to send message to a client.");
                                        }
                                    }
                                }
                                WhisperMode::NONE => {
                                    for (id, s) in room_sessions.iter() {
                                        if *id == username.clone() {
                                        // Skip broadcasting to the sender, but to everyone else
                                            continue;
                                        }
                                        //Sending Message
                                        if s.clone().text(msg.clone()).await.is_err() {
                                            println!("Failed to send message to a client.");
                                        }
                                    }
                                }
                            }
                        }
                        //Send key rotation informer bytes
                        else{
                            let u = msg_struct.username.as_bytes();
                            let mut i: Vec<u8> = [0x33].to_vec();
                            i.extend_from_slice(u);
                            match mode{
                                WhisperMode::HIDE_FROM => {
                                    for (id, s) in room_sessions.iter() {
                                        if users_list.contains(id) || *id==username.clone(){
                                        // Skip broadcasting to the users to hide from
                                            continue;
                                        }
                                        //Sending Message
                                        if s.clone().binary(i.clone()).await.is_err() {
                                            println!("Failed to send message to a client.");
                                        }
                                    }
                                }
                                WhisperMode::SHARE_WITH => {
                                    for (id, s) in room_sessions.iter() {
                                        if !users_list.contains(id) || *id==username.clone() {
                                        // Skip broadcasting to the users not in the list
                                            continue;
                                        }
                                        //Sending Message
                                        if s.clone().binary(i.clone()).await.is_err() {
                                            println!("Failed to send message to a client.");
                                        }
                                    }
                                }
                                WhisperMode::NONE => {}
                            }
                        }
                    }
                    else{
                        let _ = session.clone().close(Some(CloseReason { code: CloseCode::Normal, description: Some("Room Closed".to_string()) })).await;
                    }
                }
                Message::Binary(data) => {
                    let bytes = data.as_ref();
                    //Checking for sender key descriptor
                    if bytes[0] == 0x11 {
                        let rooms = shared_state.lock().unwrap();
                        if let Some(room_sessions) = rooms.get(&(room_name.clone(), room_key.clone())){
                            let receiving_username_bytes = &bytes[113..];
                            let receiving_username_res = String::from_utf8(receiving_username_bytes.to_vec());
                            if let Ok(receiving_username) = receiving_username_res{
                                let receiving_user_session = room_sessions.get(&receiving_username);
                                //Modify receiving username at end with sending username
                                let sending_username_bytes = username.as_bytes();
                                let mut modified_bytes = bytes[..113].to_vec(); // take first 113 bytes
                                modified_bytes.extend_from_slice(sending_username_bytes); // append sending username
                                //Send sender_key to user
                                if receiving_user_session.unwrap().clone().binary(modified_bytes).await.is_err(){
                                    println!("Failed to send sender key to user.");
                                }
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => break,
            }
        }

        // Remove user session or Hashmap of Sessions for a Room from shared state when connection closes
        // Remove and update the ROOMS table too 
        match role {
            RoomRoles::OWNER => {    //IF OWNER DISCONNECTS, DELETE ROOM
                let __ = delete_room_data(&room_name).await;

                {
                    let mut rooms = shared_state.lock().unwrap();
                    if let Some(room_sessions) = rooms.get_mut(&(room_name.clone(), room_key.clone())){
                        //Closing all user sessions in the room when its owner leaves
                        for session in room_sessions.clone(){
                            let _ = session.1.close(Some(CloseReason { code: CloseCode::Normal, description: Some("Room Closed".to_string()) })).await;
                            
                        }
                        //Removing ROOM
                        rooms.remove(&(room_name.clone(), room_key));
                    }
                }
            }
            RoomRoles::MEMBER => {  //FOR MEMBERS
                {
                    let mut rooms = shared_state.lock().unwrap();
                    if let Some(room_sessions) = rooms.get_mut(&(room_name.clone(), room_key)) {
                        room_sessions.remove(&username);
                    }
                }
                let __ = remove_member_from_room(&username, &room_name).await;

                // Closing user session if any
                let _ = session.close(None).await;
            }
        }

        
    });

    Ok(response)
}




//------------------------------------DM CHATS--------------------------------------------------



// Shared Arc type for managing active dms with 2 active Websocket Sessions
type DMChatSharedState = Arc<Mutex<HashMap<(String, String), HashMap<String, Session>>>>;


#[actix_web::post("/add_user")]
pub async fn add_user(
    user: UserAuthenticationTokenPayload,
    body: web::Json<Add_User>
) -> impl Responder {

    let user_name = user.username;

    let passed_data = body.into_inner();

    let add_username = passed_data.username_to_add;

    let message = passed_data.message;

    let found_user = get_auth_data(&add_username).await;

    match found_user {
        Ok(data) => match data.len() {
            //USER NOT FOUND
            0 => {
                HttpResponse::NotFound().json(AddUserResponseData {
                    status_type: StatusTypes::USER_NOT_FOUND,
                    message: "User not found".to_owned(),
                })
            },
            non_zero => {
                let blocked_list = get_blocked_list(&add_username).await;
                let mut is_blocked = false;
                match blocked_list {
                    Ok(data) => {
                        let is_blocked = data
                            .get(0)
                            .map(|b| b.blocked_list.contains(&user_name))
                            .unwrap_or(false);

                        match is_blocked {
                            true => { // DENYING TO ADD USER WHO BLOCKED YOU BY USER_NOT_FOUND
                                return HttpResponse::NotFound().json(AddUserResponseData {
                                    status_type: StatusTypes::USER_NOT_FOUND,
                                    message: "User not found".to_owned(),
                                });
                            }
                            false => {
                                //PUB TO PUB/SUB CHANNEL FOR ONLINE USER
                                let pub_res = publish_notification(NotificationData{
                                    n_type: NotificationType::ADD_REQUEST,
                                    from: user_name.clone(),
                                    to: add_username.clone(),
                                    content: message.clone()
                                }).await;

                                match pub_res {
                                    Ok(subscribers) => {
                                        if subscribers==0 { //USER IS OFFLINE
                                            //QUEUE TO LIST FOR OFFLINE USER
                                            if let Err(err) = queue_notification(NotificationData{
                                                n_type: NotificationType::ADD_REQUEST,
                                                from: user_name.clone(),
                                                to: add_username,
                                                content: message.clone()
                                            })
                                            .await
                                            {
                                                return HttpResponse::InternalServerError().json(AddUserResponseData {
                                                    status_type: StatusTypes::NOTIFICATIONS_ERROR,
                                                    message: format!("Notifications Error due to Redis Error"),
                                                });
                                            }
                                        }
                                        else { //NON ZERO NUMBER OF SUBS MEANS USER IS SUBSCRIBED CURRENTLY
                                            return HttpResponse::Ok().json(AddUserResponseData {
                                                status_type: StatusTypes::ADD_REQUEST_SENT,
                                                message: "Request sent to user".to_owned(),
                                            });
                                        }

                                        return HttpResponse::Ok().json(AddUserResponseData {
                                            status_type: StatusTypes::ADD_REQUEST_SENT,
                                            message: "Request sent to user".to_owned(),
                                        });
                                    }
                                    Err(err) => {
                                        return HttpResponse::InternalServerError().json(AddUserResponseData {
                                            status_type: StatusTypes::NOTIFICATIONS_ERROR,
                                            message: format!("Notifications Error due to Redis Error"),
                                        });
                                    }
                                }  
                            }
                        }
                    }
                    Err(err) => {
                        return HttpResponse::InternalServerError().json(AddUserResponseData {
                            status_type: StatusTypes::DATABASE_ERROR,
                            message: format!("Internal server error because of DB error: {}", err),
                        });
                    }
                }
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(AddUserResponseData {
            status_type: StatusTypes::DATABASE_ERROR,
            message: format!("Internal server error because of DB error: {}", err),
        })
    }
}



#[actix_web::post("/accept_user")]
pub async fn accept_user(
    user: UserAuthenticationTokenPayload,
    body: web::Json<Accept_User>
) -> impl Responder {

    let user_name = user.username;

    let passed_data = body.into_inner();

    let user_to_add = passed_data.username_to_accept;

    let status = passed_data.acceptance_status;

    match status {
        AcceptanceStatus::ACCEPTED => {
            //PUB TO PUB/SUB CHANNEL FOR ONLINE USER
            let pub_res = publish_notification(NotificationData{
                n_type: NotificationType::ACCEPTED,
                from: user_name.clone(),
                to: user_to_add.clone(),
                content: format!("{} accepted your add request", user_name.clone())
            }).await;

            match pub_res {
                Ok(subscribers) => {
                    if subscribers==0 { //USER IS OFFLINE
                        //QUEUE TO LIST FOR OFFLINE USER
                        if let Err(err) = queue_notification(NotificationData{
                            n_type: NotificationType::ACCEPTED,
                            from: user_name.clone(),
                            to: user_to_add.clone(),
                            content: format!("{} accepted your add request", user_name.clone())
                        })
                        .await
                        {
                            if let Err(err) = insert_user_to_dms_list(&user_name, &user_to_add).await{
                                return HttpResponse::InternalServerError().json(AcceptUserResponseData {
                                    status_type: StatusTypes::DATABASE_ERROR,
                                    message: format!("Internal server error because of DB error: {}", err),
                                })
                            }

                            return HttpResponse::Ok().json(AddUserResponseData {
                                status_type: StatusTypes::NOTIFICATIONS_ERROR,
                                message: format!("Notifications Error due to Redis Error"),
                            });
                        }
                    }
                    else { //NON ZERO NUMBER OF SUBS MEANS USER IS SUBSCRIBED CURRENTLY
                        if let Err(err) = insert_user_to_dms_list(&user_name, &user_to_add).await{
                            return HttpResponse::InternalServerError().json(AcceptUserResponseData {
                                status_type: StatusTypes::DATABASE_ERROR,
                                message: format!("Internal server error because of DB error: {}", err),
                            })
                        }

                        return HttpResponse::Ok().json(AcceptUserResponseData {
                            status_type: StatusTypes::USER_ADDED,
                            message: "User is added".to_owned(),
                        });
                    }

                    if let Err(err) = insert_user_to_dms_list(&user_name, &user_to_add).await{
                        return HttpResponse::InternalServerError().json(AcceptUserResponseData {
                            status_type: StatusTypes::DATABASE_ERROR,
                            message: format!("Internal server error because of DB error: {}", err),
                        })
                    }

                    return HttpResponse::Ok().json(AcceptUserResponseData {
                        status_type: StatusTypes::USER_ADDED,
                        message: "User is added".to_owned(),
                    });
                }
                Err(err) => {
                    return HttpResponse::InternalServerError().json(AcceptUserResponseData {
                        status_type: StatusTypes::NOTIFICATIONS_ERROR,
                        message: format!("Notifications Error due to Redis Error"),
                    });
                }
            }
        }
        AcceptanceStatus::IGNORED => {
            return HttpResponse::Ok().json(AcceptUserResponseData {
                status_type: StatusTypes::USER_IGNORED,
                message: format!("User's add request ignored"),
            });
        }
    }
}


#[actix_web::get("/get_dms")]
pub async fn get_dms_data(
    user: UserAuthenticationTokenPayload
) -> impl Responder {

    let username = user.username;

    let get_dms_res = get_dms_list_data(&username).await;

    match get_dms_res{
        Ok(dms_data) => {
            HttpResponse::Ok().json(GetDmsResponseData {
                status_type: StatusTypes::DMS_DATA_FETCHED,
                data: dms_data,
                message: format!("Fetched Dms Data Succesfully"),
            })
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(GetDmsResponseData {
                status_type: StatusTypes::DATABASE_ERROR,
                data: Vec::new(),
                message: format!("DB Error: {}", err),
            })
        }
    }
}


#[actix_web::get("/get_dm_chats")]
pub async fn get_dm_chats_data(
    user: UserAuthenticationTokenPayload
) -> impl Responder {

    let username = user.username;

    let get_dm_chats_res = get_dm_chats_backup_data(&username).await;

    match get_dm_chats_res{
        Ok(dms_data) => {
            HttpResponse::Ok().json(GetDmChatsResponseData {
                status_type: StatusTypes::DMS_DATA_FETCHED,
                data: dms_data,
                message: format!("Fetched Dm Chats Data Succesfully"),
            })
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(GetDmChatsResponseData {
                status_type: StatusTypes::DATABASE_ERROR,
                data: ChatData(Vec::new()),
                message: format!("DB Error: {}", err),
            })
        }
    }
}


#[actix_web::post("/upload_dm_chats")]
pub async fn upload_dm_chats_data(
    user: UserAuthenticationTokenPayload,
    body: web::Json<ChatData>
) -> impl Responder {

    let username = user.username;
    let chats_data = body.into_inner();
    let upload_chats_result = upload_dm_chats_backup_data(&username, chats_data).await;
    match upload_chats_result{
        Ok(()) => {
            HttpResponse::Ok().json(UploadDmChatsResponseData {
                status_type: StatusTypes::DMS_DATA_UPLOADED,
                message: "Dm Chats Uploaded".to_string()
            })
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(UploadDmChatsResponseData {
                status_type: StatusTypes::DATABASE_ERROR,
                message: format!("DB Error: {}", err),
            })
        }
    }
}



#[actix_web::get("/dm_chat")]
pub async fn private_dm_chat(
    user: UserAuthenticationTokenPayload, // Extractor/Kindda Middleware for JWT validation
    req: HttpRequest,
    body: web::Payload,
    target: web::Query<HashMap<String, String>>,
    dms_shared_state: web::Data<DMChatSharedState>, // Inject shared state
) -> actix_web::Result<impl Responder> {

    let from_username = user.username;
    println!("{:?}", from_username.clone());

    let mut to_username = "".to_string();

    match target.get("target"){
        Some(name) => {
            to_username = name.to_string();
        }
        None => {
            return Err(actix_web::error::ErrorBadRequest("No target user provided"));
        }
    }


    let dms_lists_res = get_dms_list(&from_username, &to_username).await;

    match dms_lists_res{
        Ok(data) => {
            let mut sender_has_added = false;
            let mut receiver_has_added = false;
            
            for d in &data {
                if d.username == from_username {
                    sender_has_added = d.dms_list.contains(&to_username);
                } else if d.username == to_username {
                    receiver_has_added = d.dms_list.contains(&from_username);
                }
            }
            

            //Denying DM Access
            if sender_has_added!=true || receiver_has_added!=true{
                return Err(actix_web::error::ErrorUnauthorized("DM restricted"));
            }
        }
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(format!("DB Error: {}",e)));
        }
    }


    // Initialize WebSocket connection
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;


    let shared_state = dms_shared_state.clone();

    // Sort usernames to form a consistent tuple key (e.g., (A, B) always even if user A or B initiates)
    let mut users = vec![from_username.clone(), to_username.clone()];
    users.sort(); // Alphabetical sort

    //Unique Tuple for every DM
    let dm_id = (users[0].clone(), users[1].clone());

    // Add this session to the shared state
    {
        let mut dms = shared_state.lock().unwrap();

        // Inserting user sesssion into an existing dm, if not creating new hashmap of sessions
        if let Some(dm_sessions) = dms.get_mut(&(dm_id)){
            dm_sessions.insert(from_username.clone(), session.clone());
        }
        else{
            let mut session_map = HashMap::new();
            session_map.insert(from_username.clone(), session.clone());
            dms.insert(dm_id.clone(), session_map);
        }
         
    }

    // Spawn an asynchronous task to handle WebSocket messages
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(text) => {

                    let dms = shared_state.lock().unwrap();
                    if let Some(dm_sessions) = dms.get(&(dm_id)){
                        let receiver_session = dm_sessions.get(&to_username);
                        let sender_session = dm_sessions.get(&from_username);

                        //Parse to Struct
                        let sender_msg_res = serde_json::from_str::<DmSenderMessage>(&text);

                        let is_online_offline_msg = match sender_msg_res{
                            Ok(m) => m.is_online_offline_msg,
                            Err(e) => true //Treating as online/offline message during error to avoid sending any notification and ack
                        };

                        //Send server acknowledgment to sender only for normal messages
                        if is_online_offline_msg==false{
                            if sender_session.unwrap().clone().binary(AckType::ServerAck.byte()).await.is_err(){
                                println!("Failed to send server ack to sender.");
                            }
                        }
                        
                        //receiver is connected to ws
                        if dm_sessions.len()==2 {
                            if receiver_session.unwrap().clone().text(text).await.is_err(){
                                println!("Failed to send message to a client.");
                            }
                        }
                        //receiver has disconnected from ws
                        else if dm_sessions.len()==1 && is_online_offline_msg==false{
                            //PUB TO PUB/SUB CHANNEL FOR ONLINE RECEIVER
                            let pub_res = publish_notification(NotificationData{ 
                                n_type: NotificationType::MESSAGE, 
                                from: from_username.clone(), 
                                to: to_username.clone(), 
                                content: text.to_string()
                            }).await;

                            match pub_res{
                                Ok(subscribers) => {
                                    if subscribers==0 { //RECEIVER IS OFFLINE
                                        //QUEUE TO LIST FOR OFFLINE RECEIVER
                                        if let Err(err) = queue_notification(NotificationData{ 
                                            n_type: NotificationType::MESSAGE, 
                                            from: from_username.clone(), 
                                            to: to_username.clone(), 
                                            content: text.to_string()
                                        })
                                        .await
                                        {
                                            println!("Couldnt queue messg notif: {}", err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    println!("Redis error in messg notif: {}", err);
                                }
                            }
                            
                        }
                        
                    }
                    else{
                        let _ = session.clone().close(Some(CloseReason { code: CloseCode::Normal, description: Some("Room Closed".to_string()) })).await;
                    }
                }
                Message::Binary(bytes) => {
                    if bytes.as_ref()==AckType::ReceiverAck.byte(){
                        let dms = shared_state.lock().unwrap();
                        if let Some(dm_sessions) = dms.get(&(dm_id)){
                            let sender_session = dm_sessions.get(&to_username);
                            //Send receiver acknowledgment from receiver to sender
                            if sender_session.unwrap().clone().binary(AckType::ReceiverAck.byte()).await.is_err(){
                                println!("Failed to send receiver ack to sender.");
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => break,
            }
        }

        // Remove session from shared state when connection closes
        {
            let mut dms = dms_shared_state.lock().unwrap();
            if let Some(dm_sessions) = dms.get_mut(&(dm_id)) {
                dm_sessions.remove(&from_username);
            }
        }

        //closing user dm session
        let _ = session.close(None).await;     
    });

    Ok(response)
}






#[actix_web::post("/block_user")]
pub async fn block_user(
    user: UserAuthenticationTokenPayload,
    body: web::Json<Block_User>,
    dms_shared_state: web::Data<DMChatSharedState>
) -> impl Responder {

    let user_name = user.username;

    let passed_data = body.into_inner();

    let user_to_block = passed_data.username_to_block;

    let b_list_res = get_blocked_list(&user_name).await;

    match b_list_res {
        Ok(data) => {
            let already_blocked = data
                .get(0)
                .map(|b| b.blocked_list.contains(&user_to_block))
                .unwrap_or(false);

            match already_blocked {
                true => {
                    return HttpResponse::Ok().json(BlockUserResponseData {
                        status_type: StatusTypes::USER_ALREADY_BLOCKED,
                        message: format!("User is already blocked"),
                    }); 
                }
                false => {
                    if let Err(err) = insert_user_to_blocked_list(&user_name, &user_to_block).await{
                        return HttpResponse::InternalServerError().json(BlockUserResponseData {
                            status_type: StatusTypes::DATABASE_ERROR,
                            message: format!("Internal server error because of DB error: {}", err),
                        });
                    }

                    let mut users = vec![user_name, user_to_block];
                    users.sort(); // Alphabetical sort
                    //Unique Tuple for every DM
                    let dm_id = (users[0].clone(), users[1].clone());

                    {
                        let mut dms = dms_shared_state.lock().unwrap();

                        // Deleting dm sessions hashmap
                        if dms.contains_key(&dm_id) {
                            dms.remove(&dm_id);
                        }                        
                    }
                    
                    return HttpResponse::Ok().json(BlockUserResponseData {
                        status_type: StatusTypes::USER_BLOCKED,
                        message: format!("User is blocked"),
                    }); 
                }
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(BlockUserResponseData {
                status_type: StatusTypes::DATABASE_ERROR,
                message: format!("Internal server error because of DB error: {}", err),
            });
        }
    }
}




#[actix_web::post("/unblock_user")]
pub async fn unblock_user(
    user: UserAuthenticationTokenPayload,
    body: web::Json<Unblock_User>
) -> impl Responder {

    let user_name = user.username;

    let passed_data = body.into_inner();

    let user_to_unblock = passed_data.username_to_unblock;

    let b_list_res = get_blocked_list(&user_name).await;

    match b_list_res {
        Ok(data) => {
            let is_blocked = data
                .get(0)
                .map(|b| b.blocked_list.contains(&user_to_unblock))
                .unwrap_or(false);

            match is_blocked {
                true => {
                    if let Err(err) = remove_user_from_blocked_list(&user_name, &user_to_unblock).await{
                        return HttpResponse::InternalServerError().json(UnblockUserResponseData {
                            status_type: StatusTypes::DATABASE_ERROR,
                            message: format!("Internal server error because of DB error: {}", err),
                        });
                    }
                    
                    return HttpResponse::Ok().json(UnblockUserResponseData {
                        status_type: StatusTypes::USER_UNBLOCKED,
                        message: format!("User is unblocked"),
                    }); 
                }
                false => {
                    return HttpResponse::Ok().json(UnblockUserResponseData {
                        status_type: StatusTypes::USER_UNBLOCKED,
                        message: format!("User is unblocked"),
                    });
                }
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(UnblockUserResponseData {
                status_type: StatusTypes::DATABASE_ERROR,
                message: format!("Internal server error because of DB error: {}", err),
            });
        }
    }
}












//______________NOTIFICATIONS_________________________________________!


#[actix_web::get("/realtime_notifications")]
pub async fn realtime_notifications(
    user: UserAuthenticationTokenPayload,
    req: HttpRequest,
) -> impl Responder {

    let username = user.username;

    //Channel for broadcasting
    let (task_sender, mut task_receiver) = mpsc::channel::<NotificationData>(100);

    tokio::spawn(async move {
        let s = subscribe_to_notifications(&username, task_sender).await;
        println!("{:?}", s);
    });

    
    let sse_stream = stream::unfold(task_receiver, |mut rx| async {
        match rx.recv().await {
            Some(notification) => {
                let json = serde_json::to_string(&notification).unwrap_or_else(|_| "{}".to_string());
                let chunk = format!("data: {}\n\n", json);
                Some((Ok::<actix_web::web::Bytes, actix_web::Error>(Bytes::from(chunk)), rx))
            }
            None => None,
        }
    });

    HttpResponse::Ok()
        .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(sse_stream)
}



#[actix_web::get("/queued_notifications")]
pub async fn queued_notifications(
    user: UserAuthenticationTokenPayload,
    req: HttpRequest,
) -> impl Responder {

    let username = user.username;
    let queued_notifications = retrieve_queued_notifications(&username).await;

    match queued_notifications {
        Ok(n_data) => {
            HttpResponse::Ok().json(QueuedNotificationsReponseData {
                status_type: StatusTypes::NOTIFICATIONS_FETCHED_SUCCESSFULLY,
                data: n_data,
                message: format!("Retrieved Notifications Sucessfully"),
            })
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(QueuedNotificationsReponseData {
                status_type: StatusTypes::NOTIFICATIONS_ERROR,
                data: HashMap::new(),
                message: format!("Couldnt retrieve notifications due to redis error: {}", err),
            })
        }
    }
}

































//----------------------------------------Testing Route--------------------------------------------------------------
#[actix_web::get("/ws")]
async fn echo(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(msg) => {
                    println!("Got text: {msg}");
        
                    // Check if the client sent "close" to stop the connection
                    if msg == "close" {
                        println!("Closing connection as requested by client.");
                        let _ = session.close(None).await;
                        break;
                    }
                }
                Message::Close(_) => {
                    println!("Client closed the connection.");
                    break;
                }
                _ => break,
            }
        }
        
    });

    Ok(response)
}