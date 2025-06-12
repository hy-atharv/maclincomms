use std::sync::{Arc, Mutex};

use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Margin, Rect, Size}, style::{Color, Modifier, Style, Stylize}, text::{Line, Text, ToLine}, widgets::{Block, Borders, Padding, Paragraph, Wrap}, Frame};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};
use tui_textarea::TextArea;

use crate::get_current_date;


pub fn draw_publicchat_panel(
    frame: &mut Frame,
    area: Rect,
    pubchatcomps: &mut PublicChatComponents
) {

    let chatpanel_block = Block::default()
            .title("World Chat")
            .title_alignment(Alignment::Center)
            .title_top(Line::from(get_current_date()).right_aligned())
            .title_top(Line::from("[Esc]Go to Options Menu").left_aligned().on_black().white())
            .title_bottom(Line::from("[Up/Down]Scroll chats  |  [Enter]Send message").centered().on_black().white())
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

    let [chat_chunk, _, input_chunk] = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Fill(1),
                    Constraint::Percentage(2), //Spacing
                    Constraint::Percentage(10)
                ]
            ).areas(area);

    let chat_content: Vec<Text> = {

        let chat_history = pubchatcomps.chat_history.lock().unwrap(); // Lock before iterating

        chat_history.iter()
            .map(|(_, text, _, _, _)| text.clone()) // Extract the `Text` from each tuple
            .collect()
    };


    let scroll_height = get_total_scroll_height(chat_content, chat_chunk);
           

    let mut chats_scroll_view = ScrollView::new(Size::new(
                        chat_chunk.width,
                        scroll_height
                    ))
                    .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);


    let chat_area = chats_scroll_view.area().inner(Margin {
                                horizontal: 2,  // Left & Right Padding
                                vertical: 0,    // Top & Bottom Padding
                            });


    let [others_chat_chunk, join_msg_chunk, user_chat_chunk] = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Fill(1),
                        Constraint::Percentage(10), //Spacing
                        Constraint::Fill(1)
                    ]
                ).areas(chat_area);

    let mut msg_block_y = user_chat_chunk.y;

// LOOP FOR MAKING CHAT BUBBLES -------
    let chat_history = pubchatcomps.chat_history.lock().unwrap();

    for mesg_parts in chat_history.iter() {

        let mesg_text = mesg_parts.1.clone();

        let mesg_line = mesg_parts.1.to_line();

        let username = mesg_parts.0.clone();

        let is_join_leave_msg = mesg_parts.3;

        let message_ack = mesg_parts.4.as_str();

        // YOUR CHAT BUBBLE
        if username==pubchatcomps.username {

            let msg_height = get_mesg_height(mesg_text.clone(), user_chat_chunk.width);

            let (msg_width, msg_block_x) = get_msg_width(true, mesg_line.clone(), user_chat_chunk.width, user_chat_chunk.x);

            let message_block = Block::default()
                    .title(username)
                    .title_alignment(Alignment::Left)
                    .title_top(Line::from(message_ack).right_aligned().gray())
                    .title_bottom(Line::from(mesg_parts.2.clone()).right_aligned())
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(ratatui::style::Color::LightGreen));
                    
            let message = Paragraph::new(mesg_text)
                    .wrap(Wrap{trim:true})
                    .alignment(ratatui::layout::Alignment::Left)
                    .block(message_block);

            let msg_area = Rect::new(
                msg_block_x, 
                msg_block_y, 
                msg_width, 
                msg_height
            );

            msg_block_y += msg_height+2;

            chats_scroll_view.render_widget(message, msg_area);
        }
        //User Joined/Left  Message Box
        else if username!=pubchatcomps.username && username!="maclincomms".to_owned() && is_join_leave_msg==true{

            let msg_height = get_mesg_height(mesg_text.clone(), others_chat_chunk.width);

            let (msg_width, _) = get_msg_width(false, mesg_line.clone(), others_chat_chunk.width, others_chat_chunk.x);

            let x_to_subtract = ((join_msg_chunk.x+msg_width)-user_chat_chunk.x)/2;

            let message_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(ratatui::style::Color::LightCyan));
                    
            let message = Paragraph::new(mesg_text.light_cyan())
                    .wrap(Wrap{trim:true})
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(message_block);

            let msg_area = Rect::new(
                join_msg_chunk.x - x_to_subtract, 
                msg_block_y, 
                msg_width, 
                msg_height
            );

            msg_block_y += msg_height+2;

            chats_scroll_view.render_widget(message, msg_area);
            
        }
        //OTHERS CHAT BUBBLE
        else {
            
            let msg_height = get_mesg_height(mesg_text.clone(), others_chat_chunk.width);

            let (msg_width, msg_block_x) = get_msg_width(false, mesg_line.clone(), others_chat_chunk.width, others_chat_chunk.x);

            let message_block = Block::default()
                    .title(username)
                    .title_alignment(Alignment::Left)
                    .title_bottom(Line::from(mesg_parts.2.clone()).right_aligned())
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(ratatui::style::Color::LightBlue));
                    
            let message = Paragraph::new(mesg_text)
                    .wrap(Wrap{trim:true})
                    .alignment(ratatui::layout::Alignment::Left)
                    .block(message_block);

            let msg_area = Rect::new(
                msg_block_x, 
                msg_block_y, 
                msg_width, 
                msg_height
            );

            msg_block_y += msg_height+2;

            chats_scroll_view.render_widget(message, msg_area);
        }
     
    }
        
    
    frame.render_widget(chatpanel_block, area);


    frame.render_widget(&pubchatcomps.input_ta, input_chunk);

    frame.render_stateful_widget(chats_scroll_view, chat_chunk, &mut pubchatcomps.scroll_state);

}



