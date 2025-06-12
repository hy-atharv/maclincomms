use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}, Frame};

use super::login_textarea::LoginTextArea;




pub fn draw_login_screen(
    frame: &mut Frame,
    area: Rect,
    login_ta: &mut LoginTextArea
) {

    // First, split the entire area vertically to center our menu.
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10), // Top margin
                Constraint::Length(19),     // Menu height (adjust as needed)
                Constraint::Length(3), // Bottom margin
            ]
            .as_ref(),
        )
        .split(area);

    let status_chunk = vertical_chunks[2];


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

    // The middle horizontal chunk is our final input area.
    let input_area = horizontal_chunks[1];

    // Create a block with a title and borders for the input.
    let input_block = Block::default()
        .title("Login")
        .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Color::Blue));

    
    frame.render_widget(input_block.clone(), input_area);
    
    let inner_area = input_block.inner(input_area);
    

    let block_chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(0)
    .constraints([Constraint::Length(14), Constraint::Min(0)].as_ref())
    .split(inner_area);

    let top_area = block_chunks[0];

    let horizontal_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(15), // left margin
        Constraint::Percentage(70), // menu area (center)
        Constraint::Percentage(10), // right margin
    ])
    .split(top_area);

    let top_center_area = horizontal_chunks[1];

    let ta_chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
        [
            Constraint::Length(3), // username field
            Constraint::Length(3), // password field
            Constraint::Length(2), //Gap
            Constraint::Length(3), // Submit Button
            Constraint::Min(0),    // any leftover space
        ]
        .as_ref(),
    )
    .split(top_center_area);
    
    

    frame.render_widget(&login_ta.username_ta, ta_chunks[0]);
    frame.render_widget(&login_ta.userpass_ta, ta_chunks[1]);



    let submit_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Color::Magenta));

    let submit_paragraph = Paragraph::new("Press [Enter] to submit".white().bold())
        .alignment(ratatui::layout::Alignment::Center)
        .block(submit_block);


    let original = ta_chunks[3];

        // Calculate a new width (e.g., half of the original) and center it.
    let new_width = original.width / 1;
    let new_x = original.x + (original.width - new_width) / 1;
    let submit_area = Rect::new(new_x, original.y, new_width, original.height);

    
    frame.render_widget(submit_paragraph, submit_area);


    let help_text1 = Line::from("[Up/Down]Navigate | [Esc]Back".white().bold().on_black()).centered();
    

    frame.render_widget(help_text1, block_chunks[1]);

    let original = status_chunk;

        // Calculate a new width (e.g., half of the original) and center it.
    let new_width = original.width / 2;
    let new_x = original.x + (original.width - new_width) / 2;
    let status_area = Rect::new(new_x, original.y, new_width, original.height);

    frame.render_widget(&login_ta.status_block, status_area);

}