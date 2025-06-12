use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Margin, Rect, Size}, style::{Color, Modifier, Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}, Frame};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};
use std::sync::{Arc, Mutex};

use crate::{get_current_date, user_model::{DmsListData, NotificationData, NotificationType}};



pub fn draw_dmuser_panel(
    frame: &mut Frame,
    area: Rect,
    dmusercomps: &mut DmUserComponents
) {

    let dmuserpanel_block = Block::default()
            .title("DMs")
            .title_alignment(Alignment::Center)
            .title_top(Line::from(get_current_date()).right_aligned())
            .title_top(Line::from("[Esc]Go to Options Menu").left_aligned().on_black().white())
            .title_bottom(Line::from("[Up/Down]Navigate between dms").centered().on_black().white())
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

    let [action_status_area, dmslist_area] = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Fill(1)
            ]
        ).areas(area);

    let [_, inner_area, __] = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
        ).areas(dmslist_area);

    let dms_chunk = inner_area;

    
    let mut dms_content: Vec<DmsListData> = dmusercomps.dms_list.lock().unwrap().to_vec();

    // notifications_content.push(NotificationData { n_type: NotificationType::ADD_REQUEST, from: "Rishit".to_string(), to: "Steve".to_string(), content: "Yo lets build the next great OS!!".to_string(), time: "12:43 PM".to_string() });
    // notifications_content.push(NotificationData { n_type: NotificationType::MESSAGE, from: "Arjoneel".to_string(), to: "Steve".to_string(), content: "Dude I think we should get started to work on the next great AWS services full of AI and Left Wing Ideology".to_string(), time: "1:43 AM".to_string() });
    // notifications_content.push(NotificationData { n_type: NotificationType::ACCEPTED, from: "Karthik".to_string(), to: "Steve".to_string(), content: "Karthik accepted your add request".to_string(), time: "9:03 AM".to_string() });

    let scroll_height: u16 = ((5*dms_content.len())) as u16;

    let mut dms_scroll_view = ScrollView::new(Size::new(
        dms_chunk.width,
        scroll_height
    ))
    .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    let dms_area = dms_scroll_view.area().inner(Margin{
        horizontal: 2,
        vertical: 0
    });

    //Declaring notification block size and position
    let dm_block_width = dms_area.width;
    let dm_block_height = 3;
    let dm_block_x = dms_area.x;
    let mut dm_block_y = dms_area.y;

    let mut index = 0;

    for dm in dms_content.iter().rev() {

        let with_user = dm.with_user.clone();
        let latest_msg = if dm.latest_msg.len() > 100 {
            format!("{}....", &dm.latest_msg[..100])
        } else {
            dm.latest_msg.clone()
        };
        
        let dm_time = dm.time.clone();

        //Formatting as per different 
        let dm_block = if dmusercomps.current_index == index {
            Block::default()
                .title(with_user.bold().light_magenta())
                .title_alignment(Alignment::Left)
                .title_top(Line::from("[Enter]DM User ".light_green().bold()).alignment(Alignment::Right))
                .title_bottom(Line::from(dm_time).right_aligned().light_magenta())
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Thick)
                .border_style(Style::default().fg(ratatui::style::Color::LightGreen))
        } else {
            Block::default()
                .title(with_user.bold().light_magenta())
                .title_alignment(Alignment::Left)
                .title_bottom(Line::from(dm_time).right_aligned().light_magenta())
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(ratatui::style::Color::LightBlue))
        };


        let user_dm = Paragraph::new(Line::from(latest_msg).gray())
                .alignment(ratatui::layout::Alignment::Left)
                .block(dm_block);


        let dm_block_area = Rect::new(
            dm_block_x, 
            dm_block_y, 
            dm_block_width, 
            dm_block_height
        );

        dm_block_y += dm_block_height+2;

        dms_scroll_view.render_widget(user_dm, dm_block_area);


        index = index + 1;
    }

    frame.render_widget(dmuserpanel_block, area);

    frame.render_widget(&dmusercomps.action_status_block,
        Rect::new(
             action_status_area.x + (action_status_area.width/4),
             action_status_area.y,
             action_status_area.width - (action_status_area.width/2),
             action_status_area.height
    ));

    frame.render_stateful_widget(dms_scroll_view, dms_chunk, &mut dmusercomps.scroll_state);

}



// NOTIFICATIONS COMPONENTS

#[derive(Debug, Clone)]
pub struct DmUserComponents {
    pub scroll_state: ScrollViewState,
    pub dms_list: Arc<Mutex<Vec<DmsListData>>>,
    pub current_index: i32,
    pub action_status_block: Paragraph<'static> 
}

impl DmUserComponents {

    pub fn new() -> Self {
        Self {
            scroll_state: ScrollViewState::default(),
            dms_list: Arc::new(Mutex::new(Vec::new())),
            current_index: 0,
            action_status_block: Self::get_action_status_block()
        }
    }

    pub fn get_action_status_block() -> Paragraph<'static> {

        let text = "Press [Enter] to DM Users".to_string();
        
        let status_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::default())
        .border_style(Style::default().fg(ratatui::style::Color::LightCyan));

        let status = Paragraph::new(text.light_cyan())
        .alignment(ratatui::layout::Alignment::Center)
        .block(status_block);

        status
    }
}

