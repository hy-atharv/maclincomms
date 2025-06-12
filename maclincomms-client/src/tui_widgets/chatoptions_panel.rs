
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}, Frame};




#[derive(Debug, Clone)]
pub enum ChatOptionsAction {
    PUBLIC_CHAT,
    CREATE_ROOM,
    JOIN_ROOM,
    CURRENT_ROOM,
    ADD_USER,
    DM_USER,
    CURRENT_DM,
    BLOCK_USER,
    NOTIFICATIONS
}

pub fn draw_chatoptions_panel(
    frame: &mut Frame,
    area: Rect
) {

    let centered_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40),Constraint::Min(0)].as_ref())
        .split(area);

    let emptychatspanel_block = Block::default()
            // .title("Options")
            // .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

    let text = Text::from(vec![
        Line::from("Chat with your fellow macOS & Linux users!".magenta().bold()),
        Line::from(""),
        Line::from("Select an option from chat options".magenta().bold())
        ]);

    let emptychats_panel = Paragraph::new(text)
            .alignment(ratatui::layout::Alignment::Center);
    
    frame.render_widget(emptychatspanel_block, area);
    frame.render_widget(emptychats_panel, centered_area[1]);

}