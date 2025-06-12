use std::{collections::HashMap, io::{self}, process::exit, sync::mpsc::{self, Sender}, thread::{self}};
use base64::{engine::general_purpose, Engine};
use crossterm::{
    event::{KeyCode, KeyEventKind, KeyModifiers}
};
use disk_persist::DiskPersist;
use ratatui::{
     layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, Borders, Paragraph}, DefaultTerminal, Frame
};
use tui_big_text::BigText;
use tui_menu::{Menu, MenuEvent, MenuItem, MenuState}; 


use crate::{
    crypto::{
        decrypt_msg::{decrypt_dm_message, decrypt_room_message, decrypt_senderkey_message}, 
        dm_keys::{derive_message_key, generate_dh_keypair, generate_receiver_chainkey, generate_sender_chainkey, generate_shared_rootkey, update_receiving_chainkey, update_sending_chainkey}, 
        encrypt_msg::{encrypt_dm_chats_session, encrypt_senderkey_message, verify_room_ciphertext}, 
        room_keys::{compose_sender_key, derive_roommessage_key, generate_roomchain_key, update_my_roomchainkey, update_their_roomchainkey}
    }, 
    endpoints::Endpoints, event_model::Event, 
    get_current_time, 
    network_jobs::{
        acceptuser_thread::start_acceptuser_thread, 
        adduser_thread::start_adduser_task, 
        blockuser_thread::start_blockuser_task, 
        get_roomdata::{get_room_data}, 
        getdms_thread::start_getdms_thread, 
        joinroom_thread::start_joinroom_task, 
        login_thread::start_login_task, 
        realtime_notifications::subscribe_to_realtime_notifications, 
        register_thread::start_register_task, 
        roomcreation_thread::start_roomcreation_task, 
        unblockuser_thread::start_unblockuser_task, 
        upload_dm_chats::upload_dm_chats
    }, 
    persistent_login::persistent_authentication, 
    screen_inputs::{handle_add_user_screen_inputs, handle_block_user_screen_inputs, handle_chat_options_screen_inputs, handle_dm_chat_screen_inputs, handle_dm_user_screen_inputs, handle_login_screen_inputs, handle_notifications_screen_inputs, handle_public_chat_screen_inputs, handle_register_screen_inputs, handle_room_chat_screen_inputs, handle_room_creation_screen_inputs, handle_room_join_screen_inputs, handle_welcome_screen_inputs, take_next_lines, text_to_string}, 
    screens_model::Screens, 
    tui_widgets::{
        adduser_panel, 
        adduser_textarea::AddUserTextArea, 
        blockuser_panel, 
        blockuser_textarea::BlockUnblockUserTextArea, 
        chatoptions_panel::{self, ChatOptionsAction}, 
        dmchat_panel::{self, DmChatComponents}, 
        dmuser_panel::{self, DmUserComponents}, 
        joinroom_panel, 
        joinroom_textarea::{JoinRoomTaskStatus, JoinRoomTextArea}, 
        login_menu::{self, LoginMenuAction}, 
        login_screen, 
        login_textarea::LoginTextArea, 
        notifications_panel::{self, NotificationStatus, NotificationsComponents}, 
        publicchat_panel::{self, PublicChatComponents}, 
        register_screen, 
        register_textarea::RegisterTextArea, 
        roomchat_panel::{self, RoomChatComponents}, 
        roomcreate_panel, 
        roomcreation_textarea::RoomCreationTextArea, 
        splash_screen::draw_splash_screen
    }, 
    user_model::{AckType, ChatData, ChatEntry, DisconnectType, DmChats_Warehouse, DmDoubleRatchet_Keys, DmE2EEncryption_Data, DmMessage, DmUser_Data, DmsListData, Message, MessageType, NotificationData, NotificationType, RoomMessageType, RoomSenderMessage, Room_Keys, SenderKey, SocketMessage, UserIdentityKeys, UserSignatureKeys, WhisperMode, WorldChatMessage}, 
    websockets::websocket_thread::{start_dmchat_websocket_task, start_roomchat_websocket_task, start_worldchat_websocket_task}
};



pub async fn start_tui() -> io::Result<()> {
    // Set up the terminal
    let mut terminal = ratatui::init();


    // Drawing Splash Screen
    terminal.draw(|frame | draw_splash_screen(frame))?;


    //MAIN CHANNEL
    let (main_tx, main_rx) = mpsc::channel::<Event>();



    //Another cloned producer for sending input events to main channel
    let inputs_tx = main_tx.clone();
    //Another cloned producer for sending network events to main channel
    let network_tx = main_tx.clone();



    // Initialising App
    let mut app = MaclincommsApp::new(network_tx);



    //Spawning thread for accepting Input Events
    thread::spawn(move ||{
        handle_input_events(inputs_tx);
    });

    
    //Trying persistent authentication
    persistent_authentication(&mut app).await; //(persistent auth if succeeds also retrieves queued notifications)

    let token = app.access_token.clone();
    let notification_tx = app.network_event_tx.clone();
    let realtime_notis_endpoint = app.endpoints.realtime_notifications;

    //Spawning task to listen for Notifications Server side events from Redis Pub Sub Channel
    tokio::spawn(async move{
        subscribe_to_realtime_notifications(
            token,
            notification_tx,
            realtime_notis_endpoint
        ).await;
    });

    let app_result = app.run(
        &mut terminal, 
        main_rx, 
        main_tx
    ).await;


    ratatui::restore();
    app_result
}



// FUNCTION TO HANDLE INPUT EVENTS
fn handle_input_events(inputs_tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => inputs_tx.send(Event::InputEvent(key_event)).unwrap(),
            _ => {}
        }
    }
}



pub struct MaclincommsApp {
    pub exit: bool,
    pub exiting_status: &'static str,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_expiry: i64, //Unix Timestamp
    pub endpoints: Endpoints,
    pub outgoing_worldchat_msg_tx: Option<mpsc::Sender<SocketMessage>>,
    pub outgoing_roomchat_msg_tx: Option<mpsc::Sender<SocketMessage>>,
    pub outgoing_dmchat_msg_tx: Option<mpsc::Sender<SocketMessage>>,
    pub network_event_tx: mpsc::Sender<Event>,
    pub login_menu: MenuState<LoginMenuAction>,
    pub current_screen: Screens,
    pub register_textarea: RegisterTextArea,
    pub login_textarea: LoginTextArea,
    pub chatoptions_menu: MenuState<ChatOptionsAction>,
    pub publicchat_comps: PublicChatComponents,
    pub is_pubchat_joined: bool,
    pub roomcreation_textarea: RoomCreationTextArea,
    pub roomchat_comps: RoomChatComponents,
    pub is_roomchat_joined: bool,
    pub joinroom_textarea: JoinRoomTextArea,
    pub adduser_textarea: AddUserTextArea,
    pub dmuser_comps: DmUserComponents,
    pub dmchat_comps: DmChatComponents,
    pub is_dmchat_joined: bool,
    pub blockunblock_textarea: BlockUnblockUserTextArea,
    pub notifications_comps: NotificationsComponents,
    pub new_notis_count: i8,
    pub dmchats_warehouse: DmChats_Warehouse,
    pub dme2ee_data: DmE2EEncryption_Data,
    pub signature_keys: UserSignatureKeys,
    pub room_keys: Room_Keys,
    pub room_token: String,
    pub is_current_room_owner: bool
}




impl MaclincommsApp {

    // A constructor that initializes the APP STATE
    pub fn new(network_tx: Sender<Event>) -> Self {
        Self {
            exit: false,
            exiting_status: "maclincomms v2.0",
            username: "".to_string(),
            access_token: "".to_string(),
            refresh_token: "".to_string(),
            token_expiry: 0,
            endpoints: Endpoints::new(),
            outgoing_worldchat_msg_tx: None,
            outgoing_roomchat_msg_tx: None,
            outgoing_dmchat_msg_tx: None,
            network_event_tx: network_tx,
            login_menu: MenuState::new(vec![
                MenuItem::group(
                "Let's maclincomm",    
                vec![
                MenuItem::item("Register", LoginMenuAction::REGISTER),
                MenuItem::item("Login", LoginMenuAction::LOGIN),
                ]
                )
            ]),
            current_screen: Screens::WELCOME_SCREEN,
            register_textarea: RegisterTextArea::new(),
            login_textarea: LoginTextArea::new(),
            chatoptions_menu: MenuState::new(vec![
                MenuItem::group(
                "Chat Options",
                vec![
                MenuItem::item("World Chat", ChatOptionsAction::PUBLIC_CHAT),
                MenuItem::item("Current Room", ChatOptionsAction::CURRENT_ROOM),
                MenuItem::item("Current DM", ChatOptionsAction::CURRENT_DM),
                MenuItem::item("Create Room", ChatOptionsAction::CREATE_ROOM),
                MenuItem::item("Join Room", ChatOptionsAction::JOIN_ROOM),
                MenuItem::item("Add User", ChatOptionsAction::ADD_USER),
                MenuItem::item("DM User", ChatOptionsAction::DM_USER),
                MenuItem::item("Block/Unblock User", ChatOptionsAction::BLOCK_USER),
                MenuItem::item("Notifications", ChatOptionsAction::NOTIFICATIONS),
                ]
                )
            ]),
            publicchat_comps: PublicChatComponents::new(),
            is_pubchat_joined: false,
            roomcreation_textarea: RoomCreationTextArea::new(),
            roomchat_comps: RoomChatComponents::new(),
            is_roomchat_joined: false,
            joinroom_textarea: JoinRoomTextArea::new(),
            adduser_textarea: AddUserTextArea::new(),
            dmuser_comps: DmUserComponents::new(),
            dmchat_comps: DmChatComponents::new(),
            is_dmchat_joined: false,
            blockunblock_textarea: BlockUnblockUserTextArea::new(),
            notifications_comps: NotificationsComponents::new(),
            new_notis_count: 0,
            dmchats_warehouse: DmChats_Warehouse::new(),
            dme2ee_data: DmE2EEncryption_Data::load(),
            signature_keys: UserSignatureKeys::new(),
            room_keys: Room_Keys::new(),
            room_token: "".to_string(),
            is_current_room_owner: false
        } 
    }

