
use actix_web::web::Json;
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Client};

use std::env;

use crate::models::user_auth::User_Auth_Table;

pub async fn get_auth_data(username: &str) -> Result< Vec<User_Auth_Table>, reqwest::Error > {

    
    dotenvy::dotenv();

    let url = match env::var("SUPABASE_URL"){
        Ok(url) => url,
        Err(err) => "".to_owned()
    };

    let api_key = match env::var("SUPABASE_API_KEY"){
        Ok(key) => key,
        Err(err) => "".to_owned()
    };
    

    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
   
    let client = Client::new();
    let res = client
        .get(url)
        .query(&[("username", format!("eq.{username}").as_str())])
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    let data = res.json::<Vec<User_Auth_Table>>().await?;
    println!("{:#?}", data);
    Ok(data)
}



pub async fn insert_auth_data(user: Json<User_Auth_Table>) -> Result< (), reqwest::Error > {

    dotenvy::dotenv();

    let url = match env::var("SUPABASE_URL"){
        Ok(url) => url,
        Err(err) => "".to_owned()
    };

    let api_key = match env::var("SUPABASE_API_KEY"){
        Ok(key) => key,
        Err(err) => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

   
    let client = Client::new();
    let res = client
        .post(url)
        .json(&user)
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    Ok(())
}