
use actix_web::web::Json;
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Client};

use crate::secret_store::get_secret;

use crate::models::user_auth::User_Auth_Table;

pub async fn get_auth_data(username: &str) -> Result< Vec<User_Auth_Table>, reqwest::Error > {

    
    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url,
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
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

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url,
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
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