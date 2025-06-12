use actix_web::web::Json;
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Client};
use serde_json::json;
use crate::secret_store::get_secret;
use crate::models::room_data::Rooms_Table;


pub async fn get_room_data(roomname: &str) -> Result< Vec<Rooms_Table>, reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "ROOMS",
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
        .query(&[("room_name", format!("eq.{roomname}"))])
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    let data = res.json::<Vec<Rooms_Table>>().await?;
    println!("{:#?}", data);
    Ok(data)
}




pub async fn insert_room_data(room: Json<Rooms_Table>) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "ROOMS",
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
        .json(&room)
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    Ok(())
}



pub async fn insert_member_to_room(username: &str, roomname: &str) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "rpc/append_members_array",
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
        "new_member": username,
        "roomname": roomname
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


pub async fn remove_member_from_room(username: &str, roomname: &str) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "rpc/remove_members_array",
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
        "member_to_remove": username,
        "roomname": roomname
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


pub async fn delete_room_data(roomname: &str) -> Result< (), reqwest::Error > {

    
    let url = match get_secret("SUPABASE_URL"){
        Some(url) => url + "ROOMS",
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
        .delete(url)
        .query(&[("room_name", format!("eq.{roomname}"))])
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", res);     

    Ok(())
}


