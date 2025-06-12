use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Margin, Rect, Size}, style::{Color, Modifier, Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}, Frame};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};
use std::sync::{Arc, Mutex};

use crate::{get_current_date, user_model::{NotificationData, NotificationType}};



pub fn draw_notifications_panel(
    frame: &mut Frame,
    area: Rect,
    notificationscomps: &mut NotificationsComponents
) {

    let notificationspanel_block = Block::default()
            .title("Notifications")
            .title_alignment(Alignment::Center)
            .title_top(Line::from(get_current_date()).right_aligned())
            .title_top(Line::from("[Esc]Go to Options Menu").left_aligned().on_black().white())
            .title_bottom(Line::from("[Up/Down]Navigate between notifications").centered().on_black().white())
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));



    let [action_status_area, notis_area] = Layout::default()
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
        ).areas(notis_area);


    let n_chunk = inner_area;

    
    let mut notifications_content: Vec<NotificationData> = notificationscomps.notifications_history.lock().unwrap().to_vec();

    // notifications_content.push(NotificationData { n_type: NotificationType::ADD_REQUEST, from: "Rishit".to_string(), to: "Steve".to_string(), content: "Yo lets build the next great OS!!".to_string(), time: "12:43 PM".to_string() });
    // notifications_content.push(NotificationData { n_type: NotificationType::MESSAGE, from: "Arjoneel".to_string(), to: "Steve".to_string(), content: "Dude I think we should get started to work on the next great AWS services full of AI and Left Wing Ideology".to_string(), time: "1:43 AM".to_string() });
    // notifications_content.push(NotificationData { n_type: NotificationType::ACCEPTED, from: "Karthik".to_string(), to: "Steve".to_string(), content: "Karthik accepted your add request".to_string(), time: "9:03 AM".to_string() });

    let scroll_height: u16 = ((6*notifications_content.len())) as u16;

    let mut n_scroll_view = ScrollView::new(Size::new(
        n_chunk.width,
        scroll_height
    ))
    .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    let n_area = n_scroll_view.area().inner(Margin{
        horizontal: 2,
        vertical: 0
    });

    //Declaring notification block size and position
    let n_block_width = n_area.width;
    let n_block_height = 4;
    let n_block_x = n_area.x;
    let mut n_block_y = n_area.y;

    let mut index = 0;

    match notificationscomps.status{
        NotificationStatus::FETCHED => {

            for notification in notifications_content.iter().rev() {

                let n_type = notification.n_type.clone();
                let from_user = notification.from.clone();
                let n_content = if notification.content.len() > 100 {
                    format!("{}....", &notification.content[..100])
                } else {
                    notification.content.clone()
                };
                
                let n_time = notification.time.clone();

                //Formatting as per different 
                match n_type{
                    NotificationType::MESSAGE => {

                        let n_block = if notificationscomps.current_index == index {
                            Block::default()
                                .title(from_user.bold().light_magenta())
                                .title_alignment(Alignment::Left)
                                .title_top(Line::from(vec!["[Enter]DM User ".light_green().bold(), " [I]Ignore".light_yellow().bold()]).alignment(Alignment::Right))
                                .title_bottom(Line::from(n_time).right_aligned().light_magenta())
                                .borders(Borders::ALL)
                                .border_type(ratatui::widgets::BorderType::Thick)
                                .border_style(Style::default().fg(ratatui::style::Color::LightGreen))
                        } else {
                            Block::default()
                                .title(from_user.bold().light_magenta())
                                .title_alignment(Alignment::Left)
                                .title_bottom(Line::from(n_time).right_aligned().light_magenta())
                                .borders(Borders::ALL)
                                .border_type(ratatui::widgets::BorderType::Rounded)
                                .border_style(Style::default().fg(ratatui::style::Color::LightBlue))
                        };


                        let notif = Paragraph::new(vec![
                                Line::from("MESSAGE").bold().light_cyan(),
                                Line::from(n_content).white()
                            ])
                            .alignment(ratatui::layout::Alignment::Left)
                            .block(n_block);


                        let n_block_area = Rect::new(
                            n_block_x, 
                            n_block_y, 
                            n_block_width, 
                            n_block_height
                        );

                        n_block_y += n_block_height+2;

                        n_scroll_view.render_widget(notif, n_block_area);
                    }

                    NotificationType::ADD_REQUEST => {

                        let n_block = if notificationscomps.current_index == index {
                            Block::default()
                                .title(from_user.bold().light_magenta())
                                .title_alignment(Alignment::Left)
                                .title_top(Line::from(vec!["[Enter]Accept ".light_green().bold(), " [I]Ignore".light_yellow().bold()]).alignment(Alignment::Right))
                                .title_bottom(Line::from(n_time).right_aligned().light_magenta())
                                .borders(Borders::ALL)
                                .border_type(ratatui::widgets::BorderType::Thick)
                                .border_style(Style::default().fg(ratatui::style::Color::LightGreen))
                        } else {
                            Block::default()
                                .title(from_user.bold().light_magenta())
                                .title_alignment(Alignment::Left)
                                .title_bottom(Line::from(n_time).right_aligned().light_magenta())
                                .borders(Borders::ALL)
                                .border_type(ratatui::widgets::BorderType::Rounded)
                                .border_style(Style::default().fg(ratatui::style::Color::LightBlue))
                        };
                        

                        let notif = Paragraph::new(vec![
                                Line::from("ADD REQUEST".bold().light_cyan()),
                                Line::from(n_content).white()
                            ])
                            .alignment(ratatui::layout::Alignment::Left)
                            .block(n_block);


                        let n_block_area = Rect::new(
                            n_block_x, 
                            n_block_y, 
                            n_block_width, 
                            n_block_height
                        );

                        n_block_y += n_block_height+2;

                        n_scroll_view.render_widget(notif, n_block_area);
                    }

                    NotificationType::ACCEPTED => {

                        let n_block = if notificationscomps.current_index == index {
                            Block::default()
                                .title(from_user.bold().light_magenta())
                                .title_alignment(Alignment::Left)
                                .title_top(Line::from(vec!["[Enter]DM User ".light_green().bold(), " [I]Ignore".light_yellow().bold()]).alignment(Alignment::Right))
                                .title_bottom(Line::from(n_time).right_aligned().light_magenta())
                                .borders(Borders::ALL)
                                .border_type(ratatui::widgets::BorderType::Thick)
                                .border_style(Style::default().fg(ratatui::style::Color::LightGreen))
                        } else {
                            Block::default()
                                .title(from_user.bold().light_magenta())
                                .title_alignment(Alignment::Left)
                                .title_bottom(Line::from(n_time).right_aligned().light_magenta())
                                .borders(Borders::ALL)
                                .border_type(ratatui::widgets::BorderType::Rounded)
                                .border_style(Style::default().fg(ratatui::style::Color::LightBlue))
                        };
                        

                        let notif = Paragraph::new(vec![
                                Line::from("ADDED YOU").bold().light_cyan(),
                                Line::from(n_content).white()
                            ])
                            .alignment(ratatui::layout::Alignment::Left)
                            .block(n_block);


                        let n_block_area = Rect::new(
                            n_block_x, 
                            n_block_y, 
                            n_block_width, 
                            n_block_height
                        );

                        n_block_y += n_block_height+2;

                        n_scroll_view.render_widget(notif, n_block_area);
                    }
                }

                index = index + 1;
            }
        }
        NotificationStatus::NOTHING => {
            let text = Line::from("You're all caught up").centered().light_magenta().bold();

            let text_area = Rect::new(
                n_block_x, 
                n_block_y, 
                n_block_width, 
                n_block_height
            );

            frame.render_widget(text, inner_area);
        }
        NotificationStatus::ERROR => {
            let text = Line::from("Failed to retrieve notifications").centered().light_magenta().bold();

            let text_area = Rect::new(
                n_block_x, 
                n_block_y, 
                n_block_width, 
                n_block_height
            );

            frame.render_widget(text, inner_area);
        }
    }



    frame.render_widget(notificationspanel_block, area);

    frame.render_widget(&notificationscomps.action_status_block,
        Rect::new(
             action_status_area.x + (action_status_area.width/4),
             action_status_area.y,
             action_status_area.width - (action_status_area.width/2),
             action_status_area.height
    ));

    frame.render_stateful_widget(n_scroll_view, n_chunk, &mut notificationscomps.scroll_state);


}



// NOTIFICATIONS COMPONENTS

#[derive(Debug, Clone)]
pub struct NotificationsComponents {
    pub scroll_state: ScrollViewState,
    pub notifications_history: Arc<Mutex<Vec<NotificationData>>>,
    pub current_index: i32,
    pub status: NotificationStatus,
    pub action_status_block: Paragraph<'static>
}
#[derive(Debug, Clone)]
pub enum NotificationStatus {
    FETCHED,
    ERROR,
    NOTHING
}

impl NotificationsComponents {

    pub fn new() -> Self {
        Self {
            scroll_state: ScrollViewState::default(),
            notifications_history: Arc::new(Mutex::new(Vec::new())),
            current_index: 0,
            status: NotificationStatus::NOTHING,
            action_status_block: Self::get_action_status_block()
        }
    }

    pub fn get_action_status_block() -> Paragraph<'static> {

        let text = "Press [Enter] to Accept Requests/DM Users or [I] to Ignore".to_string();
        
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

