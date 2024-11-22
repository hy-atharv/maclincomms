use std::process::exit;

use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::{self, client::{self, IntoClientRequest}, http::Request, ClientRequestBuilder, Message}, WebSocketStream};




pub async fn read_and_send_messages(mut write: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>, user: String) {
    let mut reader = io::BufReader::new(io::stdin()).lines();
    while let Some(line) = reader.next_line().await.expect("Failed to read line") {
        if !line.trim().is_empty() && line.trim()!="exit".to_owned() {

            // Format the message with the username and message content
            let formatted_message = format!(
                "\x1b[36m{} >>>\x1b[0m \x1b[32m{} \x1b[0m", 
                user, line 
            );
            write.send(Message::Text(formatted_message)).await.expect("Failed to send message");
        }

        
//You Say Windows?! Bye Bye
        if line.trim().to_lowercase()=="windows".to_owned(){
            
            let maclin_msg = format!(
                "\x1b[36mMacLin Comms >>>\x1b[0m \x1b[32m{}, Windows?! BYE! \x1b[0m", 
                user 
            );

            let user_left_msg = format!("\x1b[93m\nUser {} left!\n\x1b[0m", user);


            println!("{}",maclin_msg);
            write.send(Message::Text(maclin_msg)).await.expect("Failed to send message");

            write.send(Message::Text(user_left_msg)).await.expect("Failed to send message");

            
            //closing connection
            let close_msg = Message::Close(Some(tungstenite::protocol::CloseFrame {
                code: tungstenite::protocol::frame::coding::CloseCode::Normal,
                reason: "User requested disconnection".into(),
            }));

            

            write.send(close_msg).await.expect("Failed to send close connection message.");

            
            println!("\x1b[93m\n\nExiting MacLin Comms...\x1b[0m");
            exit(1);
        }

    // Exit Voluntarily
        if line.trim()=="exit".to_owned(){

            let user_left_msg = format!("\x1b[93m\nUser {} left!\n\x1b[0m", user);

            write.send(Message::Text(user_left_msg)).await.expect("Failed to send message");

            //closing connection
            let close_msg = Message::Close(Some(tungstenite::protocol::CloseFrame {
                code: tungstenite::protocol::frame::coding::CloseCode::Normal,
                reason: "User requested disconnection".into(),
            }));

            

            write.send(close_msg).await.expect("Failed to send close connection message.");

            println!("\x1b[36mMacLin Comms >>>\x1b[0m \x1b[32mSee you soon {}!\x1b[0m", user);
            println!("\x1b[93m\n\nExiting MacLin Comms...\x1b[0m");
            exit(1);
        }
    }
}