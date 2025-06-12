use std::{sync::Arc, thread::sleep, time::Duration};

use base64::{engine::general_purpose, Engine};
use crossterm::{event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers}};
use futures_util::lock::Mutex;
use ratatui::{layout::Alignment, style::{Color, Modifier, Style, Stylize}, text::{Line, Text}, widgets::{Block, BorderType, Borders, Paragraph}};
use regex::Regex;
use serde::de::value;
use throbber_widgets_tui::CLOCK;

use crate::{crypto::encrypt_msg::{encrypt_dm_message, encrypt_room_message, sign_room_ciphertext}, event_model::Event, get_current_time, screens_model::Screens, tui_main::MaclincommsApp, tui_widgets::{adduser_textarea::AddUserTaskStatus, blockuser_textarea::BlockUnblockUserTaskStatus, joinroom_textarea::JoinRoomTaskStatus, login_textarea::LoginTaskStatus, notifications_panel::NotificationStatus, register_textarea::RegisterTaskStatus, roomcreation_textarea::RoomCreationTaskStatus}, user_model::{DmMessage, MessageType, NotificationData, NotificationType, RoomMessageType, RoomSenderMessage, SocketMessage, WhisperMode, WorldChatMessage}};




pub fn handle_welcome_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Left => app.login_menu.left(),
        KeyCode::Right => app.login_menu.right(),
        KeyCode::Down => app.login_menu.down(),
        KeyCode::Up => app.login_menu.up(),
        KeyCode::Esc => app.login_menu.reset(),
        KeyCode::Enter => app.login_menu.select(),
        _ => {}
    }

}


pub fn handle_register_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        
        KeyCode::Down => {

            let which = app.register_textarea.which_ta;
            
            app.register_textarea.which_ta = (which+1)%3;

            switch_register_textfield(app, which);
   
        },
        KeyCode::Up => {

            let which = app.register_textarea.which_ta;

            if which==1 {
                app.register_textarea.which_ta = 0;
                app.register_textarea.userpass_ta.set_cursor_line_style(Style::default());
                app.register_textarea.userpass_ta.set_cursor_style(Style::default());

                app.register_textarea.username_ta.set_cursor_line_style(Style::default());
                app.register_textarea.username_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));   
            }
            else if which==2 {
                app.register_textarea.which_ta = 1;
                app.register_textarea.confirmpass_ta.set_cursor_line_style(Style::default());
                app.register_textarea.confirmpass_ta.set_cursor_style(Style::default());

                app.register_textarea.userpass_ta.set_cursor_line_style(Style::default());
                app.register_textarea.userpass_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }
            else {
                app.register_textarea.which_ta = 2;
                app.register_textarea.username_ta.set_cursor_line_style(Style::default());
                app.register_textarea.username_ta.set_cursor_style(Style::default());

                app.register_textarea.confirmpass_ta.set_cursor_line_style(Style::default());
                app.register_textarea.confirmpass_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }

        },
        KeyCode::Esc => {
            app.current_screen = Screens::WELCOME_SCREEN;
        },
        KeyCode::Enter => {

            if app.register_textarea.is_uname_valid &&
            app.register_textarea.is_upass_valid &&
            app.register_textarea.is_upassconfirm &&
            app.username=="".to_string() &&
            matches!(app.register_textarea.task_status.clone(), RegisterTaskStatus::NOT_INITIATED) {

                app.register_textarea.task_status = RegisterTaskStatus::IN_PROGRESS;
                let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::Yellow));

                let throb_widget = throbber_widgets_tui::Throbber::default()
                                                .label("Registering...")
                                                .throbber_set(CLOCK)
                                                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
        
                app.register_textarea.status_block = Paragraph::new(vec![Line::from(throb_widget)])
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

                let register_event_tx  = app.network_event_tx.clone();

                //Sending login network event to start the async task in a separate thread
                register_event_tx.send(Event::RegisterEvent).unwrap();
            }

        },

        other => {}
    }

}
pub fn switch_register_textfield(app: &mut MaclincommsApp, which: i32) {
    if which==0 {
        // Deactivate Username TextField
        app.register_textarea.username_ta.set_cursor_line_style(Style::default());
        app.register_textarea.username_ta.set_cursor_style(Style::default());

        //activate userpass ta
        app.register_textarea.userpass_ta.set_cursor_line_style(Style::default());
        app.register_textarea.userpass_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }

    else if which==1 {
        // Deactivate Userpass
        app.register_textarea.userpass_ta.set_cursor_line_style(Style::default());
        app.register_textarea.userpass_ta.set_cursor_style(Style::default());

        app.register_textarea.username_ta.set_cursor_line_style(Style::default());
        app.register_textarea.username_ta.set_cursor_style(Style::default());
        
        //activate confirmpass ta
        //activate username ta
        app.register_textarea.confirmpass_ta.set_cursor_line_style(Style::default());
        app.register_textarea.confirmpass_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }

    else if which==2 {
        // Deactivate ConfirmPass
        app.register_textarea.confirmpass_ta.set_cursor_line_style(Style::default());
        app.register_textarea.confirmpass_ta.set_cursor_style(Style::default());
        
        //activate username ta
        app.register_textarea.username_ta.set_cursor_line_style(Style::default());
        app.register_textarea.username_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }
}





