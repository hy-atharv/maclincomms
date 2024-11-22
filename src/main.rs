use std::process::exit;

use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use register_user::get_user;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::{client::{self, IntoClientRequest}, http::Request, ClientRequestBuilder, Message}, WebSocketStream};


mod send_mesg;
mod receive_mesg;
mod register_user;
mod user_model;
mod login_user;


#[tokio::main]
async fn main() {

    print!("\x1B[2J\x1b[1;1H");
    println!("\x1b[93m\n\nWelcome to MacLin Comms!\n- Chat with your fellow MacOS & Linux users in Terminal\x1b[0m\n\n\x1b[36mDeveloped by Atharv Kumar Tiwari\n\n\x1b[0m");

    let (mut username,mut user_token) = ("".to_owned(), "".to_owned());

    loop {
        println!("1 -> \x1b[32mRegister Now\x1b[0m");
        println!("2 -> \x1b[32mAlready a User? Log In\x1b[0m");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                // register the user
               (username, user_token) = register_user::register().await;
               if user_token!="NO_TOKEN_GRANTED".to_owned() &&
                  user_token!="PASSWORD_MISMATCH".to_owned() &&
                  user_token!="EXISTING_USER".to_owned(){
                    break;
               }
               if user_token=="NO_TOKEN_GRANTED".to_owned(){
                println!("\x1b[93m\n\nExiting MacLin Comms\x1b[0m");
                exit(1);
               }
        
               if user_token=="PASSWORD_MISMATCH".to_owned() || 
                  user_token=="EXISTING_USER".to_owned() {
        
                (username,user_token) = register_user::register().await;

                    if user_token!="NO_TOKEN_GRANTED".to_owned() &&
                        user_token!="PASSWORD_MISMATCH".to_owned() &&
                        user_token!="EXISTING_USER".to_owned(){

                        break;
                    }
               }

            },
            "2" => {
                // log in
                (username, user_token) = login_user::login().await;
                if user_token!="NO_TOKEN_GRANTED".to_owned() &&
                  user_token!="PASSWORD_MISMATCH".to_owned() &&
                  user_token!="WRONG".to_owned(){
                    break;
               }
               if user_token=="NO_TOKEN_GRANTED".to_owned(){
                println!("\x1b[93m\n\nExiting MacLin Comms\x1b[0m");
                exit(1);
               }
        
               if user_token=="PASSWORD_MISMATCH".to_owned() || 
                  user_token=="WRONG".to_owned() {
        
                (username,user_token) = login_user::login().await;


                    if user_token!="NO_TOKEN_GRANTED".to_owned() &&
                        user_token!="PASSWORD_MISMATCH".to_owned() &&
                        user_token!="WRONG".to_owned(){

                        break;
                    }
               }
            },
            _ => {
                continue;
            }
        }

    }


    
    let url = "wss://maclincomms-server-v1-lli0.shuttle.app/world_chat".parse().unwrap();


    let mut request = ClientRequestBuilder::new(url)
        .with_header("Authorization", user_token);

        let ws_stream = match connect_async(request).await {
            Ok((ws_stream, _)) => {
                println!("\x1b[32m\nConnected to MacLin Comms Public Chat\nPls don't share any sensitive information with strangers.\n\n\x1b[0m");
                ws_stream
            }
            Err(err) => {
                println!("\x1b[91m\nFailed to Connect to MacLin Comms:\n{:#?}\n\x1b[0m", err);
                return; // Exit early if connection fails
            }
        };


    let (mut write, mut read) = ws_stream.split();

    let user_joined = format!("\x1b[93m\nUser {} joined!\n\x1b[0m", username.clone());

    write.send(Message::Text(user_joined)).await.expect("Failed to send message");

    // Handle incoming messages in a separate task
    let read_handle = tokio::spawn(receive_mesg::handle_incoming_messages(read));

    // Read from command line and send messages
    let write_handle = tokio::spawn(send_mesg::read_and_send_messages(write, username));

    // Await both tasks (optional)
    let _ = tokio::try_join!(read_handle, write_handle);

}