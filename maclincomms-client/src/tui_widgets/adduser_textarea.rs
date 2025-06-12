
use ratatui::{style::{Color, Style, Stylize}, widgets::{Block, Borders, Paragraph}};
use tui_textarea::TextArea;


#[derive(Debug, Clone)]
pub struct AddUserTextArea {
    pub username_ta: TextArea<'static>,
    pub message_ta: TextArea<'static>,
    pub which_ta: i32,
    pub status_block: Paragraph<'static>,
    pub task_status: AddUserTaskStatus
}

#[derive(Debug, Clone)]
pub enum AddUserTaskStatus {
    NOT_INITIATED,
    IN_PROGRESS,
    COMPLETED
}


impl AddUserTextArea {
    pub fn new() -> Self {
        Self {
            username_ta: Self::get_username_textarea(),
            message_ta: Self::get_message_textarea(),
            which_ta: 0,
            status_block: Self::get_status_block(),
            task_status: AddUserTaskStatus::NOT_INITIATED
        }
    }

    pub fn get_username_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_placeholder_text("Enter the username");
        ta.set_style(Style::default().fg(Color::White));
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Magenta)
                .title("Username"),
        );

        ta
    }

    pub fn get_message_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_placeholder_text("Let’s lock in and cook up a new OS fr");
        ta.set_style(Style::default().fg(Color::White));
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Magenta)
                .title("Optional Message"),
        );

        ta
    }

    pub fn get_status_block() -> Paragraph<'static> {

        let text = "Add a user".to_string();

        
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