pub fn handle_login_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        
        KeyCode::Down => {

            let which = app.login_textarea.which_ta;
            
            app.login_textarea.which_ta = (which+1)%2;

            switch_login_textfield(app, which);

        },
        KeyCode::Up => {

            let which = app.login_textarea.which_ta;
            
            app.login_textarea.which_ta = (which+1)%2;

            switch_login_textfield(app, which);

        },
        KeyCode::Esc => {
            app.current_screen = Screens::WELCOME_SCREEN;
        },

        KeyCode::Enter => {

            if !(app.login_textarea.username_ta.lines()[0].to_string().is_empty()) &&
            !(app.login_textarea.userpass_ta.lines()[0].to_string().is_empty()) &&
            (app.login_textarea.username_ta.lines()[0].to_string()==app.username || app.username=="".to_string()) &&
            matches!(app.login_textarea.task_status.clone(), LoginTaskStatus::NOT_INITIATED) {

                app.login_textarea.task_status = LoginTaskStatus::IN_PROGRESS;
                let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::Yellow));

                let throb_widget = throbber_widgets_tui::Throbber::default()
                                                .label("Logging in...")
                                                .throbber_set(CLOCK)
                                                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
        
                app.login_textarea.status_block = Paragraph::new(vec![Line::from(throb_widget)])
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

                let login_event_tx  = app.network_event_tx.clone();

                //Sending login network event to start the async task in a separate thread
                login_event_tx.send(Event::LoginEvent).unwrap();
            }
            else if!(app.login_textarea.username_ta.lines()[0].to_string().is_empty()) &&
            !(app.login_textarea.userpass_ta.lines()[0].to_string().is_empty()) &&
            app.login_textarea.username_ta.lines()[0].to_string()!=app.username &&
            app.username!="".to_string() &&
            matches!(app.login_textarea.task_status.clone(), LoginTaskStatus::NOT_INITIATED) {

                let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::Red));
        
                app.login_textarea.status_block = Paragraph::new("maclincomms currently supports only 1 account per device".light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);
            }
        },

        _ => {}
    }

}
pub fn switch_login_textfield(app: &mut MaclincommsApp, which: i32) {

    if which==0 {
        // Deactivate Username TextField
        app.login_textarea.username_ta.set_cursor_line_style(Style::default());
        app.login_textarea.username_ta.set_cursor_style(Style::default());

        //activate userpass ta
        app.login_textarea.userpass_ta.set_cursor_line_style(Style::default());
        app.login_textarea.userpass_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }
    else if which==1 {
        // Deactivate Password TextField
        app.login_textarea.userpass_ta.set_cursor_line_style(Style::default());
        app.login_textarea.userpass_ta.set_cursor_style(Style::default());

        //activate username ta
        app.login_textarea.username_ta.set_cursor_line_style(Style::default());
        app.login_textarea.username_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }
}


