use disk_persist::DiskPersist;
use ratatui::{style::{ Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}};

use crate::{tui_main::MaclincommsApp, user_model::DmUser_Data};

use super::get_dms::{get_dms, GetDmsResponseResult};






pub async fn start_getdms_thread(app: &mut MaclincommsApp) {

    let get_dms_token = app.access_token.clone();

    let endpoint = app.endpoints.get_dms_data;

    let get_dms_result = get_dms(get_dms_token, endpoint).await;
    

    match get_dms_result {

        GetDmsResponseResult::REQUEST_ERROR => {

            let text = "Network error or bad request".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        },

        GetDmsResponseResult::DATABASE_ERROR => {

            let text = "Database Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        },

        GetDmsResponseResult::UNKNOWN_ERROR => {

            let text = "Unknown Server Error".to_string();
                
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::default())
                .border_style(Style::default().fg(ratatui::style::Color::LightRed));
                
            app.dmuser_comps.action_status_block = Paragraph::new(text.light_red())
                .alignment(ratatui::layout::Alignment::Center)
                .block(status_block);

        }

        GetDmsResponseResult::DMS_DATA_FETCHED(data) => {

            //logic to store dms data and keys
            let persistent_storage: DiskPersist<Vec<DmUser_Data>> = DiskPersist::init("persistent-user-dms-list").unwrap();
            persistent_storage.write(&data).unwrap();
        }
    }
}