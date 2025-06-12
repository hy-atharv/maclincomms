use std::{collections::HashMap, thread};

use disk_persist::DiskPersist;
use ratatui::{style::{ Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}};

use crate::{crypto::decrypt_msg::decrypt_dm_chats_session, tui_main::MaclincommsApp, user_model::{DmChats_Warehouse, DmSessionEncryption_Key}};

use super::get_dm_chats::{get_dm_chats, GetDmChatsResponseResult};






pub async fn start_getdmchats_thread(app: &mut MaclincommsApp) {

    let get_dm_chats_token = app.access_token.clone();

    let endpoint = app.endpoints.get_dm_chats_data;

    let get_dm_chats_result = get_dm_chats(get_dm_chats_token, endpoint).await;
    

    match get_dm_chats_result {

        GetDmChatsResponseResult::REQUEST_ERROR => {

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        },

        GetDmChatsResponseResult::DATABASE_ERROR => {

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        },

        GetDmChatsResponseResult::UNKNOWN_ERROR => {

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        }

        GetDmChatsResponseResult::DMS_DATA_FETCHED(data) => {

            //Storing in Data Level Warehouse
            let mut dms_data_map: HashMap<String, Vec<(String, String, String, String, bool, String)>> = HashMap::new();

            for user_dm in data.0{
                if let Some((username, chats)) = user_dm.0.into_iter().next() {
                    let dm_messages: Vec<(String, String, String, String, bool, String)> = chats
                        .into_iter()
                        .map(|msg_tuple| 
                            (
                            msg_tuple.0, 
                            msg_tuple.1, 
                            msg_tuple.2, 
                            msg_tuple.3, 
                            msg_tuple.4, 
                            msg_tuple.5
                            )
                        )
                        .collect();

                    dms_data_map.insert(username, dm_messages);
                }
            }

            let dm_session_keys: DiskPersist<HashMap<String, DmSessionEncryption_Key>> = DiskPersist::init("persistent-dms-session-keys").unwrap();
            if let Err(e) = dm_session_keys.read() {
                //error handling
            }
            else {
                let dm_session_keys_data = dm_session_keys.read().unwrap();
                match dm_session_keys_data {
                    Some(session_key_data) => {
                        let _session_key_data = session_key_data.clone();
                        //Decrypting messages in a different thread
                        let decrypt_msgs_task_handle = thread::spawn(move||{
                            decrypt_dm_chats_session(_session_key_data.clone(), dms_data_map)
                        });

                        match decrypt_msgs_task_handle.join(){
                            Ok(dms_data_decrypted) => {
                                //Storing into App State
                                app.dmchats_warehouse = DmChats_Warehouse{
                                    dms_session_key: session_key_data,
                                    dms_data: dms_data_decrypted
                                }
                            }
                            Err(err) => {
                                //thread joinhandle error
                            }
                        }   
                    }
                    None => {
                        //reading persistent session keys from disk error
                    }
                }
            }
        }
    }
}