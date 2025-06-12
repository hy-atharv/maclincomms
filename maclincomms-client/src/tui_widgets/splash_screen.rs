use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize}; // Ensure tui-big-text is added to Cargo.toml

pub fn draw_splash_screen(frame: &mut Frame) {
    let area = frame.size();

    // Optional: Clear the screen before rendering
    frame.render_widget(Clear, area);

    // Base layout with black background
    let base_block = Block::default().style(Style::default());
    frame.render_widget(base_block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    // Render BigText centered
    let big_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::ITALIC),
        )
        .lines(vec!["maclincomms".into()])
        .build();

    frame.render_widget(big_text, layout[1]);

    // Paragraph block under big text
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .style(Style::default());

    let paragraph = Paragraph::new("Loading...".bold().italic())
        .style(Style::default().fg(Color::White))
        .block(block)
        .alignment(Alignment::Center);

    let h_parts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(layout[2]);

    frame.render_widget(paragraph, h_parts[1]);
}
