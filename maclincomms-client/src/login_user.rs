use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use reqwest::{Client, StatusCode};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::{client::{self, IntoClientRequest}, http::Request, ClientRequestBuilder, Message}, WebSocketStream};

use crate::{endpoints::Endpoints, user_model::{LoginResponse, RegisterResponse, StatusTypes, UserLoginPayload}};


#[derive(Debug)]
pub enum LoginResponseResult{
    TOKEN(String),
    INVALID_USER,
    USER_NOT_FOUND,
    DATABASE_ERROR,
    UNKNOWN_ERROR,
    REQUEST_ERROR
}


pub async fn login(username: String, password: String) -> (String, LoginResponseResult, String, i64){

    // Trim whitespace (including newline characters) from the input
    let username = username.trim().to_owned();
    let password = password.trim().to_owned();

    let (login_response_result, refresh_token, expiry) = get_user(username.clone(), password).await;

    return (username, login_response_result, refresh_token, expiry);

}


pub async fn get_user(username: String, password: String) -> (LoginResponseResult, String, i64) {

    let login_status = login_into_backend(username, password).await;

    match login_status {
        Ok(status) => {
            match status.status_type {
                    StatusTypes::LOG_IN_SUCCESSFUL => {

                            let token = status.access_token;
                            let refresh_token = status.refresh_token;
                            let expiry = status.exp;

                            return (LoginResponseResult::TOKEN(token), refresh_token, expiry) ;

                    },
                    StatusTypes::INVALID_CREDENTIALS => {

                            return (LoginResponseResult::INVALID_USER, "".to_string(), 0);

                    },
                    StatusTypes::USER_NOT_FOUND => {

                        return (LoginResponseResult::USER_NOT_FOUND, "".to_string(), 0);

                    },
                    StatusTypes::DATABASE_ERROR => {

                        return (LoginResponseResult::DATABASE_ERROR, "".to_string(), 0);

                    },
                    _ => {
                            
                        return (LoginResponseResult::UNKNOWN_ERROR, "".to_string(), 0);
                    }
            }
        },
        Err(err) => {
          return (LoginResponseResult::REQUEST_ERROR, "".to_string(), 0);
        }
    }
       
}


pub async fn login_into_backend(user_name: String, pass:String) -> Result< LoginResponse, reqwest::Error > {
    
    let endpoints = Endpoints::new();
    
    let url = endpoints.login.to_owned();

    let user = UserLoginPayload{
        username: user_name,
        password: pass
    };

    let client = Client::new();
    let res = client
        .post(url)
        .json(&user)
        .send()
        .await?;    

    let data = res.json::<LoginResponse>().await?;
    Ok(data)
}