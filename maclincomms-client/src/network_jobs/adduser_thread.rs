use ratatui::{style::{ Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}};

use crate::{tui_main::MaclincommsApp, tui_widgets::adduser_textarea::AddUserTaskStatus};

use super::{add_user::{add_user, AddUserResponseResult}};




pub async fn start_adduser_task(app: &mut MaclincommsApp) {

    let username_to_add = app.adduser_textarea.username_ta.lines()[0].to_string();

    let message = match app.adduser_textarea.message_ta.lines()[0].to_string().is_empty(){
        true => "Hey! Let's be friends".to_owned(),
        false => app.adduser_textarea.message_ta.lines()[0].to_string()
    };

    let add_user_token = app.access_token.clone();


    let endpoint = app.endpoints.add_user;


    

    let add_user_result: AddUserResponseResult = add_user(add_user_token, username_to_add, message, endpoint).await;


    match add_user_result {

        AddUserResponseResult::REQUEST_ERROR => {

            app.adduser_textarea.task_status = AddUserTaskStatus::COMPLETED;

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.adduser_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.adduser_textarea.task_status = AddUserTaskStatus::NOT_INITIATED;

        },

        AddUserResponseResult::DATABASE_ERROR => {

            app.adduser_textarea.task_status = AddUserTaskStatus::COMPLETED;

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.adduser_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.adduser_textarea.task_status = AddUserTaskStatus::NOT_INITIATED;

        },

        AddUserResponseResult::UNKNOWN_ERROR => {

            app.adduser_textarea.task_status = AddUserTaskStatus::COMPLETED;

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.adduser_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.adduser_textarea.task_status = AddUserTaskStatus::NOT_INITIATED;

        }

        AddUserResponseResult::USER_NOT_FOUND => {

            app.adduser_textarea.task_status = AddUserTaskStatus::COMPLETED;

            let text = "User could not be found".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.adduser_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.adduser_textarea.task_status = AddUserTaskStatus::NOT_INITIATED;

        }

        AddUserResponseResult::NOTIFICATIONS_ERROR => {

            app.adduser_textarea.task_status = AddUserTaskStatus::COMPLETED;

            let text = "User couldnt be notified".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.adduser_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.adduser_textarea.task_status = AddUserTaskStatus::NOT_INITIATED;

        },

        AddUserResponseResult::ADD_REQUEST_SENT => {

            app.adduser_textarea.task_status = AddUserTaskStatus::COMPLETED;

            let text = "Add Request Sent".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.adduser_textarea.status_block = Paragraph::new(text.light_green())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            //RESETTING
            app.adduser_textarea.task_status = AddUserTaskStatus::NOT_INITIATED;

        }
    }
}