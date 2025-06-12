use std::collections::HashMap;

use actix_web::web::Json;
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Client};
use serde_json::{json, Value};
use crate::{models::dms_data::{ChatData, DmUser_Data, Dms_List}, secret_store::get_secret};
use crate::models::dms_data::{Dms_Table, Blocked_List};



pub async fn insert_user_to_dms_table(username: &str) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "DMS",
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());


    let body = json!({
        "username": username,
        "dms_list": [],
        "blocked_list": [],
        "chat_history": []
    });

   
    let client = Client::new();
    let res = client
        .post(url)
        .json(&body)
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    Ok(())
}




 pub async fn get_blocked_list(username: &str) -> Result< Vec<Blocked_List>, reqwest::Error > {

    
    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "DMS",
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
        .query(&[
            ("select", "blocked_list"),
            ("username", &format!("eq.{username}"))
        ])
        .headers(headers)
        .send()
        .await?;
    
    println!("{:#?}", res);     

    let data = res.json::<Vec<Blocked_List>>().await?;
    println!("{:#?}", data);
    Ok(data)
}



pub async fn insert_user_to_blocked_list(username: &str, user_to_block: &str) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "rpc/append_blocked_list",
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());


    let body = json!({
        "user_name": username,
        "user_to_block": user_to_block
    });

   
    let client = Client::new();
    let res = client
        .post(url)
        .json(&body)
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    Ok(())
}



pub async fn remove_user_from_blocked_list(username: &str, user_to_unblock: &str) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "rpc/remove_blocked_list",
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

 
    let body = json!({
        "user_name": username,
        "user_to_unblock": user_to_unblock
    });

   
    let client = Client::new();
    let res = client
        .post(url)
        .json(&body)
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    Ok(())
}



pub async fn insert_user_to_dms_list(username: &str, user_to_add: &str) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "rpc/append_dms_list",
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());


    let body = json!({
        "user_name": username,
        "user_to_add": user_to_add
    });

   
    let client = Client::new();
    let res = client
        .post(url)
        .json(&body)
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    Ok(())
}


pub async fn get_dms_list(user1: &str, user2: &str) -> Result< Vec<Dms_List>, reqwest::Error > {

    
    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "DMS",
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
        .query(&[
            ("select", "username,dms_list"),
            ("username", &format!("in.(\"{}\",\"{}\")", user1, user2)),
        ])
        .headers(headers)
        .send()
        .await?;
    
    println!("{:#?}", res);     

    let data = res.json::<Vec<Dms_List>>().await?;
    println!("{:#?}", data);
    Ok(data)
}


pub async fn get_dms_list_data(username: &str) -> Result<Vec<DmUser_Data>, reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "rpc/get_dms_list",
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

  
    let body = json!({
        "user_name": username,
    });

   
    let client = Client::new();
    let res = client
        .post(url)
        .json(&body)
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    let data = res.json::<Vec<DmUser_Data>>().await?;
    println!("{:?}", data);
    Ok(data)
}


pub async fn get_dm_chats_backup_data(username: &str) -> Result<ChatData, reqwest::Error >  {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "rpc/get_dm_chats_backup",
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

    // Use Supabase `array_append` function to add the username to the `room_members` column
    let body = json!({
        "user_name": username,
    });

   
    let client = Client::new();
    let res = client
        .post(url)
        .json(&body)
        .headers(headers)
        .send()
        .await?;
    println!("{:?}", res);

    let data = res.json::<ChatData>().await?;
    println!("{:?}", data);
    Ok(data)  
}

pub async fn upload_dm_chats_backup_data(username: &str, chat_history: ChatData,) -> Result< (), reqwest::Error >  {

    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "DMS",
        None => "".to_owned()
    };
    let api_key = match get_secret("SUPABASE_API_KEY"){
        Some(key) => key,
        None => "".to_owned()
    };


    let mut headers = HeaderMap::new();

    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());


    if let Ok(chat_json) = serde_json::to_value(chat_history){

        let body = json!({
            "chat_history": chat_json
        });

        let client = Client::new();
        let res = client
            .patch(url)
            .query(&[("username", format!("eq.{username}"))])
            .json(&body)
            .headers(headers)
            .send()
            .await?;
        println!("{:#?}", res);
    }

    Ok(())  
}