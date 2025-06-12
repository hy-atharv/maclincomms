use disk_persist::DiskPersist;
use ratatui::{style::{Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}};

use crate::{event_model::Event, get_current_time, screens_model::Screens, tui_main::MaclincommsApp, tui_widgets::joinroom_textarea::JoinRoomTaskStatus, user_model::Room_Keys};

use super::{get_roomdata::get_room_data,  join_room::{join_room, JoinRoomResponseResult}};




pub async fn start_joinroom_task(app: &mut MaclincommsApp) {

    let roomname = app.joinroom_textarea.roomname_ta.lines()[0].to_string();

    let roomkey = app.joinroom_textarea.roomkey_ta.lines()[0].to_string();

    let join_room_token = app.access_token.clone();


    let endpoint = app.endpoints.join_room;


    let join_room_result: JoinRoomResponseResult = join_room(join_room_token, roomname, roomkey, endpoint).await;


    match join_room_result {

        JoinRoomResponseResult::REQUEST_ERROR => {

            app.joinroom_textarea.task_status = JoinRoomTaskStatus::COMPLETED;

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.joinroom_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.joinroom_textarea.task_status = JoinRoomTaskStatus::NOT_INITIATED;

        },

        JoinRoomResponseResult::DATABASE_ERROR => {

            app.joinroom_textarea.task_status = JoinRoomTaskStatus::COMPLETED;

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.joinroom_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.joinroom_textarea.task_status = JoinRoomTaskStatus::NOT_INITIATED;

        },

        JoinRoomResponseResult::UNKNOWN_ERROR => {

            app.joinroom_textarea.task_status = JoinRoomTaskStatus::COMPLETED;

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.joinroom_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.joinroom_textarea.task_status = JoinRoomTaskStatus::NOT_INITIATED;

        }

        JoinRoomResponseResult::ROOM_NOT_FOUND => {

            app.joinroom_textarea.task_status = JoinRoomTaskStatus::COMPLETED;

            let text = "Room not found".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.joinroom_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.joinroom_textarea.task_status = JoinRoomTaskStatus::NOT_INITIATED;

        }

        JoinRoomResponseResult::INVALID_CREDENTIALS => {

            app.joinroom_textarea.task_status = JoinRoomTaskStatus::COMPLETED;

            let text = "Invalid room key".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.joinroom_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.joinroom_textarea.task_status = JoinRoomTaskStatus::NOT_INITIATED;

        }

        JoinRoomResponseResult::ROOM(room) => {

            app.joinroom_textarea.task_status = JoinRoomTaskStatus::COMPLETED;

            let text = "Room joined Successfully".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.joinroom_textarea.status_block = Paragraph::new(text.light_green())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);


//------------JOINING ROOM-------------------------------------------------------------------------------

            app.roomchat_comps.chat_history.lock().unwrap().clear(); //Clear old room chats ui history if any
            app.room_keys = Room_Keys::new(); //Clear old room keys and data

            let room_token = room.room_token;
            let room_joined_name = room.room_name;

            /* UPDATING CURRENT ROOM DATA IN APP */
            app.is_current_room_owner = false;
            app.roomchat_comps.room_name = room_joined_name;
            app.room_token = room_token.clone();

            let roomchat_event_tx = app.network_event_tx.clone();

            roomchat_event_tx.send(Event::RoomChatEvent(room_token));
                    
        }
    }
}