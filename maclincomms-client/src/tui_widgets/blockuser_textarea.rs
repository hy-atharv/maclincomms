
use ratatui::{style::{Color, Style, Stylize}, widgets::{Block, Borders, Paragraph}};
use tui_textarea::TextArea;


#[derive(Debug, Clone)]
pub struct BlockUnblockUserTextArea {
    pub username_ta: TextArea<'static>,
    pub status_block: Paragraph<'static>,
    pub task_status: BlockUnblockUserTaskStatus
}

#[derive(Debug, Clone)]
pub enum BlockUnblockUserTaskStatus {
    NOT_INITIATED,
    IN_PROGRESS,
    COMPLETED
}


impl BlockUnblockUserTextArea {
    pub fn new() -> Self {
        Self {
            username_ta: Self::get_username_textarea(),
            status_block: Self::get_status_block(),
            task_status: BlockUnblockUserTaskStatus::NOT_INITIATED
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

    pub fn get_status_block() -> Paragraph<'static> {

        let text = "Block or Unblock a user".to_string();

        
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