pub fn handle_chat_options_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Down => app.chatoptions_menu.down(),
        KeyCode::Up => app.chatoptions_menu.up(),
        KeyCode::Enter => app.chatoptions_menu.select(),
        _ => {}
    }

}


pub fn handle_public_chat_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Esc => { 
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Up => app.publicchat_comps.scroll_state.scroll_up(),
        KeyCode::Down => app.publicchat_comps.scroll_state.scroll_down(),
        KeyCode::Enter => { 

            if !(app.publicchat_comps.input_ta.lines()[0].to_string().is_empty()) &&
            !(app.publicchat_comps.input_ta.lines()[0].to_string().trim().is_empty()) {

                let user_input = app.publicchat_comps.input_ta.lines()[0].to_string();

                let (cleaned_input, final_input) = clean_input_and_take_next_lines(user_input);
                
                let user_name = app.username.clone();
                 
                // Lock the chat history before modifying
                let mut chat_history = app.publicchat_comps.chat_history.lock().unwrap();
                chat_history.push((user_name.clone(), Text::from(final_input), get_current_time(), false, "".to_string()));

                app.publicchat_comps.scroll_state.scroll_to_bottom();

                let _ = app.publicchat_comps.input_ta.delete_line_by_head();

                // ✅ Send message to WebSocket
                let outgoing_tx = &app.outgoing_worldchat_msg_tx;


                match outgoing_tx {
                    Some(worldchat_ui_sender) => {
                        if let Err(e) = worldchat_ui_sender.send(
                            SocketMessage::Message(MessageType::WORLD_CHAT(
                                WorldChatMessage {
                                    username: user_name,
                                    content: cleaned_input,
                                    is_join_leave_msg: false
                                }
                            )
                        )) {
                            eprintln!("Failed to send message to WebSocket: {}", e);
                        }
                    }
                    None => {
                        //Later
                    }
                }
            }
            
        },
        _ => {}
    }

}

pub fn clean_input_and_take_next_lines(text: String) -> (String, Vec<Line<'static>>) {
    let cleaned = text.trim() // Remove leading & trailing spaces
        .split_whitespace() // Split into words, ignoring extra spaces
        .collect::<Vec<&str>>() // Collect words into a vector
        .join(" "); // Join words with a single space

    // Split by newlines and collect each line into a vector of Lines
    let lines: Vec<Line> = cleaned.split("\\n")
        .map(|line| Line::from(line.trim().to_string())) // Trim each line and convert to Line
        .filter(|line| !line.spans.is_empty()) // Remove empty lines
        .collect();
    
    (cleaned, lines)
}

pub fn clean_input(text: String) -> String {
    let cleaned = text.trim() // Remove leading & trailing spaces
        .split_whitespace() // Split into words, ignoring extra spaces
        .collect::<Vec<&str>>() // Collect words into a vector
        .join(" "); // Join words with a single space
    cleaned
}

pub fn take_next_lines(text: String) -> Vec<Line<'static>> {
    // Split by newlines and collect each line into a vector of Lines
    let lines: Vec<Line> = text.split("\\n")
        .map(|line| Line::from(line.trim().to_string())) // Trim each line and convert to Line
        .filter(|line| !line.spans.is_empty()) // Remove empty lines
        .collect();
    lines
}

pub fn text_to_string(text: &Text<'_>) -> String {
    text.lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\\n")
}


pub fn handle_room_creation_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent){

    match key_event.code {
        KeyCode::Esc => { 
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Enter => {

            if !(app.roomcreation_textarea.roomname_ta.lines()[0].to_string().is_empty())
                && matches!(app.roomcreation_textarea.task_status, RoomCreationTaskStatus::NOT_INITIATED) {

                app.roomcreation_textarea.task_status = RoomCreationTaskStatus::IN_PROGRESS;
                let status_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::default())
                    .border_style(Style::default().fg(ratatui::style::Color::Yellow));
    
                let throb_widget = throbber_widgets_tui::Throbber::default()
                    .label("Creating your room...")
                    .throbber_set(CLOCK)
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
            
                app.roomcreation_textarea.status_block = Paragraph::new(vec![Line::from(throb_widget)])
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(status_block);
    
                let room_creation_tx  = app.network_event_tx.clone();
    
                //Sending room creation network event to start the async task in a separate thread
                room_creation_tx.send(Event::RoomCreationEvent).unwrap();

            }

        },
        _ => {}
    }

}


