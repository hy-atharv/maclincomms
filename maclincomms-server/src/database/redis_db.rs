use std::collections::HashMap;

use actix_web::web::Json;
use redis::PushKind;
use redis::{AsyncCommands, aio::MultiplexedConnection};
use serde_json::json;
use tokio::sync::mpsc::{self, Sender};
use crate::models::notification_data::NotificationType;
use crate::secret_store::get_secret;
use crate::models::{
    dms_data::{Dms_Table, Blocked_List},
    notification_data::NotificationData
};




 pub async fn publish_notification(notification: NotificationData) -> redis::RedisResult<i32> {
    
    let redis_url = match get_secret("REDIS_URL"){
        Some(url) => url,
        None => "".to_owned()
    };

    let client = redis::Client::open(redis_url)?;
    let mut con: MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    // Convert to JSON string
    let json_payload = serde_json::to_string(&notification).unwrap();
    println!("{:?}", json_payload);
    // Channel to publish
    let channel = format!("NOTIFICATIONS<{},{}>", notification.from, notification.to);

    // Publish
    let subscribers = con.publish(channel, json_payload).await?;
    println!("{:?}", subscribers);

    Ok(subscribers)
    
}



pub async fn queue_notification(notification: NotificationData) -> redis::RedisResult<()> {
    
    let redis_url = match get_secret("REDIS_URL"){
        Some(url) => url,
        None => "".to_owned()
    };

    let client = redis::Client::open(redis_url)?;
    let mut con: MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    // Convert to JSON string
    let json_payload = serde_json::to_string(&notification).unwrap();

    let list_key = format!("NOTIFICATIONS<{},{}>", notification.from, notification.to);

    // Push the notification to the list
    con.rpush(&list_key, json_payload).await?;
    
    // Set a TTL of 24 hours (86400 seconds) for the list key
    con.expire(&list_key, 86400).await?;

    Ok(())
    
}



pub async fn subscribe_to_notifications(username: &str, sender: Sender<NotificationData>) -> redis::RedisResult<()> {
    let redis_url = match get_secret("REDIS_URL") {
        Some(url) => url,
        None => "".to_owned()
    };

    let client = redis::Client::open(redis_url)?;
    
    //Creating redis pubsub channel
    let(redis_sender, mut redis_receiver) = mpsc::unbounded_channel();

    let config = redis::AsyncConnectionConfig::new().set_push_sender(redis_sender);
    let mut con = client.get_multiplexed_async_connection_with_config(&config).await?; 

    // Create the pattern for channel subscription
    let channel_pattern = format!("NOTIFICATIONS<*,{}>", username);     
    
    // Subscribe to the pattern
    con.psubscribe(channel_pattern).await?;
    
    while let Some(push) = redis_receiver.recv().await {
        match push.kind {
            PushKind::PMessage => {
                if let Some(payload) = push.data.get(2) {
                    if let redis::Value::BulkString(bytes) = payload {
                        if let Ok(raw) = String::from_utf8(bytes.clone()) {
                            // Remove outer quotes caused by double-stringing
                            
                
                            // Now deserialize into your struct
                            match serde_json::from_str::<NotificationData>(&raw) {
                                Ok(notification) => {
                                    let _ = sender.send(notification).await;
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse notification: {}", e);
                                }
                            }
                        }
                    }
                }
            }
            PushKind::PSubscribe => {
                println!("✅ Subscribed to notification channels");
            }
            _ => {
                println!("⚠️ Unexpected push kind: {:?}", push.kind);
            }
        }
    }
    
    
    
    Ok(())
}


pub async fn retrieve_queued_notifications(username: &str) -> redis::RedisResult<HashMap<String, Vec<String>>> {
    
    let redis_url = match get_secret("REDIS_URL"){
        Some(url) => url,
        None => "".to_owned()
    };

    let client = redis::Client::open(redis_url)?;
    let mut con: MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    //Pattern
    let pattern = format!("NOTIFICATIONS<*,{}>", username);
    //Getting all notifications lists for the user
    let keys: Vec<String> = con.keys(pattern).await?;

    let mut result_map = HashMap::new();

    for key in keys.iter() {
        let items: Vec<String> = con.lrange(key, 0, -1).await?;
        //Storing all items from list to map
        result_map.insert(key.clone(), items);

        //Deleting key after retrieving all notifications from it
        let _ : () = con.del(key).await?;
    }

    Ok(result_map)
    
}


//Keeping Alive Redis DB in upstash during long inactivity 
pub async fn keep_alive_upstash() -> redis::RedisResult<i32> {
    
    let redis_url = match get_secret("REDIS_URL"){
        Some(url) => url,
        None => "".to_owned()
    };

    let client = redis::Client::open(redis_url)?;
    let mut con: MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    
    // Channel to publish
    let channel = format!("KEEP_ALIVE");

    // Publish Dummy Mesg
    let subscribers = con.publish(channel, "KEEPING_ALIVE").await?;

    Ok(subscribers)
    
}