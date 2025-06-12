use disk_persist::DiskPersist;
use ratatui::{style::{Style, Stylize}, widgets::{Block, Borders, Paragraph}};

use crate::{register_user::{register, RegisterResponseResult}, screens_model::Screens, tui_main::MaclincommsApp, tui_widgets::register_textarea::RegisterTaskStatus, user_model::UserData};



pub async fn start_register_task(app: &mut MaclincommsApp) {

    let username = app.register_textarea.username_ta.lines()[0].to_string();
    let password = app.register_textarea.userpass_ta.lines()[0].to_string();



    // The returned value is a Result<(String, LoginResponseResult), JoinError>.
    let register_result= register(username, password).await;

                
    let user = register_result.0;
    let res = register_result.1;
    let refreshtoken = register_result.2;
    let exp = register_result.3;

                

    match res {

        RegisterResponseResult::REQUEST_ERROR => {

            app.register_textarea.task_status = RegisterTaskStatus::COMPLETED;

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.register_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.register_textarea.task_status = RegisterTaskStatus::NOT_INITIATED;

        },

        RegisterResponseResult::DATABASE_ERROR => {

            app.register_textarea.task_status = RegisterTaskStatus::COMPLETED;

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.register_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.register_textarea.task_status = RegisterTaskStatus::NOT_INITIATED;

        },

        RegisterResponseResult::UNKNOWN_ERROR => {

            app.register_textarea.task_status = RegisterTaskStatus::COMPLETED;

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.register_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.register_textarea.task_status = RegisterTaskStatus::NOT_INITIATED;

        }

        RegisterResponseResult::EXISTING_USER => {

            app.register_textarea.task_status = RegisterTaskStatus::COMPLETED;

            let text = "Username already exists".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.register_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.register_textarea.task_status = RegisterTaskStatus::NOT_INITIATED;

        }

        RegisterResponseResult::TOKEN(token) => {

            app.register_textarea.task_status = RegisterTaskStatus::COMPLETED;

            let text = "Registered Successfully".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.register_textarea.status_block = Paragraph::new(text.light_green())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* STORING USERNAME */
            app.username = user.clone();

            /* STORING ACCESS TOKEN */
            app.access_token = token.clone();

            /* STORING REFRESH TOKEN */
            app.refresh_token = refreshtoken.clone();

            /* STORING TOKEN EXPIRY TIMESTAMP */
            app.token_expiry = exp;

            /* WRITING TO PERSISTENT STORAGE */
            let data = UserData { 
                username: user, 
                access_token: token, 
                refresh_token: refreshtoken, 
                token_expiry: exp 
            };
            let persistent_storage: DiskPersist<UserData> = DiskPersist::init("persistent-user-data").unwrap();
            persistent_storage.write(&data).unwrap();

            /* SWITCHING SCREEN */
            app.current_screen = Screens::CHAT_OPTIONS_SCREEN;
            app.chatoptions_menu.activate();
        }
    }
}