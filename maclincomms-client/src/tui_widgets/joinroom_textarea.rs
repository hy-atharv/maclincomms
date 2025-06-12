
use ratatui::{style::{Color, Style, Stylize}, widgets::{Block, Borders, Paragraph}};
use tui_textarea::TextArea;


#[derive(Debug, Clone)]
pub struct JoinRoomTextArea {
    pub roomname_ta: TextArea<'static>,
    pub roomkey_ta: TextArea<'static>,
    pub which_ta: i32,
    pub status_block: Paragraph<'static>,
    pub task_status: JoinRoomTaskStatus
}

#[derive(Debug, Clone)]
pub enum JoinRoomTaskStatus {
    NOT_INITIATED,
    IN_PROGRESS,
    COMPLETED
}


impl JoinRoomTextArea {
    pub fn new() -> Self {
        Self {
            roomname_ta: Self::get_roomname_textarea(),
            roomkey_ta: Self::get_roomkey_textarea(),
            which_ta: 0,
            status_block: Self::get_status_block(),
            task_status: JoinRoomTaskStatus::NOT_INITIATED
        }
    }

    pub fn get_roomname_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_placeholder_text("Enter the room name");
        ta.set_style(Style::default().fg(Color::White));
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Magenta)
                .title("Room Name"),
        );

        ta
    }

    pub fn get_roomkey_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_placeholder_text("Enter the room key");
        ta.set_style(Style::default().fg(Color::White));
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Magenta)
                .title("Room Key"),
        );

        ta
    }

    pub fn get_status_block() -> Paragraph<'static> {

        let text = "Chat with your friends in a private room!".to_string();

        
        let status_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::default())
        .border_style(Style::default().fg(ratatui::style::Color::Gray));

        let status = Paragraph::new(text.light_blue())
        .alignment(ratatui::layout::Alignment::Center)
        .block(status_block);

        status
    }

}


