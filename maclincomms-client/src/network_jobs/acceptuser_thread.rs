use base64::{engine::general_purpose, Engine};
use disk_persist::DiskPersist;
use ratatui::{style::{ Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}};

use crate::{crypto::dm_keys::{generate_dh_keypair, generate_sender_chainkey, generate_shared_rootkey}, event_model::Event, tui_main::MaclincommsApp, user_model::{AcceptanceStatus, DmDoubleRatchet_Keys, DmUser_Data}};

use super::{accept_user::{accept_user, AcceptUserResponseResult}, getdms_thread::start_getdms_thread};




pub async fn start_acceptuser_thread(app: &mut MaclincommsApp, user_to_accept: String) {

    let accept_user_token = app.access_token.clone();

    let endpoint = app.endpoints.accept_user;

    let accept_user_result = accept_user(accept_user_token, user_to_accept.clone(), AcceptanceStatus::ACCEPTED, endpoint).await;
    


    match accept_user_result {

        AcceptUserResponseResult::REQUEST_ERROR => {

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.notifications_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        },

        AcceptUserResponseResult::DATABASE_ERROR => {

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.notifications_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        },

        AcceptUserResponseResult::UNKNOWN_ERROR => {

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.notifications_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        }

        AcceptUserResponseResult::NOTIFICATIONS_ERROR => {

            let text = format!("Added {} but couldnt notify them", user_to_accept);
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightYellow));
                
            app.notifications_comps.action_status_block = Paragraph::new(text.light_yellow())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        }

        AcceptUserResponseResult::USER_ADDED => {

            let text = format!("{} added to your DMs list", user_to_accept);
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.notifications_comps.action_status_block = Paragraph::new(text.light_green())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            //FETCH your dm list again
            start_getdms_thread(app).await;

            //Retrieve user public key
            let dms_list: DiskPersist<Vec<DmUser_Data>> = DiskPersist::init("persistent-user-dms-list").unwrap();

            if let Ok(dmslist_res) = dms_list.read(){
                match dmslist_res{
                    Some(data) => {
                        let their_pub_key = data.into_iter()
                                .find(|dm| dm.username==user_to_accept)
                                .map(|info| info.public_identity_key)
                                .unwrap();
                        let their_pub_key_bytes: [u8;32] = general_purpose::STANDARD.decode(their_pub_key).unwrap().try_into().unwrap();
                        //Generate dh pair
                        let (pub_dh_key, priv_dh_key) = generate_dh_keypair();
                        //Generate root key
                        let rootkey = generate_shared_rootkey(their_pub_key_bytes, priv_dh_key);
                        //Generate chain key
                        let sending_chainkey = generate_sender_chainkey(rootkey);
                        //Initialise first keys with user and store in dme2ee data
                        let keys_data = DmDoubleRatchet_Keys{
                            root_key: rootkey,
                            their_old_dh_pub_key: [0u8;32],
                            dh_pub_key: pub_dh_key,
                            dh_priv_key: priv_dh_key,
                            sending_chain_key: sending_chainkey,
                            receiving_chain_key: [0u8;32],
                        };

                        app.dme2ee_data.dms.insert(user_to_accept, keys_data);
                    }
                    None => {}
                }
                
            }

        }
        _ => {}
    }
}