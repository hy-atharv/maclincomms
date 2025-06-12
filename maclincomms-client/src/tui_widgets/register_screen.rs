use std::{fmt::Alignment, io::Stdout};

use ratatui::{
    layout::{ Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}, Frame
};
use regex::Regex;
use tui_textarea::TextArea;

use super::{register_textarea::RegisterTextArea};






pub fn draw_register_screen(
    frame: &mut Frame,
    area: Rect,
    register_ta: &mut RegisterTextArea
) {
    // First, split the entire area vertically to center our menu.
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10), // Top margin
                Constraint::Length(22),     // Menu height (adjust as needed)
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
        .title("Register")
        .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Color::Blue));

    
    frame.render_widget(input_block.clone(), input_area);
    
    let inner_area = input_block.inner(input_area);
    

    let block_chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(0)
    .constraints([Constraint::Length(17), Constraint::Min(0)].as_ref())
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
            Constraint::Length(3), // confirm password field
            Constraint::Length(2), //Gap
            Constraint::Length(3), // Submit Button
            Constraint::Min(0),    // any leftover space
        ]
        .as_ref(),
    )
    .split(top_center_area);


    register_ta.is_uname_valid = validate_username(&mut register_ta.username_ta);
    register_ta.is_upass_valid = validate_password(&mut register_ta.userpass_ta);
    register_ta.is_upassconfirm = confirm_password(&mut register_ta.userpass_ta, &mut register_ta.confirmpass_ta);
    

    frame.render_widget(&register_ta.username_ta, ta_chunks[0]);
    frame.render_widget(&register_ta.userpass_ta, ta_chunks[1]);
    frame.render_widget(&register_ta.confirmpass_ta, ta_chunks[2]);



    let submit_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Color::Magenta));

    let submit_paragraph = Paragraph::new("Press [Enter] to submit".white().bold())
        .alignment(ratatui::layout::Alignment::Center)
        .block(submit_block);


    let original = ta_chunks[4];

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

    frame.render_widget(&register_ta.status_block, status_area);

    

}



fn validate_username(textarea: &mut TextArea) -> bool {
    // Create a regex to match lowercase letters and digits, at least 5 characters.
    // The regex pattern "^[a-z0-9]{5,}$" means:
    // ^            : start of string
    // [a-z0-9]    : any lowercase letter or digit
    // {5,15}         : at least 5 times
    // $            : end of string
    let re = Regex::new("^[A-Za-z0-9_]{5,15}$").unwrap();

    // Convert the first line of the textarea to a &str.
    let username = textarea.lines()[0].as_str();

    if !re.is_match(username) {
        // Validation failed: style the text area as invalid.
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title("Invalid Username"),
        );
        false
    } else {
        // Validation passed: style the text area as acceptable.
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightGreen)
                .title("Valid Username"),
        );
        true
    }
}


fn validate_password(textarea: &mut TextArea) -> bool {
    let password = textarea.lines()[0].as_str();

    // Check length first.
    if password.len() < 8 {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title("Weak Password (too short)"),
        );
        return false;
    }

    // Create regexes for each requirement.
    let letter_re = Regex::new("[A-Za-z]").unwrap();
    let digit_re = Regex::new(r"\d").unwrap();
    let special_re = Regex::new(r"[^A-Za-z0-9]").unwrap();

    if !letter_re.is_match(password) {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title("Weak Password (missing letter)"),
        );
        return false;
    }

    if !digit_re.is_match(password) {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title("Weak Password (missing digit)"),
        );
        return false;
    }

    if !special_re.is_match(password) {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title("Weak Password (missing special char)"),
        );
        return false;
    }

    // If all checks pass, the password is strong.
    textarea.set_style(Style::default().fg(Color::LightGreen));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Color::LightGreen)
            .title("Good Password"),
    );
    true
}


pub fn confirm_password(pass_textarea: &mut TextArea, confirmpass_textarea: &mut TextArea) -> bool {
    let password = pass_textarea.lines()[0].as_str();
    let confirm_password = confirmpass_textarea.lines()[0].as_str();

    if confirm_password!=password && confirm_password.len()<8 {
        confirmpass_textarea.set_style(Style::default().fg(Color::LightRed));
        confirmpass_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title("Wrong Password"),
        );
        return false;
    }

    else if confirm_password==password {
        confirmpass_textarea.set_style(Style::default().fg(Color::LightGreen));
        confirmpass_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightGreen)
                .title("Alright, let's maclincomm"),
        );
        return true;
    }

    else {
        confirmpass_textarea.set_style(Style::default().fg(Color::LightRed));
        confirmpass_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title("Wrong Password"),
        );
        return false;
    }
}
