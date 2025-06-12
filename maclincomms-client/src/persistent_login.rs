use std::collections::HashMap;

use disk_persist::DiskPersist;
use reqwest::{Error, StatusCode};

use crate::{network_jobs::{authN_user::authenticate_user, getdmchats_thread::start_getdmchats_thread, getdms_thread::start_getdms_thread, queued_notifications::get_queued_notifications, request_token::request_new_token}, screens_model::Screens, tui_main::MaclincommsApp, user_model::{DmDoubleRatchet_Keys, DmE2EEncryption_Data, DmSessionEncryption_Key, RequestNewTokenResponse, StatusTypes, UserData}};


//This function also retrieves queued notifications and dms data
pub async fn persistent_authentication(app: &mut MaclincommsApp){

    //PERSISTENT USER DATA
    let persistent_storage: DiskPersist<UserData> = DiskPersist::init("persistent-user-data").unwrap();

    let authN_url = app.endpoints.authN;
    let newtoken_url = app.endpoints.new_token;

    //READING PERSISTENT STORAGE
    if let Err(e) = persistent_storage.read() {
        app.current_screen = Screens::WELCOME_SCREEN;
    }
    //IF NO ERROR IN READING PERSISTENT STORAGE
    else {

        let user_data = persistent_storage.read().unwrap();

        match user_data {
            Some(data) => { //SOME DATA EXISTS

                app.username = data.username;

                let token = data.access_token.clone();
                let refresh_token = data.refresh_token;
                
            
            
                let auth_status = authenticate_user(token, authN_url).await;
            
            
                // HANDLING AUTHENTICATION STATUS AND LOGGIN IN TO APP
                match auth_status {
                    Ok(resp) => {
                        match resp.status() {
                            StatusCode::OK => {

                                app.access_token = data.access_token.clone();

                                //retrieving chats backup
                                start_getdmchats_thread(app).await;

                                //retrieving queued notifications
                                get_queued_notifications(app).await;

                                //retrieving dms list and keys
                                start_getdms_thread(app).await;

                                
                                
                                app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
                                app.chatoptions_menu.activate();
                            },
                            StatusCode::UNAUTHORIZED => { // TOKEN EXPIRED or INVALID, ATTEMPTING TO OBTAIN NEW WITH REFRESH TOKEN
            
                              
                                let request_token_data: Result<RequestNewTokenResponse, Error> = request_new_token(refresh_token, newtoken_url).await;
                    
            
                                match request_token_data {
                                    Ok(res) => {
                                        match res.status_type {
                                            StatusTypes::TOKEN_GRANTED => {
                                                //STORING NEW REQUESTED TOKENS AND EXPIRY
                                                app.access_token = res.access_token.clone();
                                                app.refresh_token = res.refresh_token.clone();
                                                app.token_expiry = res.exp;
            
                                                //WRITING TO PERSISTENT STORAGE
                                                let data = UserData {
                                                    username: app.username.clone(),
                                                    access_token: res.access_token,
                                                    refresh_token: res.refresh_token,
                                                    token_expiry: res.exp
                                                };
            
                                                persistent_storage.write(&data).unwrap();

                                                //retrieving chats backup
                                                start_getdmchats_thread(app).await;

                                                //retrieving queued notifications
                                                get_queued_notifications(app).await;

                                                //retrieving dms list and public keys
                                                start_getdms_thread(app).await;

            
            
                                                //ALLOWING USER IN THE APP
                                                app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
                                                app.chatoptions_menu.activate();
            
                                            },
                                            _ => {
                                                app.login_textarea.username_ta.insert_str(app.username.clone());
                                                app.current_screen = Screens::LOGIN_SCREEN;
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        app.current_screen = Screens::LOGIN_SCREEN;
                                    }
                                }
                            },
                            _ => {
                                app.current_screen = Screens::LOGIN_SCREEN;
                            }
                        }
                    },
                    Err(e) => {
                        app.current_screen = Screens::LOGIN_SCREEN;
                    }
                }
                //END OF MATCH
            },
            None => { //NO DATA IN PERSISTENT STORAGE
                app.current_screen = Screens::WELCOME_SCREEN;
            }
        }

    }
}



