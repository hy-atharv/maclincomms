
use ratatui::{style::{Style, Stylize}, widgets::{Block, Borders, Paragraph}};
use tui_textarea::TextArea;


#[derive(Debug, Clone)]
pub struct RegisterTextArea {
    pub username_ta: TextArea<'static>,
    pub userpass_ta: TextArea<'static>,
    pub confirmpass_ta: TextArea<'static>,
    pub status_block: Paragraph<'static>,
    pub is_uname_valid: bool,
    pub is_upass_valid: bool,
    pub is_upassconfirm: bool,
    pub which_ta: i32,
    pub task_status: RegisterTaskStatus
}

#[derive(Debug, Clone)]
pub enum RegisterTaskStatus {
    NOT_INITIATED,
    IN_PROGRESS,
    COMPLETED
}


impl RegisterTextArea {
    pub fn new() -> Self {
        Self {
            username_ta: Self::get_username_textarea(),
            userpass_ta: Self::get_userpass_textarea(),
            confirmpass_ta: Self::get_confirm_userpass_textarea(),
            status_block: Self::get_status_block(),
            is_uname_valid: false,
            is_upass_valid: false,
            is_upassconfirm: false,
            which_ta: 0,
            task_status: RegisterTaskStatus::NOT_INITIATED
        }
    }

    pub fn get_username_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_placeholder_text("Enter a unique username");
        ta
    }
    
    pub fn get_userpass_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_mask_char('\u{2022}');
        ta.set_placeholder_text("Create a strong password");
        ta
    }
    
    pub fn get_confirm_userpass_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_mask_char('\u{2022}');
        ta.set_placeholder_text("Confirm your password");
        ta
    }

    pub fn get_status_block() -> Paragraph<'static> {

        let text = "Lets get started!".to_string();

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