pub fn get_total_scroll_height(messsages: Vec<Text>, chat_chunk: Rect) -> u16 {

    let [others_chat_chunk, _, user_chat_chunk] = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Fill(1),
                        Constraint::Percentage(10), //Spacing
                        Constraint::Fill(1)
                    ]
                ).areas(chat_chunk); 

    let max_width = user_chat_chunk.width - 6;

    let mut msg_count = 0;
    let mut line_count = 0;
    for text in messsages {
        for line in text {
            line_count+=1;
            // Split the line into words
            let words: Vec<&str> = line
            .spans
            .iter()
            .flat_map(|span| span.content.split_whitespace())
            .collect();

            let mut rem_space = max_width;

            for word in words {

                let word_len = word.len() as u16;
    
                if word_len > max_width && rem_space==max_width {
                    line_count += 1;
                    rem_space = max_width - (word_len - max_width);
                }
                if word_len > max_width && rem_space!=max_width {
                    line_count += 1;
                    rem_space = max_width - ((word_len + 1) - rem_space); // -1 for space consideration
                }
                if (word_len+1) > rem_space && rem_space!=max_width {
                    line_count += 1;
                    rem_space = max_width - word_len;
                }
                if (word_len+1) <= rem_space && rem_space!=max_width {
                    rem_space = rem_space - (word_len + 1);
                }
                if word_len <= rem_space && rem_space==max_width {
                    rem_space = rem_space - word_len;
                }
            }
        }
        msg_count += 1;
    }

    let scroll_height = line_count + (4*msg_count); // 2 for block lines and 2 for gap between each

    return scroll_height; // returns total height
}



// MESSAGE BLOCK HEIGHT

pub fn get_mesg_height(text: Text, width: u16) -> u16 {

    let mut line_count = 0;

    let max_width = width - 2; // 2 for box lines

 

    for line in text {
        line_count+=1;
        // Split the line into words
        let words: Vec<&str> = line
        .spans
        .iter()
        .flat_map(|span| span.content.split_whitespace())
        .collect();

        let mut rem_space = max_width;
    
        for word in words {

            let word_len = word.len() as u16;

            if word_len > max_width && rem_space==max_width {
                line_count += 1;
                rem_space = max_width - (word_len - max_width);
            }
            if word_len > max_width && rem_space!=max_width {
                line_count += 1;
                rem_space = max_width - ((word_len + 1) - rem_space); // -1 for space consideration
            }
            if (word_len+1) > rem_space && rem_space!=max_width {
                line_count += 1;
                rem_space = max_width - word_len;
            }
            if (word_len+1) <= rem_space && rem_space!=max_width {
                rem_space = rem_space - (word_len + 1);
            }
            if word_len <= rem_space && rem_space==max_width {
                rem_space = rem_space - word_len;
            }
        }
    }
    
    let mesg_height = line_count + 2; // for box lines

    return mesg_height; // returns the height for message block
}

//MESSAGE BLOCK WIDTH

pub fn get_msg_width(is_user_side: bool, line: Line, width: u16, x: u16) -> (u16, u16) {

    // Calculate total number of characters including spaces
    let total_chars: u16 = line
    .spans.iter()
    .map(|span| span.content.chars().count() as u16)
    .sum();

    if is_user_side {  //MY MESSAGE BLOCK WIDTH

        if total_chars > (width - 2){
            return (width, x)
        }
        else if total_chars > 15 {
            let new_x = x + (width-total_chars) - 2;
            return (total_chars + 2, new_x);
        }
        else {
            let new_x = x + (width-15) - 2;
            return (17, new_x);
        }
    }

    else {  // OTHERS MESSAGE BLOCK WIDTH

        if total_chars > (width - 2){
            return (width, x)
        }
        else if total_chars > 15 {
            return (total_chars + 2, x);
        }
        else {
            return (17, x);
        }
    }

    
}





//PUBLIC CHAT COMPONENETS

#[derive (Debug, Clone)]
pub struct PublicChatComponents {
    pub input_ta: TextArea<'static>,
    pub scroll_state: ScrollViewState,
    pub chat_history: Arc<Mutex<Vec<(String, Text<'static>, String, bool, String)>>>,  // Vector of tuple of (username, his message, time, joined_left, ack(>))
    pub username: String
}

impl PublicChatComponents {

    pub fn new() -> Self {
        Self {
            input_ta: Self::get_input_textarea(),
            scroll_state: ScrollViewState::default(),
            chat_history: Arc::new(Mutex::new(Vec::new())),
            username: "".to_string()
        }
    }
    
    pub fn get_input_textarea() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_line_style(Style::default());
        ta.set_placeholder_text("Chat with the world");
        ta.set_style(Style::default().fg(Color::White));
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Green)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Press Enter to send"),
        );

        ta
    }
}