pub fn handle_room_join_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Esc => { 
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Up => {

            let which = app.joinroom_textarea.which_ta;

            app.joinroom_textarea.which_ta = (which+1)%2;

            switch_joinroom_textfield(app, which);

        },
        KeyCode::Down => {

            let which = app.joinroom_textarea.which_ta;

            app.joinroom_textarea.which_ta = (which+1)%2;

            switch_joinroom_textfield(app, which);

        },
        KeyCode::Enter => {

            if !(app.joinroom_textarea.roomname_ta.lines()[0].to_string().is_empty()) &&
            !(app.joinroom_textarea.roomkey_ta.lines()[0].to_string().is_empty()) &&
            matches!(app.joinroom_textarea.task_status.clone(), JoinRoomTaskStatus::NOT_INITIATED) {

                app.joinroom_textarea.task_status = JoinRoomTaskStatus::IN_PROGRESS;
                let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::Yellow));

                let throb_widget = throbber_widgets_tui::Throbber::default()
                        .label("Joining room...")
                        .throbber_set(CLOCK)
                        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
        
                app.joinroom_textarea.status_block = Paragraph::new(vec![Line::from(throb_widget)])
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

                let joinroom_event_tx  = app.network_event_tx.clone();

                //Sending login network event to start the async task in a separate thread
                joinroom_event_tx.send(Event::RoomJoinEvent).unwrap();
            }
        },
        _ => {}
    }

}
pub fn switch_joinroom_textfield(app: &mut MaclincommsApp, which: i32) {

    if which==0 {
        // Deactivate Roomname TextField
        app.joinroom_textarea.roomname_ta.set_cursor_line_style(Style::default());
        app.joinroom_textarea.roomname_ta.set_cursor_style(Style::default());

        //activate room key ta
        app.joinroom_textarea.roomkey_ta.set_cursor_line_style(Style::default());
        app.joinroom_textarea.roomkey_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }
    else if which==1 {
        // Deactivate Room Key TextField
        app.joinroom_textarea.roomkey_ta.set_cursor_line_style(Style::default());
        app.joinroom_textarea.roomkey_ta.set_cursor_style(Style::default());

        //activate Room Name ta
        app.joinroom_textarea.roomname_ta.set_cursor_line_style(Style::default());
        app.joinroom_textarea.roomname_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }
}



