use std::io::Stdout;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}, Frame
};
use tui_menu::{Menu, MenuItem, MenuState};



#[derive(Debug, Clone)]
pub enum LoginMenuAction {
    REGISTER,
    LOGIN
}




pub fn draw_login_menu(
    frame: &mut Frame,
    area: Rect,
    menu_state: &mut MenuState<LoginMenuAction>,
) {
    // First, split the entire area vertically to center our menu.
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10), // Top margin
                Constraint::Length(10),     // Menu height (adjust as needed)
                Constraint::Percentage(10), // Bottom margin
            ]
            .as_ref(),
        )
        .split(area);

    
    let help_text2 = Line::from("Open in fullscreen for better experience".red().bold().on_black()).centered();

    frame.render_widget(help_text2, vertical_chunks[0]);

    // Use the middle chunk as our centered area.
    let centered_area = vertical_chunks[1];

    
    // Now split the centered area horizontally to further center the menu.
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30), // Left margin
                Constraint::Percentage(40), // Menu width (adjust as needed)
                Constraint::Percentage(30), // Right margin
            ]
            .as_ref(),
        )
        .split(centered_area);

    // The middle horizontal chunk is our final menu area.
    let menu_area = horizontal_chunks[1];

    // Create a block with a title and borders for the menu.
    let menu_block = Block::default()
        .title("Welcome")
        .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Color::Blue));

    
    frame.render_widget(menu_block.clone(), menu_area);
    
    let inner_area = menu_block.inner(menu_area);

    let menu = Menu::new()
    .default_style(Style::default().fg(Color::White).bg(Color::Reset).add_modifier(Modifier::BOLD))
    // Set the style for the highlighted (selected) item
    .highlight(Style::default().fg(Color::LightMagenta).bg(Color::Black).add_modifier(Modifier::ITALIC));
    

    let vertical_chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
    .split(inner_area);

    let top_area = vertical_chunks[0];

    let horizontal_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(40), // left margin
        Constraint::Percentage(50), // menu area (center)
        Constraint::Percentage(10), // right margin
    ])
    .split(top_area);

    let top_center_area = horizontal_chunks[1];

    // Render the stateful widget.
    // The menu state is passed as a mutable reference so that it can update the selected item.
    frame.render_stateful_widget(menu, top_center_area, menu_state);

    let help_text1 = Line::from("[Enter]Select | [Up/Down]Navigate | [Esc]Back".white().bold().on_black()).centered();
    

    frame.render_widget(help_text1, vertical_chunks[1]);

    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
        .split(centered_area);

    
    
    

}
