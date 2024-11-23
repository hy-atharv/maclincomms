
use actix_web::{web, HttpResponse, HttpRequest, Error,Responder};
use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};

use crate::{core::{encoding_token::encode_user_token, hashing_data::verify_user_password}, database::auth_db::{get_auth_data, insert_auth_data}, models::{
    jwt_models::{UserAuthenticationToken, UserClaims}, login_model:: Login_User, register_model:: Register_User, response_data::{AuthorizationResponseData, LoginResponseData, RegisterResponseData}, status_types:: StatusTypes, user_auth::User_Auth_Table
}};

use crate::core::hashing_data::hash_user_password;

use actix_ws::{Message, Session};
use futures_util::StreamExt;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;




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
    let (salt_bytes, hash_bytes) = hash_user_password(passed_user.password);
    let existing_user = get_auth_data(&passed_user.username).await;
    match existing_user {
        Ok(data) => match data.len() {
            0 => {
                if let Err(err) = insert_auth_data(web::Json(User_Auth_Table {
                    username: passed_user.username.clone(),
                    password_hash: general_purpose::STANDARD.encode(hash_bytes),
                    password_salt: general_purpose::STANDARD.encode(salt_bytes),
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
                // Assigning a JWT Access Token
                let access_tok = encode_user_token(UserClaims{
                username: user.clone(),
                exp: (Utc::now() + Duration::minutes(60)).timestamp()
                });
                // Assigning a JWT Refresh Token
                let refresh_tok = encode_user_token(UserClaims {
                username: user,
                exp: (Utc::now() + Duration::days(2)).timestamp(),
                });
                
                HttpResponse::Ok().json(RegisterResponseData {
                    status_type: StatusTypes::REGISTRATION_SUCCESSFUL,
                    exp: (Utc::now() + Duration::minutes(59)).timestamp(),
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
        Ok(data) => {
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
                exp: (Utc::now() + Duration::minutes(60)).timestamp()
            });
            // Assigning a JWT Refresh Token
            let refresh_tok = encode_user_token(UserClaims {
                username: user,
                exp: (Utc::now() + Duration::days(2)).timestamp(),
            });

            HttpResponse::Ok().json(LoginResponseData {
                status_type: StatusTypes::LOG_IN_SUCCESSFUL,
                exp: (Utc::now() + Duration::minutes(59)).timestamp(),
                access_token: access_tok,
                refresh_token: refresh_tok,
                message: "Logged In Successfully".to_owned(),
            })
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


// Shared type for managing active WebSocket sessions
type SharedState = Arc<Mutex<HashMap<Uuid, Session>>>;


#[actix_web::get("/world_chat")]
pub async fn public_chat(
    user: UserAuthenticationToken, // Extractor/Kindda Middleware for JWT validation
    req: HttpRequest,
    body: web::Payload,
    shared_state: web::Data<SharedState>, // Inject shared state
) -> actix_web::Result<impl Responder> {
    println!("Authenticated user: {:?}", user);

    // Initialize WebSocket connection
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let shared_state = shared_state.clone();
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
                    println!("User [{}] says: {}", user.username, text);

                    // Broadcast to all connected sessions
                    let sessions = shared_state.lock().unwrap();
                    for (id, s) in sessions.iter() {
                        // Skip broadcasting to the sender
                        if *id == session_id {
                            // Skip broadcasting to the sender
                            continue;
                        }
                        
                        if s.clone().text(text.clone()).await.is_err() {
                            println!("Failed to send message to a client.");
                        }
                    }

                    // If "close" message, remove session and break
                    // if text == "close" {
                    //     println!("Closing connection for user: {}", user.username);
                    //     break;
                    // }
                }
                Message::Close(_) => {
                    println!("User {} closed the connection.", user.username);
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


















//Testing Route
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