pub fn handle_room_chat_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Esc => { 
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Up => app.roomchat_comps.scroll_state.scroll_up(),
        KeyCode::Down => app.roomchat_comps.scroll_state.scroll_down(),
        KeyCode::Enter => {

            if !(app.roomchat_comps.input_ta.lines()[0].to_string().is_empty()) &&
            !(app.roomchat_comps.input_ta.lines()[0].to_string().trim().is_empty()) {

                let user_input = app.roomchat_comps.input_ta.lines()[0].to_string();

                let cleaned_input= clean_input(user_input);

                //PARSE WHISPER COMMANDS (--hf or --sw)
                let (users_list, mode, final_message) = parse_whisper_command(&cleaned_input);

                //Take lines vector for ui
                let ui_input = take_next_lines(final_message.clone());

                
                let user_name = app.username.clone();
                 
                // Lock the chat history before modifying
                let mut chat_history = app.roomchat_comps.chat_history.lock().unwrap();
                chat_history.push((user_name.clone(), Text::from(ui_input), get_current_time(), false, "".to_string()));

                //Adding maclincomms system message to notify whisper mode used
                match mode{
                    WhisperMode::HIDE_FROM => {
                        let users = users_list.join("\\n");
                        let mut text = take_next_lines(users);
                        text.insert(0, Line::from("Message Hidden From:".bold().light_magenta()));
                        chat_history.push((
                            "maclincomms".to_string(), 
                            Text::from(text), 
                            get_current_time(), 
                            false, 
                            "".to_string()
                        ));
                    }
                    WhisperMode::SHARE_WITH => {
                        let users = users_list.join("\\n");
                        let mut text = take_next_lines(users);
                        text.insert(0, Line::from("Message Shared With:".bold().light_magenta()));
                        chat_history.push((
                            "maclincomms".to_string(), 
                            Text::from(text), 
                            get_current_time(), 
                            false, 
                            "".to_string()
                        ));
                    }
                    WhisperMode::NONE => {}
                }

                app.roomchat_comps.scroll_state.scroll_to_bottom();

                let _ = app.roomchat_comps.input_ta.delete_line_by_head();

                //Encrypting Message 
                let ciphertext = encrypt_room_message(app.room_keys.chain_key, &final_message);
                //Signing ciphertext
                let signature_priv_key = app.signature_keys.private_signature_key;
                let signature = sign_room_ciphertext(signature_priv_key, &ciphertext);
                //Appending signature to ciphertext as one message payload
                let signed_ciphertext = ciphertext + "." + &signature; 

                // ✅ Send message to WebSocket
                let outgoing_tx = &app.outgoing_roomchat_msg_tx;

                match outgoing_tx {
                    Some(roomchat_ui_sender) => {
                        if let Err(e) = roomchat_ui_sender.send(
                            SocketMessage::Message(MessageType::ROOM(
                                RoomMessageType::SENDER(
                                RoomSenderMessage {
                                    username: user_name,
                                    content: signed_ciphertext,
                                    users: users_list,
                                    whisper_mode: mode,
                                    is_join_leave_msg: false
                                }
                            )
                        ))) {
                            eprintln!("Failed to send message to WebSocket: {}", e);
                        }
                    }
                    None => {
                        //Later
                    }
                }

            }
        },
        _ => {}
    }

}
pub fn parse_whisper_command(input: &str) -> (Vec<String>, WhisperMode, String) {

    let hide_from_re = Regex::new(r#"^whisper\s+--hf\s+\[([^\[\]]+)\]"#).unwrap();

    let share_with_re = Regex::new(r#"^whisper\s+--sw\s+\[([^\[\]]+)\]"#).unwrap();
    
    if let Some(captures) = hide_from_re.captures(input) {
        if let Some(users_str) = captures.get(1) {
            let final_message = hide_from_re.replace(input, "").trim().to_string();
            return (
                users_str
                    .as_str()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
                WhisperMode::HIDE_FROM,
                final_message
            );
        }
    }
    else if let Some(captures) = share_with_re.captures(input) {
        if let Some(users_str) = captures.get(1) {
            let final_message = share_with_re.replace(input, "").trim().to_string();
            return (
                users_str
                    .as_str()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
                WhisperMode::SHARE_WITH,
                final_message
            );
        }
    }

    (Vec::new(), WhisperMode::NONE, input.to_string())
}



pub fn handle_add_user_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Down => {

            let which = app.adduser_textarea.which_ta;
            
            app.adduser_textarea.which_ta = (which+1)%2;

            switch_adduser_textfield(app, which);

        },
        KeyCode::Up => {

            let which = app.adduser_textarea.which_ta;
            
            app.adduser_textarea.which_ta = (which+1)%2;

            switch_adduser_textfield(app, which);

        },
        KeyCode::Esc => { 
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Enter => {

            if !(app.adduser_textarea.username_ta.lines()[0].to_string().is_empty())
                && matches!(app.adduser_textarea.task_status, AddUserTaskStatus::NOT_INITIATED) {
                    
                    
                app.adduser_textarea.task_status = AddUserTaskStatus::IN_PROGRESS;
                let status_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::default())
                    .border_style(Style::default().fg(ratatui::style::Color::Yellow));
    
                let throb_widget = throbber_widgets_tui::Throbber::default()
                    .label("Adding user...")
                    .throbber_set(CLOCK)
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
            
                app.adduser_textarea.status_block = Paragraph::new(vec![Line::from(throb_widget)])
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(status_block);
    
                let adduser_tx  = app.network_event_tx.clone();
    
                //Sending add user network event to start the async task in a separate thread
                adduser_tx.send(Event::AddUserEvent).unwrap();

            }

        },
        _ => {}
    }

}
pub fn switch_adduser_textfield(app: &mut MaclincommsApp, which: i32) {

    if which==0 {
        // Deactivate Username TextField
        app.adduser_textarea.username_ta.set_cursor_line_style(Style::default());
        app.adduser_textarea.username_ta.set_cursor_style(Style::default());

        //activate message ta
        app.adduser_textarea.message_ta.set_cursor_line_style(Style::default());
        app.adduser_textarea.message_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }
    else if which==1 {
        // Deactivate message TextField
        app.adduser_textarea.message_ta.set_cursor_line_style(Style::default());
        app.adduser_textarea.message_ta.set_cursor_style(Style::default());

        //activate username ta
        app.adduser_textarea.username_ta.set_cursor_line_style(Style::default());
        app.adduser_textarea.username_ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }
}