    async fn run(&mut self, terminal: &mut DefaultTerminal, main_events_channel_rx: mpsc::Receiver<Event>, main_events_channel_tx: mpsc::Sender<Event>) -> io::Result<()>{
        while !self.exit {
            terminal.draw(|frame | self.draw(frame))?;

            match main_events_channel_rx.recv().unwrap() {
                Event::IncomingPublicMessageEvent(msg) => {
                    let formatted_msg = take_next_lines(msg.content.clone());
                    //Update ui only if the message is not empty
                    if !msg.content.is_empty(){
                        if let Ok(mut chat_history_lock) = self.publicchat_comps.chat_history.lock() {
                            chat_history_lock.push((
                                msg.username,
                                Text::from(formatted_msg),
                                get_current_time(),
                                msg.is_join_leave_msg,
                                "".to_string()
                            ));
                        }
                        self.publicchat_comps.scroll_state.scroll_to_bottom();
                    }
                },

                Event::IncomingPublicMessageAckEvent(ack_type) => {
                    if matches!(ack_type, AckType::ServerAck){
                        if let Ok(mut chat_history_lock) = self.publicchat_comps.chat_history.lock() {
                            for message in chat_history_lock.iter_mut().rev(){
                                //my sent message
                                if message.0==self.username{
                                    //Update server ack tick
                                    message.4 = ">".to_string();
                                    break;
                                }
                            }
                        }
                    }
                }

                Event::IncomingRoomMessageEvent(msg) => {
                    //Normal chat message
                    if msg.is_join_leave_msg==false{
                        if let Ok(mut chat_history_lock) = self.roomchat_comps.chat_history.lock() {
                            //Extract ciphertext and signature
                            let msg_parts: Vec<&str> = msg.content.split(".").collect();
                            let ciphertext = msg_parts[0];
                            let signature_b64 = msg_parts[1];

                            if let Some(their_sender_key) = self.room_keys.their_sender_keys.get_mut(&msg.username){
                                let their_signature_pub_key = their_sender_key.pub_sig_key;
                                let their_chain_key = their_sender_key.chain_key;
                                //Verify Signature
                                if let Ok(()) = verify_room_ciphertext(their_signature_pub_key, ciphertext, signature_b64){
                                    //Decrypt message
                                    let receiving_msg_key = derive_roommessage_key(their_chain_key);
                                    let decrypted_msg = decrypt_room_message(receiving_msg_key, ciphertext);
                                    //Update Chain Key
                                    update_their_roomchainkey(their_sender_key);
                                    //Only update ui if message is not empty and decrypted successfully
                                    if !decrypted_msg.is_empty(){
                                        let formatted_msg = take_next_lines(decrypted_msg);
                                        chat_history_lock.push((
                                            msg.username,
                                            Text::from(formatted_msg),
                                            get_current_time(),
                                            msg.is_join_leave_msg,
                                            "".to_string()
                                        ));
                                    }
                                }
                            } 
                        }
                    }
                    //Join Message (User joined)
                    //Retrieve new member's pub key, send your sender key and get his sender key
                    else if msg.is_join_leave_msg==true && msg.content.split_whitespace().nth(1) == Some("joined"){
                        //Making Sender Key [ChainKey][Public Signature Key]
                        let pub_sig_key = self.signature_keys.public_signature_key.clone();
                        let sender_key = compose_sender_key(self.room_keys.chain_key.clone(), pub_sig_key);
                        //Retrieve member keys again for new member
                        let get_room_data_endpoint = self.endpoints.get_room_data;
                        let token = self.room_token.clone();
                        let room_data_res = get_room_data(token, get_room_data_endpoint).await;
                        if let Some(room_data) = room_data_res{
                            let new_user = msg.username.clone();
                            let mut pub_key_bytes = [0u8;32];
                            let their_pub_key_res = room_data.members_keys
                                    .iter()
                                    .find_map(|val| {
                                        val.as_object()?.get(&new_user)
                                    });
                            if let Some(k) = their_pub_key_res{
                                let pub_key = k.as_str().unwrap();
                                pub_key_bytes = general_purpose::STANDARD.decode(pub_key).unwrap().try_into().unwrap();
                                //Storing user's pub id key
                                self.room_keys.their_idpublic_keys.insert(new_user.clone(), pub_key_bytes);
                            }
                            //Deriving bi-directional chain keys for secure transfer
                            //Generate first DH pair
                            let (public_dh_key, private_dh_key) = generate_dh_keypair();
                            //Storing my pub_dh_key for each user
                            self.room_keys.my_dh_pub_keys.insert(new_user.clone(), public_dh_key.to_vec());

                            //Compute root key
                            let rootkey = generate_shared_rootkey(pub_key_bytes, private_dh_key);
                            //Derive sending chain key
                            let sending_chainkey = generate_sender_chainkey(rootkey);
                            
                            //Encrypting Sender Key
                            let encrypted_skey_for_user = encrypt_senderkey_message(sending_chainkey, &sender_key);

                            //Storing each encrypted version of sender key, for different users to share with
                            self.room_keys.my_sender_key_encryptions.insert(new_user.clone(), encrypted_skey_for_user.clone());

                            //Sending new joiner my sender key
                            let skey_tx = self.outgoing_roomchat_msg_tx.clone().unwrap();
                            let skey_descriptor_byte = [0x11].to_vec();
                            let username_bytes = new_user.as_bytes().to_vec();
                            let skey_binary_mesg = [skey_descriptor_byte, encrypted_skey_for_user, public_dh_key.to_vec(), username_bytes].concat();
                            //Sending ENCRYPTED SENDER KEY in bytes
                            if let Err(e) = skey_tx.send(SocketMessage::RoomSenderKey(skey_binary_mesg))
                            {
                                println!("Couldnt send sender key message");
                            }
                        }
                        //Pushing join/leave message to chat history
                        if let Ok(mut chat_history_lock) = self.roomchat_comps.chat_history.lock() {
                            let formatted_msg = take_next_lines(msg.content);
                            chat_history_lock.push((
                                msg.username,
                                Text::from(formatted_msg),
                                get_current_time(),
                                msg.is_join_leave_msg,
                                "".to_string()
                            )); 
                        }
                    }
                    //Leave Message (User left)
                    //Retrieve dms list again, resend your new sender keys and get their new sender keys
                    else if msg.is_join_leave_msg==true && msg.content.split_whitespace().nth(1) == Some("left"){
                        //Clear old room data
                        self.room_keys = Room_Keys::new();
                        //Retrieving Room Data and Memeber Keys for BI-directional encrypted transfer of Sender Key
                        let get_room_data_endpoint = self.endpoints.get_room_data;
                        let room_data_res = get_room_data(self.room_token.clone(), get_room_data_endpoint).await;

                        //For room
                        //Generating My Chain Key
                        let chain_key = generate_roomchain_key();
                        self.room_keys.chain_key = chain_key;
                        //Making Sender Key [ChainKey][Public Signature Key]
                        let pub_sig_key = self.signature_keys.public_signature_key.clone();
                        let sender_key = compose_sender_key(chain_key, pub_sig_key);
                        
                        //Sending sender key to each user by encrypting with their shared root key derived message key
                        if let Some(data) = room_data_res{
                            //For all users in room
                            for user in data.room_members{
                                let mut pub_key_bytes = [0u8;32];
                                let their_pub_key_res = data.members_keys
                                        .iter()
                                        .find_map(|val| {
                                            val.as_object()?.get(&user)
                                        });
                                if let Some(k) = their_pub_key_res{
                                    let pub_key = k.as_str().unwrap();
                                    pub_key_bytes = general_purpose::STANDARD.decode(pub_key).unwrap().try_into().unwrap();
                                    //Storing user's pub id key
                                    self.room_keys.their_idpublic_keys.insert(user.clone(), pub_key_bytes);
                                }
                                //Deriving bi-directional chain keys for secure transfer
                                //Generate first DH pair
                                let (public_dh_key, private_dh_key) = generate_dh_keypair();
                                //Storing my pub_dh_key for each user
                                self.room_keys.my_dh_pub_keys.insert(user.clone(), public_dh_key.to_vec());

                                //Compute root key
                                let rootkey = generate_shared_rootkey(pub_key_bytes, private_dh_key);
                                //Derive sending chain key
                                let sending_chainkey = generate_sender_chainkey(rootkey);
                                
                                //Encrypting Sender Key
                                let encrypted_skey_for_user = encrypt_senderkey_message(sending_chainkey, &sender_key);

                                //Storing each encrypted version of sender key, for different users to share with
                                self.room_keys.my_sender_key_encryptions.insert(user, encrypted_skey_for_user);
                            }
                        }

                        //Sending your new sender keys again
                        //Sending my sender key to every user in bytes
                        let skey_tx = self.outgoing_roomchat_msg_tx.clone().unwrap();
                        for (username, encrypted_key) in self.room_keys.my_sender_key_encryptions.clone(){
                            if let Some(pub_dh_key) = self.room_keys.my_dh_pub_keys.get(&username){
                                if username==self.username{
                                    continue;
                                }
                                let skey_descriptor_byte = [0x11].to_vec();
                                let username_bytes = username.as_bytes().to_vec();
                                let skey_binary_mesg = [skey_descriptor_byte, encrypted_key, pub_dh_key.to_vec(), username_bytes].concat();
                                //Sending ENCRYPTED SENDER KEY in bytes
                                if let Err(e) = skey_tx.send(SocketMessage::RoomSenderKey(skey_binary_mesg))
                                {
                                    println!("Couldnt send sender key message");
                                }
                            }
                        }
                        //Pushing join/leave message to chat history
                        if let Ok(mut chat_history_lock) = self.roomchat_comps.chat_history.lock() {
                            let formatted_msg = take_next_lines(msg.content);
                            chat_history_lock.push((
                                msg.username,
                                Text::from(formatted_msg),
                                get_current_time(),
                                msg.is_join_leave_msg,
                                "".to_string()
                            )); 
                        }
                    }
                    //Scroll automatically to bottom to latest chat
                    self.roomchat_comps.scroll_state.scroll_to_bottom();
                },

                Event::IncomingRoomSenderKeyMessageEvent(payload_bytes) => {
                    //Extract Sender Key Encrypted Payload, DH_Pub_Key to form shared root key and Sender_Username at end
                    let encrypted_sender_key_bytes = &payload_bytes[1..81];
                    let their_dh_pub_key_bytes: &[u8;32] = &payload_bytes[81..113].try_into().unwrap();
                    let username_bytes = &payload_bytes[113..];
                    let username = match String::from_utf8(username_bytes.to_vec()){
                        Ok(name)=> name,
                        Err(err) => "".to_string()
                    };
                    let my_priv_key_bytes: [u8;32] = self.room_keys.my_idpriv_key;
                    let rootkey = generate_shared_rootkey(*their_dh_pub_key_bytes, my_priv_key_bytes);
                    let receiving_chainkey = generate_receiver_chainkey(rootkey);
                    let recv_mkey = derive_message_key(receiving_chainkey);
                    let decrypted_senderkey_bytes = decrypt_senderkey_message(recv_mkey, encrypted_sender_key_bytes);
                    let chainkey_bytes: &[u8;32] = &decrypted_senderkey_bytes[..32].try_into().unwrap();
                    let pub_sigkey_bytes: &[u8;32] = &decrypted_senderkey_bytes[32..].try_into().unwrap();
                    //Storing user's sending key
                    self.room_keys.their_sender_keys.insert(
                        username, 
                        SenderKey{
                            chain_key: *chainkey_bytes,
                            pub_sig_key: *pub_sigkey_bytes
                        }
                    );
                },

                Event::UnknownRotateRoomChainKeyEvent(u) => {
                    if let Some(their_sender_key) = self.room_keys.their_sender_keys.get_mut(&u){
                        update_their_roomchainkey(their_sender_key);
                    }
                },

                Event::IncomingRoomMessageAckEvent(ack_type) => {
                    if matches!(ack_type, AckType::ServerAck){
                        let room_keys_data = &mut self.room_keys;
                        update_my_roomchainkey(room_keys_data);
                        if let Ok(mut chat_history_lock) = self.roomchat_comps.chat_history.lock() {
                            for message in chat_history_lock.iter_mut().rev(){
                                //my sent message
                                if message.0==self.username{
                                    //Update server ack tick
                                    message.4 = ">".to_string();
                                    break;
                                }
                            }
                        }
                    }
                }

                Event::IncomingRealtimeNotificationEvent(notification) => {
                    let notification_cloned = notification.clone(); //for second check for accepted notification
                    if let Ok(mut n_history_lock) = self.notifications_comps.notifications_history.lock() {
                        //For Message Notification, parsing its content
                        if matches!(notification.n_type, NotificationType::MESSAGE){
                            if let Some(keys) = self.dme2ee_data.dms.get_mut(&notification.from){
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
                                        //If message was decrypted successfully and is not empty
                                        if !decrypted_message.is_empty(){
                                            //Pushing to chats in data level
                                            if let Some(dm_chats) = self.dmchats_warehouse.dms_data.get_mut(&notification.from){
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

                        self.new_notis_count += 1;
                        self.notifications_comps.status = NotificationStatus::FETCHED;
                        
                    }
                    // For Accepted Notification
                    // Add user's keys, calculate and store shared secret, sending and receiving chain keys
                    match notification_cloned.n_type{
                        NotificationType::ACCEPTED => {
                            //Retrieve latest dms list
                            start_getdms_thread(self).await;
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

                                        self.dme2ee_data.dms.insert(notification_cloned.from, keys_data);     
                                    }
                                    None => {
                                        //Handle Empty Dms List
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },

                Event::IncomingDMMessageEvent(msg) => {
                    //Normal Message to be decrypted
                    if msg.is_online_offline_msg==false{
                        //Extracting Public Key sent
                        let msg_parts: Vec<&str> = msg.content.split('.').collect();
                        let ciphertext = msg_parts[0];
                        let their_dh_pub_key = msg_parts[1];
                        let their_dh_pub_key_bytes: [u8;32] = general_purpose::STANDARD.decode(their_dh_pub_key).unwrap().try_into().unwrap();
                        let mut decrypted_message = "".to_string();
                        //Comparing if they DH-RATCHETED and sent a new pub key
                        if let Some(keys) = self.dme2ee_data.dms.get_mut(&msg.username){
                            //Same DH for now
                            if keys.their_old_dh_pub_key==their_dh_pub_key_bytes{
                                //Get Recv Chainkey
                                let recv_chain_key = keys.receiving_chain_key;
                                let recv_msg_key = derive_message_key(recv_chain_key);
                                //Decrypt Message
                                decrypted_message = decrypt_dm_message(recv_msg_key, ciphertext);
                                //Rotate chain key
                                update_receiving_chainkey(keys);
                            }
                            else{
                                //Check if receiving first message
                                if keys.their_old_dh_pub_key==[0u8;32]{
                                    keys.their_old_dh_pub_key = their_dh_pub_key_bytes;
                                    //Load private key from disk
                                    let id_keys: DiskPersist<UserIdentityKeys> = DiskPersist::init("persistent-user-identity-keypair").unwrap();
                                    if let Ok(data_res) = id_keys.read(){
                                        match data_res{
                                            Some(data) => {
                                                let priv_key = data.private_identity_key;
                                                let priv_key_bytes: [u8;32] = general_purpose::STANDARD.decode(priv_key).unwrap().try_into().unwrap();
                                                let rootkey = generate_shared_rootkey(their_dh_pub_key_bytes, priv_key_bytes);
                                                let receiving_chainkey = generate_receiver_chainkey(rootkey);
                                                keys.receiving_chain_key = receiving_chainkey;
                                                let recv_mkey = derive_message_key(receiving_chainkey);
                                                //Decrypt Message
                                                decrypted_message = decrypt_dm_message(recv_mkey, ciphertext);
                                                //Rotate chain key
                                                update_receiving_chainkey(keys);
                                                //Generate new dh pair
                                                let (public_dh_key, private_dh_key) = generate_dh_keypair();
                                                let new_rootkey = generate_shared_rootkey(their_dh_pub_key_bytes, private_dh_key);
                                                let sending_chainkey = generate_sender_chainkey(new_rootkey);
                                                //Store new ratcheted keys
                                                keys.root_key = new_rootkey;
                                                keys.dh_pub_key = public_dh_key;
                                                keys.dh_priv_key = private_dh_key;
                                                keys.sending_chain_key = sending_chainkey;
                                            }
                                            None => {}
                                        }
                                    }

                                }
                                else{
                                    keys.their_old_dh_pub_key = their_dh_pub_key_bytes;
                                    let my_dh_priv = keys.dh_priv_key;
                                    let rootkey = generate_shared_rootkey(their_dh_pub_key_bytes, my_dh_priv);
                                    let receiving_chainkey = generate_receiver_chainkey(rootkey);
                                    keys.receiving_chain_key = receiving_chainkey;
                                    let recv_mkey = derive_message_key(receiving_chainkey);
                                    //Decrypt Message
                                    decrypted_message = decrypt_dm_message(recv_mkey, ciphertext);
                                    //Rotate chain key
                                    update_receiving_chainkey(keys);
                                    //Generate new dh pair
                                    let (public_dh_key, private_dh_key) = generate_dh_keypair();
                                    let new_rootkey = generate_shared_rootkey(their_dh_pub_key_bytes, private_dh_key);
                                    let sending_chainkey = generate_sender_chainkey(new_rootkey);
                                    //Store new ratcheted keys
                                    keys.root_key = new_rootkey;
                                    keys.dh_pub_key = public_dh_key;
                                    keys.dh_priv_key = private_dh_key;
                                    keys.sending_chain_key = sending_chainkey;
                                }
                            }
                        }
                        //If decrypted message is successfully decrypted and is not empty
                        if !decrypted_message.is_empty(){
                            //Send receiver's acknowledgement (>>) to sender
                            let ack_tx = self.outgoing_dmchat_msg_tx.clone().unwrap();
                            if let Err(e) = ack_tx.send(SocketMessage::Acknowledgement(AckType::ReceiverAck.byte())){
                                println!("Couldnt send receiver ack event in channel");
                            }
                            //Add Formatted Message to UI history
                            let formatted_msg = take_next_lines(decrypted_message);
                            if let Ok(mut chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                                chat_history_lock.push((
                                    msg.username, 
                                    Text::from(formatted_msg),
                                    get_current_time(),
                                    msg.is_online_offline_msg,
                                    "".to_string() 
                                ));
                            }
                        }
                    }
                    //Online Offline Message (NO NEED TO DECRYPT)
                    else if msg.is_online_offline_msg==true{
                        let formatted_msg = take_next_lines(msg.content);
                        if let Ok(mut chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                            chat_history_lock.push((
                                msg.username, 
                                Text::from(formatted_msg),
                                get_current_time(),
                                msg.is_online_offline_msg,
                                "".to_string() 
                            ));
                        }
                    }
                    //Scroll automatically to latest message
                    self.dmchat_comps.scroll_state.scroll_to_bottom();
                },

                Event::IncomingDMMessageAckEvent(ack_type) => {
                    match ack_type{
                        AckType::ServerAck => {
                            if let Ok(mut chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                                for message in chat_history_lock.iter_mut().rev(){
                                    //my sent message
                                    if message.0==self.username{
                                        //Update server ack tick
                                        message.4 = ">".to_string();
                                        break;
                                    }
                                }
                            }
                        }
                        AckType::ReceiverAck => {
                            if let Some(keys) = self.dme2ee_data.dms.get_mut(&self.dmchat_comps.to_user){
                                update_sending_chainkey(keys);
                                if let Ok(mut chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                                    for message in chat_history_lock.iter_mut().rev(){
                                        //my sent message
                                        if message.0==self.username{
                                            //Update server ack tick
                                            message.4 = ">>".to_string();
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Event::InputEvent(key_event) => {
                    
                            match self.current_screen {
                                Screens::REGISTER_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Up &&
                                       key_event.code != KeyCode::Down &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        match self.register_textarea.which_ta {
                                            0 => { self.register_textarea.username_ta.input(key_event); },
                                            1 => { self.register_textarea.userpass_ta.input(key_event); },
                                            2 => { self.register_textarea.confirmpass_ta.input(key_event); },
                                            _ => {},
                                        }
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }
            
                                Screens::LOGIN_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Up &&
                                       key_event.code != KeyCode::Down &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        match self.login_textarea.which_ta {
                                            0 => { self.login_textarea.username_ta.input(key_event); },
                                            1 => { self.login_textarea.userpass_ta.input(key_event); },
                                            _ => {},
                                        }
                                        
                                    }
                                    else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }
            
                                Screens::PUBLIC_CHAT_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Up &&
                                       key_event.code != KeyCode::Down &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        self.publicchat_comps.input_ta.input(key_event);
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }

                                Screens::ROOM_CREATION_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        self.roomcreation_textarea.roomname_ta.input(key_event);
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }

                                Screens::ROOM_JOIN_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Up &&
                                       key_event.code != KeyCode::Down &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        match self.joinroom_textarea.which_ta {
                                            0 => { self.joinroom_textarea.roomname_ta.input(key_event); },
                                            1 => { self.joinroom_textarea.roomkey_ta.input(key_event); },
                                            _ => {},
                                        }
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }

                                Screens::ROOM_CHAT_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Up &&
                                       key_event.code != KeyCode::Down &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        self.roomchat_comps.input_ta.input(key_event);
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }

                                Screens::ADD_USER_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Up &&
                                       key_event.code != KeyCode::Down &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        match self.adduser_textarea.which_ta {
                                            0 => { self.adduser_textarea.username_ta.input(key_event); },
                                            1 => { self.adduser_textarea.message_ta.input(key_event); },
                                            _ => {},
                                        }
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }

                                Screens::DM_USER_SCREEN => {
                                    
                                    self.handle_key_event(key_event)?;

                                }

                                Screens::DM_CHAT_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       key_event.code != KeyCode::Up &&
                                       key_event.code != KeyCode::Down &&
                                       key_event.code != KeyCode::Enter &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        self.dmchat_comps.input_ta.input(key_event);
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }

                                Screens::BLOCK_USER_SCREEN => {
                                    if key_event.code != KeyCode::Esc &&
                                       (!key_event.modifiers.contains(KeyModifiers::CONTROL)) {
                                        
                                        self.blockunblock_textarea.username_ta.input(key_event);
                                        
                                    } else {
                                        self.handle_key_event(key_event)?;
                                    }
                                }
            
                                _ => {
                                    self.handle_key_event(key_event)?;
                                }

                                Screens::NOTIFICATIONS_SCREEN => {
                                    
                                    self.handle_key_event(key_event)?;

                                }
                            }
                },

                Event::RegisterEvent => {
                    start_register_task(self).await;
                },

                Event::LoginEvent => {
                    start_login_task(self).await;
                },

                Event::ExitWorldChatEvent => {
                    self.current_screen = Screens::CHAT_OPTIONS_SCREEN;
                }

                Event::RoomCreationEvent => {
                    start_roomcreation_task(self).await;
                },

                Event::RoomJoinEvent => {
                    start_joinroom_task(self).await;
                },

                Event::RoomChatEvent(room_token) => {
                    //Load private key from disk
                    let mut my_priv_key = "".to_string();
                    let id_keys: DiskPersist<UserIdentityKeys> = DiskPersist::init("persistent-user-identity-keypair").unwrap();
                    if let Ok(data_res) = id_keys.read(){
                        match data_res{
                            Some(data) => {
                                my_priv_key = data.private_identity_key;
                            }
                            None => {}
                        }
                    }
                    let my_priv_key_bytes: [u8;32] = general_purpose::STANDARD.decode(my_priv_key).unwrap().try_into().unwrap();
                    self.room_keys.my_idpriv_key = my_priv_key_bytes;

                    //Retrieving Room Data and Memeber Keys for BI-directional encrypted transfer of Sender Key
                    let get_room_data_endpoint = self.endpoints.get_room_data;
                    let room_data_res = get_room_data(room_token.clone(), get_room_data_endpoint).await;

                    //For room
                    //Generating My Chain Key
                    let chain_key = generate_roomchain_key();
                    self.room_keys.chain_key = chain_key;
                    //Making Sender Key [ChainKey][Public Signature Key]
                    let pub_sig_key = self.signature_keys.public_signature_key.clone();
                    let sender_key = compose_sender_key(chain_key, pub_sig_key);
                    
                    
                    //Sending sender key to each user by encrypting with their shared root key derived message key

                    if let Some(data) = room_data_res{
                        //For all users in room
                        for user in data.room_members{
                            if user==self.username{
                                continue;
                            }
                            let mut pub_key_bytes = [0u8;32];
                            let their_pub_key_res = data.members_keys
                                    .iter()
                                    .find_map(|val| {
                                        val.as_object()?.get(&user)
                                    });
                            if let Some(k) = their_pub_key_res{
                                let pub_key = k.as_str().unwrap();
                                pub_key_bytes = general_purpose::STANDARD.decode(pub_key).unwrap().try_into().unwrap();
                                //Storing user's pub id key
                                self.room_keys.their_idpublic_keys.insert(user.clone(), pub_key_bytes);
                            }
                            //Deriving bi-directional chain keys for secure transfer
                            //Generate first DH pair
                            let (public_dh_key, private_dh_key) = generate_dh_keypair();
                            //Storing my pub_dh_key for each user
                            self.room_keys.my_dh_pub_keys.insert(user.clone(), public_dh_key.to_vec());

                            //Compute root key
                            let rootkey = generate_shared_rootkey(pub_key_bytes, private_dh_key);
                            //Derive sending chain key
                            let sending_chainkey = generate_sender_chainkey(rootkey);
                            
                            //Encrypting Sender Key
                            let encrypted_skey_for_user = encrypt_senderkey_message(sending_chainkey, &sender_key);

                            //Storing each encrypted version of sender key, for different users to share with
                            self.room_keys.my_sender_key_encryptions.insert(user, encrypted_skey_for_user);
                        }
                    }

                    match self.is_roomchat_joined {
                        true => {
                            let outgoing_tx = &self.outgoing_roomchat_msg_tx;
                            match outgoing_tx {
                                Some(roomchat_sender) => {
                                    let leave_tx = self.outgoing_roomchat_msg_tx.clone().unwrap();
                                    //Sending Leave Message only if user is NOT room owner (cuz when owner leaves, room anyways is deleted)
                                    if self.is_current_room_owner==false{
                                        if let Err(e) = leave_tx.send(SocketMessage::Leave(MessageType::ROOM(RoomMessageType::SENDER(RoomSenderMessage{
                                            username: self.username.clone(),
                                            content: format!("{} left", self.username.clone()),
                                            users: Vec::new(),
                                            whisper_mode: WhisperMode::NONE,
                                            is_join_leave_msg: true
                                        }))))
                                        {
                                            println!("Couldnt send leave message");
                                        }
                                    }

                                    if let Err(e) = roomchat_sender.send(
                                        SocketMessage::Disconnect(DisconnectType::ROOM)
                                    ) {
                                        eprintln!("Failed to send message to WebSocket: {}", e);
                                    }
                                    //OTHER ROOM ACTIVITY STARTING
                                    self.current_screen = Screens::ROOM_CHAT_SCREEN;
                                    //Initialising for new channel
                                    self.outgoing_roomchat_msg_tx = None;
                                    //Another cloned producer that sends user message events in main channel
                                    let inc_tx = main_events_channel_tx.clone();
                                    start_roomchat_websocket_task(
                                        self,
                                        self.username.clone(), 
                                        room_token, 
                                        self.endpoints.room_chat, 
                                        //chat_history,
                                        inc_tx
                                    ).await;

                                    let join_tx = self.outgoing_roomchat_msg_tx.clone().unwrap();
                                    if let Err(e) = join_tx.send(SocketMessage::Join(MessageType::ROOM(RoomMessageType::SENDER(RoomSenderMessage{
                                        username: self.username.clone(),
                                        content: format!("{} joined", self.username.clone()),
                                        users: Vec::new(),
                                        whisper_mode: WhisperMode::NONE,
                                        is_join_leave_msg: true
                                    }))))
                                    {
                                        println!("Couldnt send join message");
                                    }

                                    //Sending my sender key to every user in bytes
                                    for (username, encrypted_key) in self.room_keys.my_sender_key_encryptions.clone(){
                                        if let Some(pub_dh_key) = self.room_keys.my_dh_pub_keys.get(&username){
                                            if username==self.username{
                                                continue;
                                            }
                                            let skey_descriptor_byte = [0x11].to_vec();
                                            let username_bytes = username.as_bytes().to_vec();
                                            let skey_binary_mesg = [skey_descriptor_byte, encrypted_key, pub_dh_key.to_vec(), username_bytes].concat();
                                            //Sending ENCRYPTED SENDER KEY in bytes
                                            if let Err(e) = join_tx.send(SocketMessage::RoomSenderKey(skey_binary_mesg))
                                            {
                                                println!("Couldnt send sender key message");
                                            }
                                        }
                                    }
                                }
                                None => {
                                    //Later
                                }
                            }
                        }
                        false => {
                            /* NAVIGATING TO ROOM CHAT SCREEN */
                            self.current_screen = Screens::ROOM_CHAT_SCREEN;
                            self.is_roomchat_joined = true;
                            self.roomchat_comps.username = self.username.clone();
                            //let chat_history = Arc::clone(&self.roomchat_comps.chat_history);
                            //Another cloned producer that sends user message events in main channel
                            let inc_tx = main_events_channel_tx.clone();
                            start_roomchat_websocket_task(
                                self,
                                self.username.clone(), 
                                room_token, 
                                self.endpoints.room_chat, 
                                //chat_history,
                                inc_tx
                            ).await; 

                            let join_tx = self.outgoing_roomchat_msg_tx.clone().unwrap();
                            if let Err(e) = join_tx.send(SocketMessage::Join(MessageType::ROOM(RoomMessageType::SENDER(RoomSenderMessage{
                                username: self.username.clone(),
                                content: format!("{} joined", self.username.clone()),
                                users: Vec::new(),
                                whisper_mode: WhisperMode::NONE,
                                is_join_leave_msg: true
                            }))))
                            {
                                println!("Couldnt send join message");
                            }

                            //Sending my sender key to every user in bytes
                            for (username, encrypted_key) in self.room_keys.my_sender_key_encryptions.clone(){
                                if let Some(pub_dh_key) = self.room_keys.my_dh_pub_keys.get(&username){
                                    if username==self.username{
                                        continue;
                                    }
                                    let skey_descriptor_byte = [0x11].to_vec();
                                    let username_bytes = username.as_bytes().to_vec();
                                    let skey_binary_mesg = [skey_descriptor_byte, encrypted_key, pub_dh_key.to_vec(), username_bytes].concat();
                                    //Sending ENCRYPTED SENDER KEY in bytes
                                    if let Err(e) = join_tx.send(SocketMessage::RoomSenderKey(skey_binary_mesg))
                                    {
                                        println!("Couldnt send sender key message");
                                    }
                                }
                            }
                        }
                    }
                },

                Event::ExitRoomChatEvent => {
                    self.current_screen = Screens::ROOM_JOIN_SCREEN;
                    let status_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::default())
                        .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
                    self.joinroom_textarea.status_block = Paragraph::new("Room closed".light_red())
                        .alignment(ratatui::layout::Alignment::Center)
                        .block(status_block);

                    /* Refresh status for retries */
                    self.joinroom_textarea.task_status = JoinRoomTaskStatus::NOT_INITIATED;
                }

                Event::AddUserEvent => {
                    start_adduser_task(self).await;
                },

                Event::AcceptUserEvent(user) => {
                    start_acceptuser_thread(self, user).await;
                },

                Event::DmChatEvent(target_user) => {
                    match self.is_dmchat_joined {
                        true => {
                            let outgoing_tx = &self.outgoing_dmchat_msg_tx;
                            match outgoing_tx {
                                Some(dmchat_sender) => {
                                    let leave_tx = self.outgoing_dmchat_msg_tx.clone().unwrap();
                                    if let Err(e) = leave_tx.send(SocketMessage::Leave(MessageType::DM(DmMessage{
                                        username: self.username.clone(),
                                        content: format!("{} went offline", self.username.clone()),
                                        is_online_offline_msg: true
                                    })))
                                    {
                                        println!("Couldnt send offline message");
                                    }

                                    //Getting previous dm chat history and saving it in data warehouse
                                    if let Ok(chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                                        //Getting mutable reference of data level dm chats of previous user to be updated
                                        let previous_user = self.dmchat_comps.to_user.clone();
                                        if let Some(warehouse_chats) = self.dmchats_warehouse.dms_data.get_mut(&previous_user){
                                            //Updating UI Chat History to Data Warehouse
                                            warehouse_chats.clear(); //clear old to rewrite again
                                            //Iterating and pushing ui chat history to data warehouse chats
                                            for chat in chat_history_lock.iter().cloned(){
                                                warehouse_chats.push((
                                                    chat.0, //username
                                                    text_to_string(&chat.1), //message from Text<> to string (lines joined by "\n")
                                                    chat.2, //ui time
                                                    "".to_string(), //key timestamp
                                                    chat.3, // isonline_offline,
                                                    chat.4 //message ack
                                                ));
                                            }
                                        }
                                    }
                                    //Getting last state of keys and saving it to persistent disk
                                    let data = &self.dme2ee_data;
                                    let disk: DiskPersist<DmE2EEncryption_Data> = DiskPersist::init("persistent-dms-e2e-keys").unwrap();
                                    disk.write(data).unwrap();

                                    //Disconnecting from web socket
                                    if let Err(e) = dmchat_sender.send(
                                        SocketMessage::Disconnect(DisconnectType::DM)
                                    ) {
                                        eprintln!("Failed to send message to WebSocket: {}", e);
                                    }

                                    //-----OTHER DM ACTIVITY STARTING-----
                                    //Initialising for new dm
                                    self.dmchat_comps.chat_history.lock().unwrap().clear(); //Clear old dm chats ui history if any
                                    //Initialising for new channel
                                    self.outgoing_dmchat_msg_tx = None;
                                    //Another cloned producer that sends user message events in main channel
                                    let inc_tx = main_events_channel_tx.clone();

                                    //Changing dm chat name
                                    self.dmchat_comps.to_user = target_user.clone();

                                    //Loading dm chat history from Data Level Warehouse
                                    if let Some(user_chats) = self.dmchats_warehouse.dms_data.get(&target_user){
                                        if let Ok(mut chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                                            for chat in user_chats.iter().cloned(){
                                                let formatted_msg = take_next_lines(chat.1); //Message
                                                chat_history_lock.push((
                                                    chat.0, //Username
                                                    Text::from(formatted_msg),
                                                    chat.2, //UI Time
                                                    chat.4, //is_offline_online_msg
                                                    chat.5 // message ack
                                                ));
                                            }
                                        }
                                    }

                                    let ws_connection = start_dmchat_websocket_task(
                                            self,
                                            self.username.clone(), 
                                            target_user.clone(),
                                            self.access_token.clone(), 
                                            self.endpoints.dm_chat, 
                                            //chat_history,
                                            inc_tx
                                        ).await;

                                    match ws_connection{
                                        Ok(()) => {//Connection successful
                                            self.current_screen = Screens::DM_CHAT_SCREEN;

                                            let tx = self.outgoing_dmchat_msg_tx.clone().unwrap();
                                            if let Err(e) = tx.send(SocketMessage::Join(MessageType::DM(DmMessage{
                                                username: self.username.clone(),
                                                content: format!("{} is online", self.username.clone()),
                                                is_online_offline_msg: true
                                            })))
                                            {
                                                println!("Couldnt send online message");
                                            }
                                            //scroll to latest message
                                            self.dmchat_comps.scroll_state.scroll_to_bottom();
                                        }
                                        Err(err) => { //Error connecting to ws
                                            match self.current_screen{
                                                Screens::NOTIFICATIONS_SCREEN => {
                                                    let text = format!("Couldnt establish DM with {}", target_user);
                                                    let status_block = Block::default()
                                                        .borders(Borders::ALL)
                                                        .border_type(ratatui::widgets::BorderType::default())
                                                        .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                                                        
                                                    self.notifications_comps.action_status_block = Paragraph::new(text.light_red())
                                                        .alignment(ratatui::layout::Alignment::Center)
                                                        .block(status_block);
                                                }
                                                Screens::DM_USER_SCREEN => {
                                                    let text = format!("Couldnt establish DM with {}", target_user);
                                                    let status_block = Block::default()
                                                        .borders(Borders::ALL)
                                                        .border_type(ratatui::widgets::BorderType::default())
                                                        .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                                                        
                                                    self.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                                                        .alignment(ratatui::layout::Alignment::Center)
                                                        .block(status_block);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                None => {
                                    //Later
                                }
                            }
                        }
                        false => {
                            /* Starting First DM after startup */
                            self.dmchat_comps.username = self.username.clone();
                            
                            //Another cloned producer that sends user message events in main channel
                            let inc_tx = main_events_channel_tx.clone();

                            //Changing dm chat name
                            self.dmchat_comps.to_user = target_user.clone();

                            //Loading dm chat history from Data Level Warehouse
                            if let Some(user_chats) = self.dmchats_warehouse.dms_data.get(&target_user){
                                if let Ok(mut chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                                    for chat in user_chats.iter().cloned(){
                                        let formatted_msg = take_next_lines(chat.1); //Message
                                        chat_history_lock.push((
                                            chat.0, //Username
                                            Text::from(formatted_msg),
                                            chat.2, //UI Time
                                            chat.4, //is_offline_online_msg
                                            chat.5 // message ack
                                        ));
                                    }
                                }
                            }
                            
                            let ws_connection = start_dmchat_websocket_task(
                                self,
                                self.username.clone(), 
                                target_user.clone(),
                                self.access_token.clone(), 
                                self.endpoints.dm_chat, 
                                //chat_history,
                                inc_tx
                            ).await;

                            match ws_connection{
                                Ok(()) => {//Connection successful
                                    self.current_screen = Screens::DM_CHAT_SCREEN;
                                    self.is_dmchat_joined = true;
                                    let tx = self.outgoing_dmchat_msg_tx.clone().unwrap();
                                    if let Err(e) = tx.send(SocketMessage::Join(MessageType::DM(DmMessage{
                                        username: self.username.clone(),
                                        content: format!("{} is online", self.username.clone()),
                                        is_online_offline_msg: true
                                    })))
                                    {
                                        println!("Couldnt send online message");
                                    }
                                    //scroll to latest message
                                    self.dmchat_comps.scroll_state.scroll_to_bottom();
                                }
                                Err(err) => { //Error connecting to ws
                                    match self.current_screen{
                                        Screens::NOTIFICATIONS_SCREEN => {
                                            let text = format!("Couldnt establish DM with {}", target_user);
                                            let status_block = Block::default()
                                                .borders(Borders::ALL)
                                                .border_type(ratatui::widgets::BorderType::default())
                                                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                                                
                                            self.notifications_comps.action_status_block = Paragraph::new(text.light_red())
                                                .alignment(ratatui::layout::Alignment::Center)
                                                .block(status_block);
                                        }
                                        Screens::DM_USER_SCREEN => {
                                            let text = format!("Couldnt establish DM with {}", target_user);
                                            let status_block = Block::default()
                                                .borders(Borders::ALL)
                                                .border_type(ratatui::widgets::BorderType::default())
                                                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                                                
                                            self.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                                                .alignment(ratatui::layout::Alignment::Center)
                                                .block(status_block);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                },

                Event::ExitDmChatEvent => {
                    //Getting previous dm chat history
                    if let Ok(chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                        //Getting mutable reference of data level dm chats of previous user to be updated
                        let previous_user = self.dmchat_comps.to_user.clone();
                        if let Some(warehouse_chats) = self.dmchats_warehouse.dms_data.get_mut(&previous_user){
                            //Updating UI Chat History to Data Warehouse
                            warehouse_chats.clear(); //clear old to rewrite again
                            //Iterating and pushing ui chat history to data warehouse chats
                            for chat in chat_history_lock.iter().cloned(){
                                warehouse_chats.push((
                                    chat.0, //username
                                    text_to_string(&chat.1), //message from Text<> to string (lines joined by "\n")
                                    chat.2, //ui time
                                    "".to_string(), //key timestamp
                                    chat.3, // isonline_offline
                                    chat.4 //message ack
                                ));
                            }
                        }
                    }
                    //Getting last state of keys and saving it to persistent disk
                    let data = &self.dme2ee_data;
                    let disk: DiskPersist<DmE2EEncryption_Data> = DiskPersist::init("persistent-dms-e2e-keys").unwrap();
                    disk.write(data).unwrap();

                    //Back to DMS list screen and a alert 
                    self.current_screen = Screens::DM_USER_SCREEN;
                    let status_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::default())
                        .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
                    self.dmuser_comps.action_status_block = Paragraph::new("DM closed due to some error".light_red())
                        .alignment(ratatui::layout::Alignment::Center)
                        .block(status_block);
                }

                Event::UploadDmChatsEvent(chat_data) => {
                    let token = self.access_token.clone();
                    let upload_dm_chats_endpoint = self.endpoints.upload_dm_chats_data;
                    upload_dm_chats(token, upload_dm_chats_endpoint, chat_data).await;

                    //EXITING
                    self.exit = true;
                }

                Event::LoadDmsDataEvent => {
                    start_getdms_thread(self).await;
                    //Get latest user message from Data Warehouse and append to list
                    let warehouse_data = &mut self.dmchats_warehouse.dms_data;
                    let new_dms_list: DiskPersist<Vec<DmUser_Data>> = DiskPersist::init("persistent-user-dms-list").unwrap();
                    let new_dms = match new_dms_list.read(){
                        Ok(list) => {
                            if let Some(data) = list{
                                data
                            }
                            else{
                                Vec::new()
                            }
                        }
                        Err(err) => {
                            Vec::new()
                        }
                    };
                    let dm_users_list: Vec<String> = new_dms.iter().map(|dm_data| dm_data.username.clone()).collect();
                    let warehouse_dm_users_list: Vec<String> = warehouse_data.iter().map(|d| d.0.clone()).collect();
                    let mut dms_content = self.dmuser_comps.dms_list.lock().unwrap();
                    for user in dm_users_list.iter(){
                        if warehouse_dm_users_list.contains(user){
                            let latest_msg_data = warehouse_data.get(user)
                                    .unwrap()
                                    .iter()
                                    .rev()
                                    .next();
                            
                            if let Some(data) = latest_msg_data{
                                //Pushing latest mesg to dms list    
                                dms_content.push(DmsListData {
                                    with_user: user.clone(), 
                                    latest_msg: data.0.clone() + ": " + &data.1, // Linus: was fun night
                                    time: data.2.clone()
                                });
                            }
                            else{ //No chats till now
                                //Pushing latest mesg to dms list    
                                dms_content.push(DmsListData {
                                    with_user: user.clone(), 
                                    latest_msg: "".to_string(), 
                                    time: "".to_string()
                                });
                            }
                            
                        }
                        else{
                            warehouse_data.insert(user.to_string(), Vec::new());
                        }
                    }
                    //Changing to normal
                    let text = "Press [Enter] to DM Users".to_string();
                    let status_block = Block::default()
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::default())
                            .border_style(Style::default().fg(ratatui::style::Color::LightCyan));
                                                                
                    self.dmuser_comps.action_status_block = Paragraph::new(text.light_cyan())
                            .alignment(ratatui::layout::Alignment::Center)
                            .block(status_block);           
                }

                Event::BlockEvent => {
                    start_blockuser_task(self).await;
                },

                Event::UnblockEvent => {
                    start_unblockuser_task(self).await;
                } 

                _ => {}
            }
            

            for e in self.login_menu.drain_events() {
                match e {
                    MenuEvent::Selected(item) => match item {
                        LoginMenuAction::REGISTER => {
                            self.current_screen = Screens::REGISTER_SCREEN;
                        }
                        LoginMenuAction::LOGIN => {
                            self.current_screen = Screens::LOGIN_SCREEN;
                        }
                    },
                }
                self.login_menu.reset();
            }


            for e in self.chatoptions_menu.drain_events() {
                match e {
                    MenuEvent::Selected(item) => match item {
                        ChatOptionsAction::PUBLIC_CHAT => {
                            match self.is_pubchat_joined {
                                true => {
                                    self.current_screen = Screens::PUBLIC_CHAT_SCREEN;
                                }
                                false => {
                                    self.current_screen = Screens::PUBLIC_CHAT_SCREEN;
                                    self.is_pubchat_joined = true;
                                    self.publicchat_comps.username = self.username.clone();
                                    //Another cloned producer that sends user message events in main channel
                                    let inc_tx = main_events_channel_tx.clone();
                                    start_worldchat_websocket_task(
                                        self,
                                        self.username.clone(), 
                                        self.access_token.clone(), 
                                        self.endpoints.world_chat,  
                                        //chat_history,
                                        inc_tx
                                    ).await;

                                    let join_tx = self.outgoing_worldchat_msg_tx.clone().unwrap();
                                    if let Err(e) = join_tx.send(SocketMessage::Join(MessageType::WORLD_CHAT(WorldChatMessage{
                                        username: self.username.clone(),
                                        content: format!("{} joined", self.username.clone()),
                                        is_join_leave_msg: true
                                    })))
                                    {
                                        println!("Couldnt send join message");
                                    }
                                }
                            }
                        }
                        ChatOptionsAction::CREATE_ROOM => {
                            self.current_screen = Screens::ROOM_CREATION_SCREEN;
                        }
                        ChatOptionsAction::JOIN_ROOM => {
                            self.current_screen = Screens::ROOM_JOIN_SCREEN;
                        }
                        ChatOptionsAction::CURRENT_ROOM => {
                            match self.is_roomchat_joined {
                                true => {
                                    self.current_screen = Screens::ROOM_CHAT_SCREEN;
                                }
                                false => {
                                    self.current_screen = Screens::ROOM_JOIN_SCREEN;
                                }
                            }
                        }
                        ChatOptionsAction::ADD_USER => {
                            self.current_screen = Screens::ADD_USER_SCREEN;
                        }
                        ChatOptionsAction::DM_USER => {
                            let loaddms_tx = self.network_event_tx.clone();
                            if let Err(err) = loaddms_tx.send(Event::LoadDmsDataEvent){
                                eprintln!("Coudlnt send load dms event");
                            }
                            
                            self.current_screen = Screens::DM_USER_SCREEN;
                            //Changing Action Bar to Loading
                            let text = format!("Loading your DMs");
                            let status_block = Block::default()
                                    .borders(Borders::ALL)
                                    .border_type(ratatui::widgets::BorderType::default())
                                    .border_style(Style::default().fg(ratatui::style::Color::LightYellow));
                                                                        
                            self.dmuser_comps.action_status_block = Paragraph::new(text.light_yellow())
                                    .alignment(ratatui::layout::Alignment::Center)
                                    .block(status_block);
                        }
                        ChatOptionsAction::CURRENT_DM => {
                            match self.is_dmchat_joined {
                                true => {
                                    self.current_screen = Screens::DM_CHAT_SCREEN;
                                }
                                false => {
                                    self.current_screen = Screens::DM_USER_SCREEN;
                                }
                            }
                        }
                        ChatOptionsAction::BLOCK_USER => {
                            self.current_screen = Screens::BLOCK_USER_SCREEN;
                        }
                        ChatOptionsAction::NOTIFICATIONS => {
                            self.current_screen = Screens::NOTIFICATIONS_SCREEN;
                            
                        }
                    },
                }
                self.chatoptions_menu.reset();
            }

            
        }
        //Restoring Terminal
        ratatui::restore();
        //Exiting
        exit(1);
    }

    fn draw(&mut self, frame: &mut Frame) {
        // Draw a full screen block with a border
        let outer_block = Block::default()
            .title(
                Span::from(Span::styled(
                    "[^Q]Exit",
                    Style::default().add_modifier(Modifier::BOLD).fg(Color::White).bg(Color::Black),
                )),
            )
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Double)
            .border_style(Style::default().fg(Color::LightBlue));
    
        let full_area = frame.area();
        frame.render_widget(outer_block, full_area);
    
        // Define an inner area for content (with some margin inside the border)
        let inner_area = Layout::default()
            .direction(Direction::Vertical)
            .margin(2) // margin creates the "inner" area inside the border
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(full_area)[0];
    
        // Split the inner area into two parts: one for the header and one for the rest
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
            .split(inner_area);

        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Length(1)].as_ref())
            .split(chunks[0]);

        let big_text = BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Quadrant)
            .alignment(Alignment::Center)
            .style(Style::new().add_modifier(Modifier::ITALIC))
            .lines(vec![
                self.exiting_status.magenta().into(),
            ])
            .build();

        // Render the header at the top of the inner area
        frame.render_widget(big_text, chunks[0]);

        let dev_text = Line::from("Developed by Atharv Kumar Tiwari".magenta().italic()).centered();

        frame.render_widget(dev_text, header_chunks[1]);
        
      

        let panelarea_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(97),Constraint::Min(0)].as_ref())
            .split(chunks[1]);


        let panel_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Length(40),Constraint::Min(0)].as_ref())
            .split(panelarea_chunks[0]); 
        

        let optionsmenu_area = Layout::default()
            .direction(Direction::Vertical)
            .margin(3)
            .constraints([Constraint::Percentage(90),Constraint::Min(0)].as_ref())
            .split(panel_chunks[0]);


        //NOTIFICATIONS TEXT
        let notifications_alert = match self.new_notis_count{
            0 => "You're all caught up".to_string(),
            _ => format!("🔔You got {} new notifications", self.new_notis_count)
        };

        let optionspanel_block = Block::default()
            .title("Options".cyan().italic())
            .title_bottom(Line::from(notifications_alert).centered().bold().on_black().light_magenta())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

    
        let chatoptions_menu = Menu::new()
            .dropdown_width(30)
            .default_style(Style::default().fg(Color::White).bg(Color::Reset).add_modifier(Modifier::BOLD))
            .highlight(Style::default().fg(Color::LightMagenta).bg(Color::Black).add_modifier(Modifier::ITALIC));

    
        let help_text1 = Line::from("[Up/Down]Navigate | [Enter]Select".white().bold().on_black()).centered();



      if !matches!(&self.current_screen, Screens::WELCOME_SCREEN) &&
         !matches!(&self.current_screen, Screens::REGISTER_SCREEN) &&
         !matches!(&self.current_screen, Screens::LOGIN_SCREEN) {
        
        frame.render_widget(optionspanel_block, panel_chunks[0]);


        frame.render_stateful_widget(chatoptions_menu, optionsmenu_area[0], &mut self.chatoptions_menu);

        frame.render_widget(help_text1, optionsmenu_area[1]);
            
    }

    
        // Render additional content into chunks[1] as needed...
        match self.current_screen {
            Screens::WELCOME_SCREEN => login_menu::draw_login_menu(frame, chunks[1], &mut self.login_menu),
            Screens::REGISTER_SCREEN => register_screen::draw_register_screen(frame, chunks[1], &mut self.register_textarea),
            Screens::LOGIN_SCREEN => login_screen::draw_login_screen(frame, chunks[1], &mut self.login_textarea),
            Screens::CHAT_OPTIONS_SCREEN => chatoptions_panel::draw_chatoptions_panel(frame, panel_chunks[1]),
            Screens::PUBLIC_CHAT_SCREEN => publicchat_panel::draw_publicchat_panel(frame, panel_chunks[1], &mut self.publicchat_comps),
            Screens::ROOM_CREATION_SCREEN => roomcreate_panel::draw_roomcreate_panel(frame, panel_chunks[1], &mut self.roomcreation_textarea),
            Screens::ROOM_JOIN_SCREEN => joinroom_panel::draw_joinroom_panel(frame, panel_chunks[1], &mut self.joinroom_textarea),
            Screens::ROOM_CHAT_SCREEN => roomchat_panel::draw_roomchat_panel(frame, panel_chunks[1], &mut self.roomchat_comps),
            Screens::ADD_USER_SCREEN => adduser_panel::draw_adduser_panel(frame, panel_chunks[1], &mut self.adduser_textarea),
            Screens::DM_USER_SCREEN => dmuser_panel::draw_dmuser_panel(frame, panel_chunks[1], &mut self.dmuser_comps),
            Screens::DM_CHAT_SCREEN => dmchat_panel::draw_dmchat_panel(frame, panel_chunks[1], &mut self.dmchat_comps),
            Screens::BLOCK_USER_SCREEN => blockuser_panel::draw_blockunblockuser_panel(frame, panel_chunks[1], &mut self.blockunblock_textarea),
            Screens::NOTIFICATIONS_SCREEN => notifications_panel::draw_notifications_panel(frame, panel_chunks[1], &mut self.notifications_comps),
        }
        

    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {

        // for exiting
        if key_event.kind == KeyEventKind::Press
            && key_event.code == KeyCode::Char('q')
            && key_event.modifiers.contains(KeyModifiers::CONTROL)
        {
            //DISCONNECTING FROM PUBLIC CHAT WEB SOCKET
            let public_outgoing_tx = &self.outgoing_worldchat_msg_tx;
            match public_outgoing_tx {
                Some(pubchat_sender) => {
                    let leave_tx = self.outgoing_worldchat_msg_tx.clone().unwrap();
                    if let Err(e) = leave_tx.send(SocketMessage::Leave(MessageType::WORLD_CHAT(WorldChatMessage{
                        username: self.username.clone(),
                        content: format!("{} left", self.username.clone()),
                        is_join_leave_msg: true
                    })))
                    {
                        println!("Couldnt send leave message");
                    }

                    if let Err(e) = pubchat_sender.send(
                        SocketMessage::Disconnect(DisconnectType::WORLD_CHAT)
                    ) {
                        eprintln!("Failed to send message to WebSocket: {}", e);
                    }
                }
                None => {}
            }
            //DISCONNECTING FROM ROOM CHAT WEB SOCKET
            let room_outgoing_tx = &self.outgoing_roomchat_msg_tx;
            match room_outgoing_tx {
                Some(roomchat_sender) => {
                    let leave_tx = self.outgoing_roomchat_msg_tx.clone().unwrap();
                    //Sending Leave Message only if user is NOT room owner (cuz when owner leaves, room anyways is deleted)
                    if self.is_current_room_owner==false{
                        if let Err(e) = leave_tx.send(SocketMessage::Leave(MessageType::ROOM(RoomMessageType::SENDER(RoomSenderMessage{
                            username: self.username.clone(),
                            content: format!("{} left", self.username.clone()),
                            users: Vec::new(),
                            whisper_mode: WhisperMode::NONE,
                            is_join_leave_msg: true
                        }))))
                        {
                            println!("Couldnt send leave message");
                        }
                    }

                    if let Err(e) = roomchat_sender.send(
                        SocketMessage::Disconnect(DisconnectType::ROOM)
                    ) {
                        eprintln!("Failed to send message to WebSocket: {}", e);
                    }
                }
                None => {}
            }
            //UPDATING DMS UI CHATS HISTORY TO DATA WAREHOUSE
            if let Ok(chat_history_lock) = self.dmchat_comps.chat_history.lock() {
                //Getting mutable reference of data level dm chats of previous user to be updated
                let previous_user = self.dmchat_comps.to_user.clone();
                if let Some(warehouse_chats) = self.dmchats_warehouse.dms_data.get_mut(&previous_user){
                    //Updating UI Chat History to Data Warehouse
                    warehouse_chats.clear(); //clear old to rewrite again
                    //Iterating and pushing ui chat history to data warehouse chats
                    for chat in chat_history_lock.iter().cloned(){
                        warehouse_chats.push((
                            chat.0, //username
                            text_to_string(&chat.1), //message from Text<> to string (lines joined by "\n")
                            chat.2, // ui time
                            "".to_string(), // key timestamp
                            chat.3, //isonline_offline
                            chat.4 //message ack
                        ));
                    }
                }
            }
            //Getting last state of keys and saving it to persistent disk
            let data = &self.dme2ee_data;
            let disk: DiskPersist<DmE2EEncryption_Data> = DiskPersist::init("persistent-dms-e2e-keys").unwrap();
            disk.write(data).unwrap();
            //DISCONNECTING FROM DM CHAT WEB SOCKET
            let dm_outgoing_tx = &self.outgoing_dmchat_msg_tx;
            match dm_outgoing_tx {
                Some(dmchat_sender) => {
                    let leave_tx = self.outgoing_dmchat_msg_tx.clone().unwrap();
                    if let Err(e) = leave_tx.send(SocketMessage::Leave(MessageType::DM(DmMessage{
                        username: self.username.clone(),
                        content: format!("{} went offline", self.username.clone()),
                        is_online_offline_msg: true
                    })))
                    {
                        println!("Couldnt send offline message");
                    }

                    if let Err(e) = dmchat_sender.send(
                        SocketMessage::Disconnect(DisconnectType::DM)
                    ) {
                        eprintln!("Failed to send message to WebSocket: {}", e);
                    }
                }
                None => {}
            }
            //ENCRYPTING DM SESSION CHATS
            encrypt_dm_chats_session(&mut self.dmchats_warehouse.dms_session_key, &mut self.dmchats_warehouse.dms_data);
            //UPLOADING ENCRYPTED SESSION CHATS TO DB
            let mut chat_data_vec: Vec<ChatEntry> = Vec::new();
            for (user,chats) in self.dmchats_warehouse.dms_data.clone(){
                let modelled_chats: Vec<Message> = chats
                            .iter()
                            .cloned()
                            .map(|c| Message(c.0, c.1, c.2, c.3, c.4, c.5))
                            .collect();
                let obj_map = HashMap::from([(user, modelled_chats)]);
                let chat_entry = ChatEntry(obj_map);
                chat_data_vec.push(chat_entry);
            }
            let chats_data = ChatData(chat_data_vec);
            let uploadchats_event_tx  = self.network_event_tx.clone();
            if let Err(e) = uploadchats_event_tx.send(
                Event::UploadDmChatsEvent(chats_data)
            ) {
                eprintln!("Failed to send event: {}", e);
            }

            //EXITING AFTER DM CHATS UPLOADED
            self.exiting_status = "Exiting"
            //Exit logic in UploadDmChatsEvent
        }

        // Handling inputs for each screen
        match self.current_screen {
            Screens::WELCOME_SCREEN => handle_welcome_screen_inputs(self, key_event),
            Screens::REGISTER_SCREEN => handle_register_screen_inputs(self, key_event),
            Screens::LOGIN_SCREEN => handle_login_screen_inputs(self, key_event),
            Screens::CHAT_OPTIONS_SCREEN => handle_chat_options_screen_inputs(self, key_event),
            Screens::PUBLIC_CHAT_SCREEN => handle_public_chat_screen_inputs(self, key_event),
            Screens::ROOM_CREATION_SCREEN => handle_room_creation_screen_inputs(self, key_event),
            Screens::ROOM_JOIN_SCREEN => handle_room_join_screen_inputs(self, key_event),
            Screens::ROOM_CHAT_SCREEN => handle_room_chat_screen_inputs(self, key_event),
            Screens::ADD_USER_SCREEN => handle_add_user_screen_inputs(self, key_event),
            Screens::DM_USER_SCREEN => handle_dm_user_screen_inputs(self, key_event),
            Screens::DM_CHAT_SCREEN => handle_dm_chat_screen_inputs(self, key_event),
            Screens::BLOCK_USER_SCREEN => handle_block_user_screen_inputs(self, key_event),
            Screens::NOTIFICATIONS_SCREEN => handle_notifications_screen_inputs(self, key_event)
        }

        Ok(())
    }

}
  
