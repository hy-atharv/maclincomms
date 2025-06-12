use ratatui::{style::{ Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}};

use crate::{tui_main::MaclincommsApp, tui_widgets::blockuser_textarea::BlockUnblockUserTaskStatus};

use super::{unblock_user::{unblock_user, UnblockUserResponseResult}};




pub async fn start_unblockuser_task(app: &mut MaclincommsApp) {

    let username_to_unblock = app.blockunblock_textarea.username_ta.lines()[0].to_string();


    let unblock_user_token = app.access_token.clone();


    let endpoint = app.endpoints.unblock_user;


   let block_user_result: UnblockUserResponseResult = unblock_user(unblock_user_token, username_to_unblock, endpoint).await;


    match block_user_result {

        UnblockUserResponseResult::REQUEST_ERROR => {

            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::COMPLETED;

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.blockunblock_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::NOT_INITIATED;

        },

        UnblockUserResponseResult::DATABASE_ERROR => {

            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::COMPLETED;

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.blockunblock_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::NOT_INITIATED;

        },

        UnblockUserResponseResult::UNKNOWN_ERROR => {

            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::COMPLETED;

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.blockunblock_textarea.status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::NOT_INITIATED;

        }

        UnblockUserResponseResult::USER_UNBLOCKED => {

            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::COMPLETED;

            let text = "User unblocked".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.blockunblock_textarea.status_block = Paragraph::new(text.light_green())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            //RESETTING
            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::NOT_INITIATED;

        }
    }
}