pub fn handle_dm_user_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Esc => { 
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Up => {

            if app.dmuser_comps.current_index!=0 {
                let index = app.dmuser_comps.current_index;
                app.dmuser_comps.current_index = (index-1) as i32;
            }
            app.dmuser_comps.scroll_state.scroll_up();
            app.dmuser_comps.scroll_state.scroll_up();
            app.dmuser_comps.scroll_state.scroll_up();
            app.dmuser_comps.scroll_state.scroll_up();
            app.dmuser_comps.scroll_state.scroll_up();
            app.dmuser_comps.scroll_state.scroll_up();

        },
        KeyCode::Down => {

            let dms_len = app.dmuser_comps.dms_list.lock().unwrap().len();

            if app.dmuser_comps.current_index!=((dms_len-1) as i32){
                let index = app.dmuser_comps.current_index;
                app.dmuser_comps.current_index = (index+1) as i32;
            }
            app.dmuser_comps.scroll_state.scroll_down();
            app.dmuser_comps.scroll_state.scroll_down();
            app.dmuser_comps.scroll_state.scroll_down();
            app.dmuser_comps.scroll_state.scroll_down();
            app.dmuser_comps.scroll_state.scroll_down();
            app.dmuser_comps.scroll_state.scroll_down();
            
        },
        KeyCode::Char('l') => {

           let index = app.dmuser_comps.current_index;

           let mut dms_content = app.dmuser_comps.dms_list.lock().unwrap();

        },
        KeyCode::Enter => {

            let mut dms_content = app.dmuser_comps.dms_list.lock().unwrap();

            let index = app.dmuser_comps.current_index;

            let reversed: Vec<_> = dms_content.iter().rev().collect();
            let dm_user = reversed.get(index as usize).unwrap().with_user.clone();

            let text = format!("Loading your chats with {}", dm_user.clone());
            let status_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::default())
                    .border_style(Style::default().fg(ratatui::style::Color::LightYellow));
                                                        
            app.dmuser_comps.action_status_block = Paragraph::new(text.light_yellow())
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(status_block);
 

            //EMITTING DM CHAT EVENT
            //Only if the chat aint already happening with that user
            if dm_user!=app.dmchat_comps.to_user{
                let dm_chat_tx = app.network_event_tx.clone();
                dm_chat_tx.send(Event::DmChatEvent(dm_user)).unwrap();
            }
            else{
                app.current_screen = Screens::DM_CHAT_SCREEN;
            }
        },
        _ => {}
    }

}


