use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use reqwest::Client;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::{client::{self, IntoClientRequest}, http::Request, ClientRequestBuilder, Message}, WebSocketStream};

use crate::user_model::{RegisterResponse, StatusTypes, User};



pub async fn login() -> (String, String){

    let mut username = String::new();

    println!("\x1b[93mEnter your Username: \x1b[0m");
    
    std::io::stdin().read_line(&mut username).unwrap();

    // Trim whitespace (including newline characters) from the input
    let username = username.trim();

    let user = get_user(&username).await;

    return (username.to_owned(),user);

}


pub async fn get_user(username: &str) -> String {

    println!("\x1b[93m{username}, enter your password: \x1b[0m");

    let mut password = rpassword::read_password().unwrap();
    password = password.trim().to_owned();

    println!("\x1b[93m{username}, confirm your password: \x1b[0m");

    let mut c_password = rpassword::read_password().unwrap();
    c_password = c_password.trim().to_owned();

    let pass_match = password==c_password;

    match pass_match{
        true => {
            println!("\x1b[93m\nLogging in user {} ...\nRemember your username & password!\n\x1b[0m", username);
            let register_status = login_into_backend(username, password).await;
            match register_status {
                Ok(status) => {
                    match status.status_type {
                        StatusTypes::LOG_IN_SUCCESSFUL => {
                            let token = status.access_token;
                            let refresh_token = status.refresh_token;
                            println!("\x1b[93m\nAlright {}, let's get you connected to the World\n\x1b[0m", username);
                            println!("\x1b[32mConnecting to MacLin Comms Server...\x1b[0m");
                            return token;

                        },
                        StatusTypes::INVALID_CREDENTIALS => {
                            println!("\x1b[91m\n{username}, you entered wrong password!\n\n\x1b[0m");
                            return "WRONG".to_owned();
                        }
                        _ => {
                            println!("\x1b[91m\nSome unexpected error occured, Pls try again later or register a new account..\x1b[0m");
                            return "NO_TOKEN_GRANTED".to_owned();
                        }
                    }
                },
                Err(err) => {
                    println!("\x1b[91m\nSome error occured while making request to server, Pls try again later..\x1b[0m");
                    return "NO_TOKEN_GRANTED".to_owned();
                }
            }
        }
        false => {
            println!("\x1b[91m\n{username}, passwords dont match, give it a retry!\n\n\x1b[0m");
            return "PASSWORD_MISMATCH".to_owned();
        }   
    }
}


pub async fn login_into_backend(username:&str, pass:String) -> Result< RegisterResponse, reqwest::Error > {
    
    let url = "https://maclincomms-server-v1-lli0.shuttle.app/login_user".to_owned();

    let user = User{
        username: username.to_owned(),
        password: pass
    };

    let client = Client::new();
    let res = client
        .post(url)
        .json(&user)
        .send()
        .await?;    

    let data = res.json::<RegisterResponse>().await?;
    Ok(data)
}