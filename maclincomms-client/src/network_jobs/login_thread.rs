use disk_persist::DiskPersist;
use ratatui::{style::{Style, Stylize}, widgets::{Block, Borders, Paragraph}};

use crate::{login_user::{login, LoginResponseResult}, screens_model::Screens, tui_main::MaclincommsApp, tui_widgets::login_textarea::LoginTaskStatus, user_model::UserData};



pub async fn start_login_task(app: &mut MaclincommsApp) {

    let username = app.login_textarea.username_ta.lines()[0].to_string();
    let password = app.login_textarea.userpass_ta.lines()[0].to_string();


    // The returned value is a Result<(String, LoginResponseResult), JoinError>.
    let login_result: (String, LoginResponseResult, String, i64) = login(username, password).await;

                
    let user = login_result.0;
    let res = login_result.1;
    let refreshtoken = login_result.2;
    let exp = login_result.3;

                

    match res {

        LoginResponseResult::REQUEST_ERROR => {

            app.login_textarea.task_status = LoginTaskStatus::COMPLETED;

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.login_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.login_textarea.task_status = LoginTaskStatus::NOT_INITIATED;

        },

        LoginResponseResult::DATABASE_ERROR => {

            app.login_textarea.task_status = LoginTaskStatus::COMPLETED;

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.login_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.login_textarea.task_status = LoginTaskStatus::NOT_INITIATED;

        },

        LoginResponseResult::UNKNOWN_ERROR => {

            app.login_textarea.task_status = LoginTaskStatus::COMPLETED;

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.login_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.login_textarea.task_status = LoginTaskStatus::NOT_INITIATED;

        }

        LoginResponseResult::USER_NOT_FOUND => {

            app.login_textarea.task_status = LoginTaskStatus::COMPLETED;

            let text = "User not found".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.login_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.login_textarea.task_status = LoginTaskStatus::NOT_INITIATED;

        }

        LoginResponseResult::INVALID_USER => {

            app.login_textarea.task_status = LoginTaskStatus::COMPLETED;

            let text = "Invalid credentials".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.login_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.login_textarea.task_status = LoginTaskStatus::NOT_INITIATED;

        }

        LoginResponseResult::TOKEN(token) => {

            app.login_textarea.task_status = LoginTaskStatus::COMPLETED;

            let text = "Logged in".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.login_textarea.status_block = Paragraph::new(text.light_green())
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