pub fn handle_dm_chat_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Esc => { 
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Up => app.dmchat_comps.scroll_state.scroll_up(),
        KeyCode::Down => app.dmchat_comps.scroll_state.scroll_down(),
        KeyCode::Enter => {

            if !(app.dmchat_comps.input_ta.lines()[0].to_string().is_empty()) &&
            !(app.dmchat_comps.input_ta.lines()[0].to_string().trim().is_empty()) {

                let user_input = app.dmchat_comps.input_ta.lines()[0].to_string();

                let (cleaned_input, final_input) = clean_input_and_take_next_lines(user_input);

                
                let user_name = app.username.clone();
                 
                // Lock the chat history before modifying
                let mut chat_history = app.dmchat_comps.chat_history.lock().unwrap();
                chat_history.push((user_name.clone(), Text::from(final_input), get_current_time(), false, "".to_string()));

                app.dmchat_comps.scroll_state.scroll_to_bottom();

                let _ = app.dmchat_comps.input_ta.delete_line_by_head();
                

                //Encrypting message before sending and appending User's DH Pub Key
                //[Ciphertext].[DH Pub Key]
                let mut appended_dh_key_message = "".to_string();
                if let Some(keys) = app.dme2ee_data.dms.get(&app.dmchat_comps.to_user){
                    let sending_chain_key = keys.sending_chain_key;
                    let ciphertext = encrypt_dm_message(sending_chain_key, &cleaned_input);
                    let dh_pub_key = general_purpose::STANDARD.encode(keys.dh_pub_key);
                    appended_dh_key_message = ciphertext + "." + &dh_pub_key;
                }


                // ✅ Send message to WebSocket
                let outgoing_tx = &app.outgoing_dmchat_msg_tx;

                match outgoing_tx {
                    Some(dmchat_ui_sender) => {
                        if let Err(e) = dmchat_ui_sender.send(
                            SocketMessage::Message(MessageType::DM(
                                DmMessage {
                                    username: user_name,
                                    content: appended_dh_key_message,
                                    is_online_offline_msg: false
                                }
                            )
                        )) {
                            eprintln!("Failed to send message to WebSocket: {}", e);
                        }
                    }
                    None => {
                        //Later
                    }
                }

            }
        },
        _ => {}
    }

}


pub fn handle_block_user_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    if key_event.kind == KeyEventKind::Press
        && key_event.modifiers.contains(KeyModifiers::CONTROL)
    {
        match key_event.code {
            KeyCode::Char('b') => {

                if!(app.blockunblock_textarea.username_ta.lines()[0].to_string().is_empty())
                && matches!(app.blockunblock_textarea.task_status, BlockUnblockUserTaskStatus::NOT_INITIATED) {
    
                    app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::IN_PROGRESS;
                    let status_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::default())
                        .border_style(Style::default().fg(ratatui::style::Color::Yellow));
        
                    let throb_widget = throbber_widgets_tui::Throbber::default()
                        .label("Blocking user...")
                        .throbber_set(CLOCK)
                        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
                
                    app.blockunblock_textarea.status_block = Paragraph::new(vec![Line::from(throb_widget)])
                        .alignment(ratatui::layout::Alignment::Center)
                        .block(status_block);
        
                    let blockunblock_tx  = app.network_event_tx.clone();
        
                    //Sending room creation network event to start the async task in a separate thread
                    blockunblock_tx.send(Event::BlockEvent).unwrap();
    
                }
            },
            KeyCode::Char('u') => {

                if!(app.blockunblock_textarea.username_ta.lines()[0].to_string().is_empty())
                && matches!(app.blockunblock_textarea.task_status, BlockUnblockUserTaskStatus::NOT_INITIATED) {
    
                    app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::IN_PROGRESS;
                    let status_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::default())
                        .border_style(Style::default().fg(ratatui::style::Color::Yellow));
        
                    let throb_widget = throbber_widgets_tui::Throbber::default()
                        .label("Unblocking user...")
                        .throbber_set(CLOCK)
                        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
                
                    app.blockunblock_textarea.status_block = Paragraph::new(vec![Line::from(throb_widget)])
                        .alignment(ratatui::layout::Alignment::Center)
                        .block(status_block);
        
                    let blockunblock_tx  = app.network_event_tx.clone();
        
                    //Sending room creation network event to start the async task in a separate thread
                    blockunblock_tx.send(Event::UnblockEvent).unwrap();
    
                }
            },
            _ => {}
        }
    }
    else if key_event.kind == KeyEventKind::Press
        && key_event.code == KeyCode::Esc
    {
        app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
        app.chatoptions_menu.activate();
    }


}


