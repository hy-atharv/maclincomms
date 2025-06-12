use disk_persist::DiskPersist;
use ratatui::{style::{Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}};

use crate::{event_model::Event, tui_main::MaclincommsApp, tui_widgets::roomcreation_textarea::RoomCreationTaskStatus, user_model::{Room_Keys, UserIdentityKeys}};

use super::{create_room::{create_room, CreateRoomResponseResult}};




pub async fn start_roomcreation_task(app: &mut MaclincommsApp) {

    let roomname = app.roomcreation_textarea.roomname_ta.lines()[0].to_string();

    let create_room_token = app.access_token.clone();

    let persistent_storage: DiskPersist<UserIdentityKeys> = DiskPersist::init("persistent-user-identity-keypair").unwrap();

    let user_keydata = persistent_storage.read().unwrap();

    let key:String = match user_keydata {
        Some(data) => {
            let p_key = data.public_identity_key;
            p_key
        }
        None => {
            "".to_owned()
        }
    };

    let endpoint = app.endpoints.create_room;


    let create_room_result: CreateRoomResponseResult = create_room(create_room_token, roomname, key, endpoint).await;


    match create_room_result {

        CreateRoomResponseResult::REQUEST_ERROR => {

            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::COMPLETED;

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.roomcreation_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::NOT_INITIATED;

        },

        CreateRoomResponseResult::DATABASE_ERROR => {

            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::COMPLETED;

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.roomcreation_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::NOT_INITIATED;

        },

        CreateRoomResponseResult::UNKNOWN_ERROR => {

            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::COMPLETED;

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.roomcreation_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::NOT_INITIATED;

        }

        CreateRoomResponseResult::ROOM_ALREADY_EXISTS => {

            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::COMPLETED;

            let text = "Room already exists with that name".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.roomcreation_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::NOT_INITIATED;

        }

        CreateRoomResponseResult::ROOM(room) => {

            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::COMPLETED;

            let text = "Room created Successfully".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.roomcreation_textarea.status_block = Paragraph::new(text.light_green())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            //RESETTING
            app.roomcreation_textarea.task_status = RoomCreationTaskStatus::NOT_INITIATED;
            let text = "Chat with your friends in a private room!".to_string();
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::Gray));
            app.roomcreation_textarea.status_block = Paragraph::new(text.light_blue())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);



//------------OWNER JOINING ROOM-------------------------------------------------------------------------------
            
            let room_token = room.room_token;
            let room_created_name = room.room_name.clone();
            let room_created_key = room.room_key.clone();

            /* UPDATING CURRENT ROOM DATA IN APP */
            app.is_current_room_owner = true;
            app.roomchat_comps.room_name = room_created_name;
            app.roomchat_comps.room_key = room_created_key;
            app.room_token = room_token.clone();


            //DISPLAYING ROOM INFO MESG
            let info_roomname = format!("{}", app.roomchat_comps.room_name.clone());
            let info_roomkey = format!("{}", app.roomchat_comps.room_key.clone());

            // Lock the chat history before modifying
            app.roomchat_comps.chat_history.lock().unwrap().clear(); //Clear old room chats ui history if any
            app.room_keys = Room_Keys::new(); //Clear old room keys and data
            let mut chat_history = app.roomchat_comps.chat_history.lock().unwrap();
            
            //Pushing info messages to room chat history
            chat_history.push((
                "maclincomms".to_owned(), 
                Text::from(vec![
                    Line::from(vec!["ROOM NAME: ".bold().light_magenta(), info_roomname.light_cyan()]),
                    Line::from(vec!["ROOM KEY: ".bold().light_magenta(), info_roomkey.light_cyan()]),
                    Line::from(""),
                    Line::from("Share it with your friends!".light_cyan())
                ]),
                "".to_owned(),
                false,
                "".to_string()
            ));
            

            let roomchat_event_tx = app.network_event_tx.clone();

            roomchat_event_tx.send(Event::RoomChatEvent(room_token));
                    
        }
    }
}