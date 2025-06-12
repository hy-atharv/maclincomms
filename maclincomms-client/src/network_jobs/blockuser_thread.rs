use disk_persist::DiskPersist;
use ratatui::{style::{ Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}};

use crate::{event_model::Event, tui_main::MaclincommsApp, tui_widgets::blockuser_textarea::BlockUnblockUserTaskStatus, user_model::DmUser_Data};

use super::{block_user::{block_user, BlockUserResponseResult}, getdms_thread::start_getdms_thread};




pub async fn start_blockuser_task(app: &mut MaclincommsApp) {

    let username_to_block = app.blockunblock_textarea.username_ta.lines()[0].to_string();


    let block_user_token = app.access_token.clone();


    let endpoint = app.endpoints.block_user;


    let block_user_result: BlockUserResponseResult = block_user(block_user_token, username_to_block.clone(), endpoint).await;


    match block_user_result {

        BlockUserResponseResult::REQUEST_ERROR => {

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

        BlockUserResponseResult::DATABASE_ERROR => {

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

        BlockUserResponseResult::UNKNOWN_ERROR => {

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

        BlockUserResponseResult::USER_ALREADY_BLOCKED => {

            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::COMPLETED;

            let text = "User is already blocked".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightYellow));
                
            app.blockunblock_textarea.status_block = Paragraph::new(text.light_yellow())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            /* Refresh status for retries */
            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::NOT_INITIATED;

        }

        BlockUserResponseResult::USER_BLOCKED => {

            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::COMPLETED;

            let text = "User blocked".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                
            app.blockunblock_textarea.status_block = Paragraph::new(text.light_green())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

            //RESETTING
            app.blockunblock_textarea.task_status = BlockUnblockUserTaskStatus::NOT_INITIATED;

            //REMOVING USER'S DETAILS LOCALLY SAVED...
            let persistent_storage: DiskPersist<Vec<DmUser_Data>> = DiskPersist::init("persistent-user-dms-list").unwrap();

            if let Err(e) = persistent_storage.read() {
                //error handling
            }
            else {
                let dms_list = persistent_storage.read().unwrap();
                match dms_list {
                    Some(mut data) => {
                        data.retain(|dm| dm.username != username_to_block);
                        //Rewrite the persistent dms list
                        persistent_storage.write(&data).unwrap();
                    }
                    None => {
                        //do nothing
                    }
                }
            }

            //FETCH your dm list again
            start_getdms_thread(app).await;
            //Remove user's keys and chats
            app.dme2ee_data.dms.remove(&username_to_block);
            app.dmchats_warehouse.dms_session_key.remove(&username_to_block);
            app.dmchats_warehouse.dms_data.remove(&username_to_block);
        }
    }
}