pub fn handle_notifications_screen_inputs( app: &mut MaclincommsApp, key_event: KeyEvent,){

    match key_event.code {
        KeyCode::Esc => { 
            app.new_notis_count = 0;
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        },
        KeyCode::Up => {

            if app.notifications_comps.current_index!=0 {
                let index = app.notifications_comps.current_index;
                app.notifications_comps.current_index = (index-1) as i32;
            }
            app.notifications_comps.scroll_state.scroll_up();
            app.notifications_comps.scroll_state.scroll_up();
            app.notifications_comps.scroll_state.scroll_up();
            app.notifications_comps.scroll_state.scroll_up();
            app.notifications_comps.scroll_state.scroll_up();
            app.notifications_comps.scroll_state.scroll_up();

        },
        KeyCode::Down => {

            let n_len = app.notifications_comps.notifications_history.lock().unwrap().len();

            if 0!=(n_len as usize){

                if app.notifications_comps.current_index!=((n_len-1) as i32){
                    let index = app.notifications_comps.current_index;
                    app.notifications_comps.current_index = (index+1) as i32;
                }
                app.notifications_comps.scroll_state.scroll_down();
                app.notifications_comps.scroll_state.scroll_down();
                app.notifications_comps.scroll_state.scroll_down();
                app.notifications_comps.scroll_state.scroll_down();
                app.notifications_comps.scroll_state.scroll_down();
                app.notifications_comps.scroll_state.scroll_down();

            }
        },
        KeyCode::Char('i') => { //Ignore

            let mut n_content = app.notifications_comps.notifications_history.lock().unwrap();

            let last_index = n_content.len()-1;

            let index = app.notifications_comps.current_index;
             
            n_content.remove(last_index - (index as usize));
 
         },
        KeyCode::Enter => {

            let mut n_content = app.notifications_comps.notifications_history.lock().unwrap();

            let index = app.notifications_comps.current_index;

            let reversed: Vec<_> = n_content.iter().rev().collect();
            let n_user = reversed.get(index as usize).unwrap().from.clone();
            let n_type = reversed.get(index as usize).unwrap().n_type.clone();

            match n_type{
                NotificationType::ADD_REQUEST => {
                    let text = format!("Adding {} to your DMs list", n_user.clone());
                    let status_block = Block::default()
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::default())
                            .border_style(Style::default().fg(ratatui::style::Color::LightYellow));
                                                                
                    app.notifications_comps.action_status_block = Paragraph::new(text.light_yellow())
                            .alignment(ratatui::layout::Alignment::Center)
                            .block(status_block);

                    //EMITTING ACCEPT REQ EVENT
                    let accept_tx = app.network_event_tx.clone();
                    accept_tx.send(Event::AcceptUserEvent(n_user)).unwrap();
                }
                _ => {
                    let text = format!("Loading your chats with {}", n_user.clone());
                    let status_block = Block::default()
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::default())
                            .border_style(Style::default().fg(ratatui::style::Color::LightYellow));
                                                                
                    app.notifications_comps.action_status_block = Paragraph::new(text.light_yellow())
                            .alignment(ratatui::layout::Alignment::Center)
                            .block(status_block);

        
                    //EMITTING DM CHAT EVENT
                    //Only if the chat aint already happening with that user
                    if n_user!=app.dmchat_comps.to_user{
                        let dm_chat_tx = app.network_event_tx.clone();
                        dm_chat_tx.send(Event::DmChatEvent(n_user)).unwrap();
                    }
                    else{
                        app.current_screen = Screens::DM_CHAT_SCREEN;
                    }
                }
            }
        },
        _ => {}
    }

}

