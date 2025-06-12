
use std::collections::HashMap;

use disk_persist::DiskPersist;
use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use ratatui::{style::Style, text::Line, widgets::{Block, Borders, Paragraph}};
use reqwest::{Client, StatusCode};
use throbber_widgets_tui::CLOCK;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::{client::{self, IntoClientRequest}, http::Request, ClientRequestBuilder, Message}, WebSocketStream};

use crate::{crypto::identity_pair::generate_identity_keypair, endpoints::Endpoints, tui_main::MaclincommsApp, tui_widgets::register_textarea::{RegisterTaskStatus, RegisterTextArea}, user_model::{DmE2EEncryption_Data, DmSessionEncryption_Key, RegisterResponse, StatusTypes, UserRegisterPayload}};

#[derive(Debug)]
pub enum RegisterResponseResult{
    TOKEN(String),
    EXISTING_USER,
    DATABASE_ERROR,
    UNKNOWN_ERROR,
    REQUEST_ERROR
}


pub async fn register(username: String, password: String) -> (String, RegisterResponseResult, String, i64){

    // Initialising Persistent Dms E2E Keys File
    let persistent_dm_e2e_keys: DiskPersist<DmE2EEncryption_Data> = DiskPersist::init("persistent-dms-e2e-keys").unwrap();
    let init_data = DmE2EEncryption_Data { dms: HashMap::new()};
    persistent_dm_e2e_keys.write(&init_data).unwrap();

    // Initialising Persistent Dms Session Keys File
    let persistent_dm_session_keys: DiskPersist<HashMap<String, DmSessionEncryption_Key>> = DiskPersist::init("persistent-dms-session-keys").unwrap();
    let init_data = HashMap::new();
    persistent_dm_session_keys.write(&init_data).unwrap();
    

    // Trim whitespace (including newline characters) from the input
    let username = username.trim().to_owned();
    let password = password.trim().to_owned();

    let (register_response_result, refresh_token, expiry)  = get_user(username.clone(), password).await;

    return (username, register_response_result, refresh_token, expiry);

}


pub async fn get_user(username: String, password: String) -> (RegisterResponseResult, String, i64) {


    let register_status = register_into_db(username, password).await;

    match register_status {
                Ok(status) => {
                    match status.status_type {
                        StatusTypes::REGISTRATION_SUCCESSFUL => {
                            let token = status.access_token;
                            let refresh_token = status.refresh_token;
                            let expiry = status.exp;
                            
                            return (RegisterResponseResult::TOKEN(token), refresh_token, expiry) ;

                        },
                        StatusTypes::USER_ALREADY_EXISTS => {
                            
                            return (RegisterResponseResult::EXISTING_USER, "".to_string(), 0);
                        },
                        StatusTypes::DATABASE_ERROR => {
                            
                            return (RegisterResponseResult::DATABASE_ERROR, "".to_string(), 0);
                        },
                        _ => {
                            
                            return (RegisterResponseResult::UNKNOWN_ERROR, "".to_string(), 0);
                        }
                    }
                },
                Err(err) => {
                    return (RegisterResponseResult::REQUEST_ERROR, "".to_string(), 0);
                }
    }
}


pub async fn register_into_db(user_name:String, pass:String) -> Result< RegisterResponse, reqwest::Error > {

    let endpoints = Endpoints::new();
    
    let url = endpoints.register.to_owned();

    let pub_id_key = generate_identity_keypair();

    let user = UserRegisterPayload{
        username: user_name,
        password: pass,
        public_identity_key: pub_id_key
    };

    let client = Client::new();
    let res = client
        .post(url)
        .json(&user)
        .send()
        .await?;   

    let data = res.json::<RegisterResponse>().await?;
    
    Ok(data)
}