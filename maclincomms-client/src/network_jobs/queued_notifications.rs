use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};
use disk_persist::DiskPersist;
use reqwest::{Client};
use serde::{Deserialize, Serialize};

use crate::{crypto::{decrypt_msg::decrypt_dm_message, dm_keys::{derive_message_key, generate_dh_keypair, generate_receiver_chainkey, generate_sender_chainkey, generate_shared_rootkey}}, get_current_date, get_current_time, tui_main::MaclincommsApp, tui_widgets::notifications_panel::NotificationStatus, user_model::{DmDoubleRatchet_Keys, DmMessage, DmUser_Data, NotificationData, NotificationType, StatusTypes, UserIdentityKeys}};

use super::getdms_thread::start_getdms_thread;


#[derive(Deserialize, Debug, Clone)]
pub struct TempNotif {
    pub n_type: NotificationType,
    pub from: String,
    pub to: String,
    pub content: String,
}


#[derive(Serialize, Deserialize)]
pub struct QueuedNotificationsReponseData{
    pub status_type: StatusTypes,
    pub data: HashMap<String, Vec<String>>,
    pub message: String
}

pub async fn get_queued_notifications(app: &mut MaclincommsApp) {
    
    let token = app.access_token.clone();

    let queued_notifications_endpoint = app.endpoints.queued_notifications;

   
    let url = queued_notifications_endpoint.to_string();
    let client = Client::new();

    let response = client
        .get(url)
        .header("Authorization", token)
        .send()
        .await;

    match response{
        Ok(data) => {
            let res_data = data.json::<QueuedNotificationsReponseData>().await.unwrap();

            match res_data.status_type{
                StatusTypes::NOTIFICATIONS_FETCHED_SUCCESSFULLY => {

                    let n_map = res_data.data;

                    //let mut notifications_list = app.notifications_comps.notifications_history.lock().unwrap();
                    
                    for (_key, json_vec) in n_map {
                        for json_str in json_vec {
                            // Convert each string to TempNotif
                            if let Ok(temp) = serde_json::from_str::<TempNotif>(&json_str) {
                                let time = get_current_time() + " on " + get_current_date().as_str();
                    
                                let notification = NotificationData {
                                    n_type: temp.n_type,
                                    from: temp.from,
                                    to: temp.to,
                                    content: temp.content,
                                    time,
                                };
                    
                                let notification_cloned = notification.clone(); //for second check for accepted notification
                                if let Ok(mut n_history_lock) = app.notifications_comps.notifications_history.lock() {
                                    //For Message Notification, parsing its content
                                    if matches!(notification.n_type, NotificationType::MESSAGE){
                                        if let Some(keys) = app.dme2ee_data.dms.get_mut(&notification.from){
                                                let mut decrypted_message = "".to_string();
                                                //Parsing Message Contents
                                                let msg_data_res = serde_json::from_str::<DmMessage>(&notification.content);
                                                if let Ok(msg_data) = msg_data_res{
                                                    //Check if the public key sent with the message is the same as old or not
                                                    // ------> Extracting dh_pub key from [encrpted_msg_content].[dhpub_key]
                                                    let msg_parts: Vec<&str> = msg_data.content.split('.').collect();
                                                    let ciphertext = msg_parts[0];
                                                    let their_dh_pub = msg_parts[1];
                                                    let their_dh_pub_bytes: [u8;32] = general_purpose::STANDARD.decode(their_dh_pub).unwrap().try_into().unwrap();
                                                    //match new and old
                                                    if keys.their_old_dh_pub_key==their_dh_pub_bytes{
                                                        //Get Recv Chainkey
                                                        let recv_chain_key = keys.receiving_chain_key;
                                                        let recv_msg_key = derive_message_key(recv_chain_key);
                                                        //Decrypt Message
                                                        decrypted_message = decrypt_dm_message(recv_msg_key, ciphertext);
                                                    }
                                                    else{
                                                        //Check if receiving first message
                                                        if keys.their_old_dh_pub_key==[0u8;32]{
                                                            keys.their_old_dh_pub_key = their_dh_pub_bytes;
                                                            //Load private key from disk
                                                            let id_keys: DiskPersist<UserIdentityKeys> = DiskPersist::init("persistent-user-identity-keypair").unwrap();
                                                            if let Ok(data_res) = id_keys.read(){
                                                                match data_res{
                                                                    Some(data) => {
                                                                        let priv_key = data.private_identity_key;
                                                                        let priv_key_bytes: [u8;32] = general_purpose::STANDARD.decode(priv_key).unwrap().try_into().unwrap();
                                                                        let rootkey = generate_shared_rootkey(their_dh_pub_bytes, priv_key_bytes);
                                                                        let receiving_chainkey = generate_receiver_chainkey(rootkey);
                                                                        keys.receiving_chain_key = receiving_chainkey;
                                                                        let recv_mkey = derive_message_key(receiving_chainkey);
                                                                        //Decrypt Message
                                                                        decrypted_message = decrypt_dm_message(recv_mkey, ciphertext);
                                                                        //Generate new dh pair
                                                                        let (public_dh_key, private_dh_key) = generate_dh_keypair();
                                                                        let new_rootkey = generate_shared_rootkey(their_dh_pub_bytes, private_dh_key);
                                                                        let sending_chainkey = generate_sender_chainkey(new_rootkey);
                                                                        //Store new ratcheted keys
                                                                        keys.root_key = new_rootkey;
                                                                        keys.dh_pub_key = public_dh_key;
                                                                        keys.dh_priv_key = private_dh_key;
                                                                        keys.sending_chain_key = sending_chainkey;
                                                                        keys.receiving_chain_key = receiving_chainkey;
                                                                    }
                                                                    None => {}
                                                                }
                                                            }

                                                        }
                                                        else{
                                                            keys.their_old_dh_pub_key = their_dh_pub_bytes;
                                                            let my_dh_priv = keys.dh_priv_key;
                                                            let rootkey = generate_shared_rootkey(their_dh_pub_bytes, my_dh_priv);
                                                            let receiving_chainkey = generate_receiver_chainkey(rootkey);
                                                            keys.receiving_chain_key = receiving_chainkey;
                                                            let recv_mkey = derive_message_key(receiving_chainkey);
                                                            //Decrypt Message
                                                            decrypted_message = decrypt_dm_message(recv_mkey, ciphertext);
                                                            //Generate new dh pair
                                                            let (public_dh_key, private_dh_key) = generate_dh_keypair();
                                                            let new_rootkey = generate_shared_rootkey(their_dh_pub_bytes, private_dh_key);
                                                            let sending_chainkey = generate_sender_chainkey(new_rootkey);
                                                            //Store new ratcheted keys
                                                            keys.root_key = new_rootkey;
                                                            keys.dh_pub_key = public_dh_key;
                                                            keys.dh_priv_key = private_dh_key;
                                                            keys.sending_chain_key = sending_chainkey;
                                                            keys.receiving_chain_key = receiving_chainkey;
                                                        }
                                                    }

                                                    //Pushing to chats in data level
                                                    if let Some(dm_chats) = app.dmchats_warehouse.dms_data.get_mut(&notification.from){
                                                        dm_chats.push((
                                                            notification.from.clone(), 
                                                            decrypted_message.clone(), 
                                                            notification.time.clone().split(" on ").next().unwrap().to_string(), 
                                                            "".to_string(), 
                                                            msg_data.is_online_offline_msg,
                                                            "".to_string()
                                                        ));
                                                    }
                                                    //Pushing to Notification Data
                                                    n_history_lock.push(
                                                        NotificationData{
                                                            n_type: notification.n_type,
                                                            from: notification.from,
                                                            to: notification.to,
                                                            content: decrypted_message,
                                                            time: notification.time
                                                        }
                                                    );
                                                }
                                        }
                                    }
                                    else{
                                        n_history_lock.push(
                                            NotificationData{
                                                n_type: notification.n_type,
                                                from: notification.from,
                                                to: notification.to,
                                                content: notification.content,
                                                time: notification.time
                                            }
                                        );
                                    }

                                    app.new_notis_count += 1;
                                    app.notifications_comps.status = NotificationStatus::FETCHED;
                                    
                                }
                                // For Accepted Notification
                                // Add user's keys, calculate and store shared secret, sending and receiving chain keys
                                match notification_cloned.n_type{
                                    NotificationType::ACCEPTED => {
                                        //Retrieve latest dms list
                                        start_getdms_thread(app).await;
                                        //Read the latest written data on disk
                                        let persistent_dms_list: DiskPersist<Vec<DmUser_Data>> = DiskPersist::init("persistent-user-dms-list").unwrap();
                                        if let Ok(data_res) = persistent_dms_list.read(){
                                            match data_res{
                                                Some(dms_list) => {
                                                    //Get the public id key of added user
                                                    let mut their_pub_key = "".to_string();
                                                    let their_pub_key_res = dms_list.iter()
                                                        .find(|dm| dm.username==notification_cloned.from)
                                                        .map(|dm| dm.public_identity_key.clone());
                                                    if let Some(k) = their_pub_key_res{
                                                        their_pub_key = k;
                                                    }
                                                    let their_pub_key_bytes: [u8;32] = general_purpose::STANDARD.decode(their_pub_key).unwrap().try_into().unwrap();
                                                    //Generate first DH pair
                                                    let (public_dh_key, private_dh_key) = generate_dh_keypair();
                                                    //Compute root key
                                                    let rootkey = generate_shared_rootkey(their_pub_key_bytes, private_dh_key);
                                                    //Derive sending chain key
                                                    let sending_chainkey = generate_sender_chainkey(rootkey);
                                                    //Initialise first keys with user and store in dme2ee data
                                                    let keys_data = DmDoubleRatchet_Keys{
                                                        root_key: rootkey,
                                                        their_old_dh_pub_key: [0u8;32],
                                                        dh_pub_key: public_dh_key,
                                                        dh_priv_key: private_dh_key,
                                                        sending_chain_key: sending_chainkey,
                                                        receiving_chain_key: [0u8;32],
                                                    };

                                                    app.dme2ee_data.dms.insert(notification_cloned.from, keys_data);     
                                                }
                                                None => {
                                                    //Handle Empty Dms List
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    let notifications_list = app.notifications_comps.notifications_history.lock().unwrap();

                    app.new_notis_count = notifications_list.len() as i8;

                    match notifications_list.len() as i8{
                        0 => {
                            app.notifications_comps.status = NotificationStatus::NOTHING;
                        }
                        _ => {
                            app.notifications_comps.status = NotificationStatus::FETCHED;
                        }
                    }
                }
                StatusTypes::NOTIFICATIONS_ERROR => {
                    app.new_notis_count = 0;
                    app.notifications_comps.status = NotificationStatus::ERROR;
                }
                _ => {}
            }
        }
        Err(err) => {
            app.new_notis_count = 0;
            app.notifications_comps.status = NotificationStatus::ERROR;
        }
    }

}