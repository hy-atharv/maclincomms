
use ratatui::{style::{Color, Style, Stylize}, widgets::{Block, Borders, Paragraph}};
use tui_textarea::TextArea;


#[derive(Debug, Clone)]
pub struct LoginTextArea {
    pub username_ta: TextArea<'static>,
    pub userpass_ta: TextArea<'static>,
    pub status_block: Paragraph<'static>,
    pub which_ta: i32,
    pub task_status: LoginTaskStatus
}

#[derive(Debug, Clone)]
pub enum LoginTaskStatus {
    NOT_INITIATED,
    IN_PROGRESS,
    COMPLETED
}


impl LoginTextArea {
    pub fn new() -> Self {
        Self {
            username_ta: Self::get_username_textarea(),
            userpass_ta: Self::get_userpass_textarea(),
            status_block: Self::get_status_block(),
            which_ta: 0,
            task_status: LoginTaskStatus::NOT_INITIATED
        }
    }

    pub fn get_username_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_placeholder_text("Enter your username");
        ta.set_style(Style::default().fg(Color::White));
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Magenta)
                .title("Username"),
        );

        ta
    }
    
    pub fn get_userpass_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_mask_char('\u{2022}');
        ta.set_placeholder_text("Enter password");
        ta.set_style(Style::default().fg(Color::White));
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Magenta)
                .title("Password"),
        );

        ta
    }

    pub fn get_status_block() -> Paragraph<'static> {

        let text = "Welcome back!".to_string();
        
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




